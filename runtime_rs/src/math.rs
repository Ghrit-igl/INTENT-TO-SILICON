use std::rc::Rc;
use std::cell::RefCell;

/// High-Performance N-Dimensional Tensor Engine for AAYU Silicon VM
/// Supports Cache-Blocked GEMM (Tile Matrix Multiply), Element-wise Broadcasting,
/// Modern LLM Activations (GELU, SiLU, Softmax, Sigmoid, ReLU), LayerNorm,
/// Scaled Dot-Product Attention, and Cosine Similarity.
#[derive(Debug, Clone)]
pub struct Tensor {
    pub shape: Vec<usize>,
    pub strides: Vec<usize>,
    pub offset: usize,
    pub data: Rc<RefCell<Vec<f64>>>,
}

impl Tensor {
    /// Construct a new tensor from shape and flat data
    pub fn new(shape: Vec<usize>, data: Vec<f64>) -> Self {
        let expected_numel: usize = shape.iter().product();
        assert_eq!(
            expected_numel,
            data.len(),
            "Data length ({}) must match product of shape dimensions ({})",
            data.len(),
            expected_numel
        );
        let strides = Self::compute_strides(&shape);
        Self {
            shape,
            strides,
            offset: 0,
            data: Rc::new(RefCell::new(data)),
        }
    }

    /// Compute row-major contiguous strides
    pub fn compute_strides(shape: &[usize]) -> Vec<usize> {
        let mut strides = vec![0; shape.len()];
        let mut current = 1;
        for i in (0..shape.len()).rev() {
            strides[i] = current;
            current *= shape[i];
        }
        strides
    }

    /// Create tensor filled with zeros
    pub fn zeros(shape: Vec<usize>) -> Self {
        let numel: usize = shape.iter().product();
        Self::new(shape, vec![0.0; numel])
    }

    /// Create tensor filled with ones
    pub fn ones(shape: Vec<usize>) -> Self {
        let numel: usize = shape.iter().product();
        Self::new(shape, vec![1.0; numel])
    }

    /// Create tensor filled with a specific value
    pub fn full(shape: Vec<usize>, val: f64) -> Self {
        let numel: usize = shape.iter().product();
        Self::new(shape, vec![val; numel])
    }

    /// Create tensor with normal distribution N(0, 1) using Box-Muller transform
    pub fn randn(shape: Vec<usize>, mut seed: u64) -> Self {
        let numel: usize = shape.iter().product();
        let mut data = Vec::with_capacity(numel);

        // Simple high-speed XorShift64 PRNG
        let next_u64 = move |s: &mut u64| -> u64 {
            let mut x = *s;
            if x == 0 { x = 0x5432109876543210; }
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            *s = x;
            x
        };

        let mut i = 0;
        while i < numel {
            // Uniform (0, 1]
            let u1 = ((next_u64(&mut seed) >> 11) as f64 + 1.0) / 9007199254740993.0;
            let u2 = ((next_u64(&mut seed) >> 11) as f64 + 1.0) / 9007199254740993.0;

            let r = (-2.0 * u1.ln()).sqrt();
            let theta = 2.0 * std::f64::consts::PI * u2;
            let z0 = r * theta.cos();
            let z1 = r * theta.sin();

            data.push(z0);
            i += 1;
            if i < numel {
                data.push(z1);
                i += 1;
            }
        }

        Self::new(shape, data)
    }

    /// Total number of elements in tensor
    #[inline(always)]
    pub fn numel(&self) -> usize {
        self.shape.iter().product()
    }

    /// Check if memory layout is contiguous
    pub fn is_contiguous(&self) -> bool {
        self.strides == Self::compute_strides(&self.shape) && self.offset == 0
    }

    /// Convert to contiguous flat vector of elements
    pub fn to_vec(&self) -> Vec<f64> {
        let n = self.numel();
        let mut out = Vec::with_capacity(n);
        let ndim = self.shape.len();
        if ndim == 0 {
            return out;
        }

        let mut idx = vec![0; ndim];
        for _ in 0..n {
            out.push(self.get_element(&idx));
            for dim in (0..ndim).rev() {
                idx[dim] += 1;
                if idx[dim] < self.shape[dim] {
                    break;
                }
                idx[dim] = 0;
            }
        }
        out
    }

    /// Convert tensor to contiguous representation
    pub fn to_contiguous(&self) -> Self {
        if self.is_contiguous() {
            self.clone()
        } else {
            Self::new(self.shape.clone(), self.to_vec())
        }
    }

    /// Get element by multi-dimensional coordinates
    pub fn get_element(&self, indices: &[usize]) -> f64 {
        assert_eq!(indices.len(), self.shape.len(), "Indices length must match shape length");
        let mut idx = self.offset;
        for (i, &index) in indices.iter().enumerate() {
            assert!(index < self.shape[i], "Index out of bounds");
            idx += index * self.strides[i];
        }
        self.data.borrow()[idx]
    }

    /// Set element by multi-dimensional coordinates
    pub fn set_element(&self, indices: &[usize], value: f64) {
        assert_eq!(indices.len(), self.shape.len(), "Indices length must match shape length");
        let mut idx = self.offset;
        for (i, &index) in indices.iter().enumerate() {
            assert!(index < self.shape[i], "Index out of bounds");
            idx += index * self.strides[i];
        }
        self.data.borrow_mut()[idx] = value;
    }

    /// Zero-copy slice view
    pub fn slice_view(&self, starts: &[usize], ends: &[usize]) -> Self {
        assert_eq!(starts.len(), self.shape.len());
        assert_eq!(ends.len(), self.shape.len());

        let mut new_shape = Vec::new();
        let mut new_offset = self.offset;

        for i in 0..self.shape.len() {
            assert!(starts[i] <= ends[i] && ends[i] <= self.shape[i], "Invalid slice bounds");
            new_shape.push(ends[i] - starts[i]);
            new_offset += starts[i] * self.strides[i];
        }

        Self {
            shape: new_shape,
            strides: self.strides.clone(),
            offset: new_offset,
            data: Rc::clone(&self.data),
        }
    }

    /// Zero-copy transpose of last two dimensions
    pub fn transpose(&self) -> Self {
        assert!(self.shape.len() >= 2, "Cannot transpose tensor with less than 2 dimensions");
        let mut new_shape = self.shape.clone();
        let mut new_strides = self.strides.clone();

        let len = new_shape.len();
        new_shape.swap(len - 1, len - 2);
        new_strides.swap(len - 1, len - 2);

        Self {
            shape: new_shape,
            strides: new_strides,
            offset: self.offset,
            data: Rc::clone(&self.data),
        }
    }

    /// Reshape tensor into new shape with identical element count
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Self, String> {
        let current_numel = self.numel();
        let target_numel: usize = new_shape.iter().product();
        if current_numel != target_numel {
            return Err(format!(
                "Cannot reshape tensor with {} elements into shape {:?} (needs {} elements)",
                current_numel, new_shape, target_numel
            ));
        }

        let contiguous = self.to_contiguous();
        Ok(Self::new(new_shape, contiguous.to_vec()))
    }

    // =========================================================================
    // HIGH-SPEED CACHE-BLOCKED GEMM (General Matrix Multiply)
    // =========================================================================

    /// Cache-Blocked Matrix Multiplication for 2D tensors: C = A x B
    /// Uses 32x32 tile blocking to maximize L1/L2 data cache hit rate.
    pub fn matmul_2d(a: &Tensor, b: &Tensor) -> Result<Tensor, String> {
        if a.shape.len() != 2 || b.shape.len() != 2 {
            return Err(format!("matmul_2d requires 2D tensors, got shapes {:?} and {:?}", a.shape, b.shape));
        }
        let m = a.shape[0];
        let k_a = a.shape[1];
        let k_b = b.shape[0];
        let n = b.shape[1];

        if k_a != k_b {
            return Err(format!("Incompatible matrix dimensions: [{}, {}] x [{}, {}]", m, k_a, k_b, n));
        }

        let a_cont = a.to_contiguous();
        let b_cont = b.to_contiguous();
        let a_data = a_cont.data.borrow();
        let b_data = b_cont.data.borrow();

        let mut c_data = vec![0.0; m * n];
        const TILE: usize = 32;

        // Tiled 3-loop GEMM with register accumulation
        for ii in (0..m).step_by(TILE) {
            let i_end = (ii + TILE).min(m);
            for kk in (0..k_a).step_by(TILE) {
                let k_end = (kk + TILE).min(k_a);
                for jj in (0..n).step_by(TILE) {
                    let j_end = (jj + TILE).min(n);

                    for i in ii..i_end {
                        let a_row_offset = i * k_a;
                        let c_row_offset = i * n;
                        for k in kk..k_end {
                            let a_ik = a_data[a_row_offset + k];
                            let b_row_offset = k * n;
                            // Loop unrolling for 4 elements at once
                            let mut j = jj;
                            while j + 4 <= j_end {
                                c_data[c_row_offset + j] += a_ik * b_data[b_row_offset + j];
                                c_data[c_row_offset + j + 1] += a_ik * b_data[b_row_offset + j + 1];
                                c_data[c_row_offset + j + 2] += a_ik * b_data[b_row_offset + j + 2];
                                c_data[c_row_offset + j + 3] += a_ik * b_data[b_row_offset + j + 3];
                                j += 4;
                            }
                            while j < j_end {
                                c_data[c_row_offset + j] += a_ik * b_data[b_row_offset + j];
                                j += 1;
                            }
                        }
                    }
                }
            }
        }

        Ok(Tensor::new(vec![m, n], c_data))
    }

    /// General Matmul supporting 2D and Batched High-Dim Tensors [..., M, K] x [..., K, N]
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape.len() == 2 && other.shape.len() == 2 {
            return Self::matmul_2d(self, other);
        }

        if self.shape.len() < 2 || other.shape.len() < 2 {
            return Err("MatMul requires tensors with at least 2 dimensions".to_string());
        }

        let a_last2 = &self.shape[self.shape.len() - 2..];
        let b_last2 = &other.shape[other.shape.len() - 2..];
        let m = a_last2[0];
        let k1 = a_last2[1];
        let k2 = b_last2[0];
        let n = b_last2[1];

        if k1 != k2 {
            return Err(format!("Matrix inner dimensions mismatch: {} != {}", k1, k2));
        }

        // For batched matrix multiply (e.g. [B, S, D] x [B, D, S]), flatten batch dims
        let a_batch_size: usize = self.shape[..self.shape.len() - 2].iter().product();
        let b_batch_size: usize = other.shape[..other.shape.len() - 2].iter().product();

        if a_batch_size != b_batch_size && a_batch_size != 1 && b_batch_size != 1 {
            return Err(format!("Batch dimensions incompatible: {} vs {}", a_batch_size, b_batch_size));
        }

        let batch_count = a_batch_size.max(b_batch_size);
        let a_cont = self.to_contiguous();
        let b_cont = other.to_contiguous();
        let a_data = a_cont.data.borrow();
        let b_data = b_cont.data.borrow();

        let a_stride = m * k1;
        let b_stride = k2 * n;
        let c_stride = m * n;
        let mut c_data = vec![0.0; batch_count * c_stride];

        for b_idx in 0..batch_count {
            let a_off = if a_batch_size == 1 { 0 } else { b_idx * a_stride };
            let b_off = if b_batch_size == 1 { 0 } else { b_idx * b_stride };
            let c_off = b_idx * c_stride;

            for i in 0..m {
                for k in 0..k1 {
                    let a_ik = a_data[a_off + i * k1 + k];
                    for j in 0..n {
                        c_data[c_off + i * n + j] += a_ik * b_data[b_off + k * n + j];
                    }
                }
            }
        }

        let mut out_shape = Vec::new();
        if self.shape.len() >= other.shape.len() {
            out_shape.extend_from_slice(&self.shape[..self.shape.len() - 2]);
        } else {
            out_shape.extend_from_slice(&other.shape[..other.shape.len() - 2]);
        }
        out_shape.push(m);
        out_shape.push(n);

        Ok(Tensor::new(out_shape, c_data))
    }

    // =========================================================================
    // BROADCASTING & ELEMENT-WISE ARITHMETIC
    // =========================================================================

    /// Compute broadcasted output shape from two shapes
    pub fn broadcast_shapes(s1: &[usize], s2: &[usize]) -> Result<Vec<usize>, String> {
        let max_len = s1.len().max(s2.len());
        let mut out = vec![0; max_len];

        for i in 0..max_len {
            let d1 = if i < s1.len() { s1[s1.len() - 1 - i] } else { 1 };
            let d2 = if i < s2.len() { s2[s2.len() - 1 - i] } else { 1 };

            if d1 == d2 || d1 == 1 || d2 == 1 {
                out[max_len - 1 - i] = d1.max(d2);
            } else {
                return Err(format!("Shapes {:?} and {:?} are not broadcastable", s1, s2));
            }
        }

        Ok(out)
    }

    fn elementwise_binary<F>(&self, other: &Tensor, op: F) -> Result<Tensor, String>
    where
        F: Fn(f64, f64) -> f64,
    {
        let out_shape = Self::broadcast_shapes(&self.shape, &other.shape)?;
        let total_numel: usize = out_shape.iter().product();
        let mut result_data = Vec::with_capacity(total_numel);

        // Fast path: identical contiguous shapes
        if self.shape == other.shape && self.is_contiguous() && other.is_contiguous() {
            let d1 = self.data.borrow();
            let d2 = other.data.borrow();
            for i in 0..total_numel {
                result_data.push(op(d1[i], d2[i]));
            }
            return Ok(Tensor::new(out_shape, result_data));
        }

        // General broadcast path
        let ndim = out_shape.len();
        let mut coord = vec![0; ndim];
        let mut c_a = vec![0; self.shape.len()];
        let mut c_b = vec![0; other.shape.len()];

        for _ in 0..total_numel {
            // Map broadcast coordinate to self coordinates
            for (i, dim) in self.shape.iter().enumerate() {
                let out_idx = ndim - self.shape.len() + i;
                c_a[i] = if *dim == 1 { 0 } else { coord[out_idx] };
            }
            // Map broadcast coordinate to other coordinates
            for (i, dim) in other.shape.iter().enumerate() {
                let out_idx = ndim - other.shape.len() + i;
                c_b[i] = if *dim == 1 { 0 } else { coord[out_idx] };
            }

            let val_a = self.get_element(&c_a);
            let val_b = other.get_element(&c_b);
            result_data.push(op(val_a, val_b));

            // Increment broadcast coordinates
            for d in (0..ndim).rev() {
                coord[d] += 1;
                if coord[d] < out_shape[d] {
                    break;
                }
                coord[d] = 0;
            }
        }

        Ok(Tensor::new(out_shape, result_data))
    }

    pub fn add(&self, other: &Tensor) -> Result<Tensor, String> {
        self.elementwise_binary(other, |a, b| a + b)
    }

    pub fn sub(&self, other: &Tensor) -> Result<Tensor, String> {
        self.elementwise_binary(other, |a, b| a - b)
    }

    pub fn mul(&self, other: &Tensor) -> Result<Tensor, String> {
        self.elementwise_binary(other, |a, b| a * b)
    }

    pub fn div(&self, other: &Tensor) -> Result<Tensor, String> {
        self.elementwise_binary(other, |a, b| {
            if b.abs() < 1e-12 { a / (if b >= 0.0 { 1e-12 } else { -1e-12 }) } else { a / b }
        })
    }

    /// Multiply tensor by a scalar value
    pub fn scale(&self, scalar: f64) -> Tensor {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let scaled_data: Vec<f64> = data.iter().map(|&x| x * scalar).collect();
        Tensor::new(self.shape.clone(), scaled_data)
    }

    /// Add a scalar value to every element
    pub fn add_scalar(&self, scalar: f64) -> Tensor {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let out_data: Vec<f64> = data.iter().map(|&x| x + scalar).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    // =========================================================================
    // MODERN NEURAL NETWORK ACTIVATIONS (GELU, SiLU, Softmax, Sigmoid, ReLU)
    // =========================================================================

    /// Rectified Linear Unit: max(0, x)
    pub fn relu(&self) -> Tensor {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let out_data: Vec<f64> = data.iter().map(|&x| if x > 0.0 { x } else { 0.0 }).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Leaky ReLU: max(alpha * x, x)
    pub fn leaky_relu(&self, alpha: f64) -> Tensor {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let out_data: Vec<f64> = data.iter().map(|&x| if x > 0.0 { x } else { alpha * x }).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Gaussian Error Linear Unit (GELU) used in LLMs (GPT, LLaMA, BERT)
    /// GELU(x) = 0.5 * x * (1 + tanh(sqrt(2/pi) * (x + 0.044715 * x^3)))
    pub fn gelu(&self) -> Tensor {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        const SQRT_2_OVER_PI: f64 = 0.7978845608028654;
        const COEFF: f64 = 0.044715;

        let out_data: Vec<f64> = data.iter().map(|&x| {
            let inner = SQRT_2_OVER_PI * (x + COEFF * x * x * x);
            0.5 * x * (1.0 + inner.tanh())
        }).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Numerically stable Sigmoid: 1 / (1 + e^-x) with clamping
    pub fn sigmoid(&self) -> Tensor {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let out_data: Vec<f64> = data.iter().map(|&x| {
            // Clamp to avoid float underflow/overflow
            let clamped = x.clamp(-88.0, 88.0);
            1.0 / (1.0 + (-clamped).exp())
        }).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Hyperbolic Tangent: tanh(x)
    pub fn tanh_act(&self) -> Tensor {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let out_data: Vec<f64> = data.iter().map(|&x| x.tanh()).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// SiLU / Swish activation: x * sigmoid(x)
    pub fn silu(&self) -> Tensor {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let out_data: Vec<f64> = data.iter().map(|&x| {
            let clamped = x.clamp(-88.0, 88.0);
            let sig = 1.0 / (1.0 + (-clamped).exp());
            x * sig
        }).collect();
        Tensor::new(self.shape.clone(), out_data)
    }

    /// Numerically Stable Softmax along specified axis (default -1 / last axis)
    /// Subtracts max(x) along axis to guarantee zero float overflow!
    pub fn softmax(&self, axis: isize) -> Result<Tensor, String> {
        let ndim = self.shape.len();
        if ndim == 0 {
            return Err("Cannot apply softmax to empty tensor".into());
        }
        let actual_axis = if axis < 0 {
            (ndim as isize + axis) as usize
        } else {
            axis as usize
        };
        if actual_axis >= ndim {
            return Err(format!("Softmax axis {} out of bounds for ndim {}", axis, ndim));
        }

        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let mut out_data = data.clone();

        let axis_size = self.shape[actual_axis];
        let outer_size: usize = self.shape[..actual_axis].iter().product();
        let inner_size: usize = self.shape[actual_axis + 1..].iter().product();

        for o in 0..outer_size {
            for i in 0..inner_size {
                let base = o * (axis_size * inner_size) + i;

                // 1. Find max for numerical stability
                let mut max_val = f64::NEG_INFINITY;
                for a in 0..axis_size {
                    let idx = base + a * inner_size;
                    if data[idx] > max_val {
                        max_val = data[idx];
                    }
                }

                // 2. Compute exp(x - max) and sum
                let mut sum_exp = 0.0;
                for a in 0..axis_size {
                    let idx = base + a * inner_size;
                    let exp_val = (data[idx] - max_val).exp();
                    out_data[idx] = exp_val;
                    sum_exp += exp_val;
                }

                // 3. Normalize by sum
                let inv_sum = if sum_exp > 0.0 { 1.0 / sum_exp } else { 1.0 };
                for a in 0..axis_size {
                    let idx = base + a * inner_size;
                    out_data[idx] *= inv_sum;
                }
            }
        }

        Ok(Tensor::new(self.shape.clone(), out_data))
    }

    // =========================================================================
    // LAYER NORMALIZATION & TRANSFORMER ATTENTION
    // =========================================================================

    /// Layer Normalization across the last dimension
    /// y = ((x - mean) / sqrt(variance + eps)) * gamma + beta
    pub fn layernorm(&self, gamma: Option<&Tensor>, beta: Option<&Tensor>, eps: f64) -> Result<Tensor, String> {
        if self.shape.is_empty() {
            return Err("LayerNorm requires non-empty tensor".into());
        }
        let d = *self.shape.last().unwrap();
        let outer_size: usize = self.shape[..self.shape.len() - 1].iter().product();

        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let mut out_data = vec![0.0; data.len()];

        let g_data = gamma.map(|g| g.to_contiguous().to_vec());
        let b_data = beta.map(|b| b.to_contiguous().to_vec());

        for o in 0..outer_size {
            let base = o * d;
            // 1. Calculate mean
            let mut sum = 0.0;
            for i in 0..d {
                sum += data[base + i];
            }
            let mean = sum / (d as f64);

            // 2. Calculate variance
            let mut var_sum = 0.0;
            for i in 0..d {
                let diff = data[base + i] - mean;
                var_sum += diff * diff;
            }
            let variance = var_sum / (d as f64);
            let inv_std = 1.0 / (variance + eps).sqrt();

            // 3. Normalize & scale/shift
            for i in 0..d {
                let norm = (data[base + i] - mean) * inv_std;
                let g = g_data.as_ref().map(|g| g[i % g.len()]).unwrap_or(1.0);
                let b = b_data.as_ref().map(|b| b[i % b.len()]).unwrap_or(0.0);
                out_data[base + i] = norm * g + b;
            }
        }

        Ok(Tensor::new(self.shape.clone(), out_data))
    }

    /// Scaled Dot-Product Attention: Softmax((Q * K^T) / sqrt(d_k)) * V
    /// Runs natively in Rust silicon speed!
    pub fn scaled_dot_product_attention(
        q: &Tensor,
        k: &Tensor,
        v: &Tensor,
        scale: Option<f64>,
    ) -> Result<Tensor, String> {
        let d_k = *q.shape.last().ok_or("Q has empty shape")?;
        let scale_factor = scale.unwrap_or(1.0 / (d_k as f64).sqrt());

        // 1. Transpose K's last two dimensions
        let k_t = k.transpose();

        // 2. Raw attention scores = Q x K^T
        let scores = q.matmul(&k_t)?;

        // 3. Scaled scores
        let scaled_scores = scores.scale(scale_factor);

        // 4. Softmax over last dimension
        let weights = scaled_scores.softmax(-1)?;

        // 5. Weighted values = Weights x V
        weights.matmul(v)
    }

    // =========================================================================
    // REDUCTIONS & SIMILARITY
    // =========================================================================

    /// Sum of all elements in tensor
    pub fn sum_all(&self) -> f64 {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        data.iter().sum()
    }

    /// Mean of all elements in tensor
    pub fn mean_all(&self) -> f64 {
        let n = self.numel();
        if n == 0 { 0.0 } else { self.sum_all() / (n as f64) }
    }

    /// Max element in tensor
    pub fn max_val(&self) -> f64 {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

    /// Min element in tensor
    pub fn min_val(&self) -> f64 {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        data.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }

    /// L2 Euclidean Norm: sqrt(sum(x_i^2))
    pub fn norm_l2(&self) -> f64 {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let sq_sum: f64 = data.iter().map(|&x| x * x).sum();
        sq_sum.sqrt()
    }

    /// Argmax along flat elements
    pub fn argmax(&self) -> usize {
        let cont = self.to_contiguous();
        let data = cont.data.borrow();
        let mut max_idx = 0;
        let mut max_val = f64::NEG_INFINITY;
        for (i, &v) in data.iter().enumerate() {
            if v > max_val {
                max_val = v;
                max_idx = i;
            }
        }
        max_idx
    }

    /// Vector Cosine Similarity: (u . v) / (||u|| * ||v||)
    pub fn cosine_similarity(v1: &[f64], v2: &[f64]) -> f64 {
        let len = v1.len().min(v2.len());
        if len == 0 { return 0.0; }

        let mut dot = 0.0;
        let mut norm1_sq = 0.0;
        let mut norm2_sq = 0.0;

        for i in 0..len {
            dot += v1[i] * v2[i];
            norm1_sq += v1[i] * v1[i];
            norm2_sq += v2[i] * v2[i];
        }

        let denom = norm1_sq.sqrt() * norm2_sq.sqrt();
        if denom < 1e-12 {
            0.0
        } else {
            dot / denom
        }
    }

    /// Pretty print tensor formatting
    pub fn print(&self) {
        if self.shape.len() == 2 {
            print!("[");
            for i in 0..self.shape[0] {
                if i > 0 { print!(" "); }
                print!("[");
                for j in 0..self.shape[1] {
                    print!("{:.4}", self.get_element(&[i, j]));
                    if j < self.shape[1] - 1 { print!(", "); }
                }
                print!("]");
                if i < self.shape[0] - 1 { println!(","); }
            }
            println!("]");
        } else {
            println!("Tensor(shape={:?}, offset={}, numel={})", self.shape, self.offset, self.numel());
        }
    }
}
