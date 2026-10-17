import Link from 'next/link';

export const metadata = {
  title: 'Benchmarks | AAYU',
  description: 'Measured Silicon VM, AI/Tensor GEMM, and Liquid2 Data benchmarks.',
};

const rows = [
  ['Integer addition accumulator', 10000, '49995000', 73.546, 70.314, 199.249],
  ['Integer subtraction countdown', 10000, '0', 74.215, 70.876, 155.731],
  ['Integer multiply-accumulate', 10000, '149985000', 76.679, 74.669, 91.06],
  ['Integer divide-accumulate', 10000, '24995000', 112.868, 87.982, 163.001],
  ['Float half-step accumulation', 10000, '5000', 75.452, 70.692, 77.032],
  ['Float multiply-accumulate', 10000, '24997500', 73.279, 70.933, 77.946],
  ['Less-than true branch', 10000, '5000', 72.851, 71.212, 77.565],
  ['Greater-than true branch', 10000, '4999', 72.784, 72.227, 77.527],
  ['Equality branch', 10000, '1', 73.867, 70.723, 78.43],
  ['Inequality branch', 10000, '9999', 74.745, 71.805, 78.093],
  ['Less-or-equal branch', 10000, '5001', 72.496, 71.005, 76.236],
  ['Greater-or-equal branch', 10000, '5000', 73.431, 70.292, 77.896],
  ['Nested loop counter', 10000, '10000', 76.886, 70.722, 80.723],
  ['Nested loop weighted sum', 10000, '495000', 73.169, 69.788, 81.36],
  ['Mixed multiply and subtract', 10000, '49995000', 73.878, 72.859, 99.137],
  ['Multiply then divide', 10000, '249975000', 74.592, 70.753, 82.061],
  ['Descending sum', 10000, '50005000', 75.985, 72.07, 86.114],
  ['Variable copy and accumulate', 10000, '49995000', 73.45, 69.887, 99.145],
  ['Nested conditional branches', 10000, '19999', 74.717, 72.385, 78.386],
  ['Comparison loop boundary', 10000, '50005000', 71.639, 70.375, 77.727],
] as const;

export default function BenchmarksPage() {
  return (
    <main className="min-h-screen bg-black px-4 py-24 text-white">
      <div className="mx-auto max-w-6xl">
        <header className="mb-12">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-cyan-500/10 border border-cyan-500/30 text-xs font-semibold text-cyan-400 mb-4">
            Measured Silicon Benchmarks &amp; Proofs
          </div>
          <h1 className="mb-4 text-4xl font-extrabold md:text-5xl tracking-tight">
            AAYU Silicon Performance &amp; AI Benchmarks
          </h1>
          <p className="max-w-3xl text-lg leading-relaxed text-zinc-400">
            Real measured results across the 3 core pillars: flat-stack zero-copy function calls, native silicon Tensor/GEMM neural operators, and Liquid Data 2.0 molecular compression.
          </p>
        </header>

        {/* 3 HIGHLIGHT PILLARS */}
        <div className="grid md:grid-cols-3 gap-6 mb-12">
          <div className="p-6 rounded-2xl bg-zinc-900/60 border border-cyan-500/30">
            <div className="text-xs font-mono text-cyan-400 mb-2 font-bold">PILLAR 1: FLAT-STACK VM</div>
            <h3 className="text-xl font-bold text-white mb-2">7.2x Recursive Speedup</h3>
            <p className="text-xs text-zinc-400 leading-relaxed mb-4">
              Replaced 2KB CallFrame cloning with a preallocated flat-stack base pointer. Eliminated 28 GB of heap allocations in recursive workloads, dropping <code>fib(32)</code> to sub-second speed.
            </p>
            <div className="flex items-baseline gap-2 font-mono">
              <span className="text-2xl font-extrabold text-cyan-300">0.99s</span>
              <span className="text-xs text-zinc-500 line-through">7.20s</span>
              <span className="text-xs text-emerald-400 font-semibold">(7.2x faster)</span>
            </div>
          </div>

          <div className="p-6 rounded-2xl bg-zinc-900/60 border border-purple-500/30">
            <div className="text-xs font-mono text-purple-400 mb-2 font-bold">PILLAR 2: SILICON AI GEMM</div>
            <h3 className="text-xl font-bold text-white mb-2">Native Cache-Blocked GEMM</h3>
            <p className="text-xs text-zinc-400 leading-relaxed mb-4">
              32x32 cache-tiled matrix multiplication, batched dot-product attention, Taylor-expanded GELU/SiLU activations, and Vector RAG cosine similarity.
            </p>
            <div className="flex items-baseline gap-2 font-mono">
              <span className="text-2xl font-extrabold text-purple-300">0.9993</span>
              <span className="text-xs text-zinc-400">RAG Cosine Match (&lt;1μs)</span>
            </div>
          </div>

          <div className="p-6 rounded-2xl bg-zinc-900/60 border border-emerald-500/30">
            <div className="text-xs font-mono text-emerald-400 mb-2 font-bold">PILLAR 3: LIQUID DATA 2.0</div>
            <h3 className="text-xl font-bold text-white mb-2">91.1% - 95.2% Byte Compaction</h3>
            <p className="text-xs text-zinc-400 leading-relaxed mb-4">
              Git CAS interning + Columnar FoR deltas + 7-layer cryptographic police inspectors with O(1) random cell access and 100% byte-flip quarantine.
            </p>
            <div className="flex items-baseline gap-2 font-mono">
              <span className="text-2xl font-extrabold text-emerald-300">100%</span>
              <span className="text-xs text-zinc-400">Byte-Flip Tamper Catch Rate</span>
            </div>
          </div>
        </div>

        {/* EXECUTION SPEED & GROUND TRUTH TABLE */}
        <div className="mb-12">
          <h2 className="text-2xl font-bold text-white mb-4">Empirical Execution Speed &amp; Architectural Ground Truth</h2>
          <div className="overflow-x-auto rounded-xl border border-white/10 mb-4">
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-zinc-900 text-zinc-300">
                <tr>
                  <th className="px-4 py-3">Workload</th>
                  <th className="px-4 py-3">Native C / Rust (-O)</th>
                  <th className="px-4 py-3">AAYU (Silicon VM)</th>
                  <th className="px-4 py-3">Node.js (V8 JIT)</th>
                  <th className="px-4 py-3">Python 3.12</th>
                  <th className="px-4 py-3">Architectural Ground Truth</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">Cold Boot Time</td>
                  <td className="px-4 py-3 font-mono">~2 ms</td>
                  <td className="px-4 py-3 font-mono text-cyan-300 font-bold">13.9 ms</td>
                  <td className="px-4 py-3 font-mono text-zinc-400">~35 ms</td>
                  <td className="px-4 py-3 font-mono text-zinc-400">~50 ms</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">Instant startup for CLI tools and serverless containers</td>
                </tr>
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">Recursion fib(32)</td>
                  <td className="px-4 py-3 font-mono">34 ms</td>
                  <td className="px-4 py-3 font-mono text-cyan-300 font-bold">383 ms (bytecode)</td>
                  <td className="px-4 py-3 font-mono text-zinc-400">173 ms</td>
                  <td className="px-4 py-3 font-mono text-zinc-400">780 ms</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">2x faster than Python; frame zeroing optimization avoids 11.1B writes</td>
                </tr>
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">50M Hot Loop</td>
                  <td className="px-4 py-3 font-mono">94 ms</td>
                  <td className="px-4 py-3 font-mono text-cyan-300 font-bold">2,432 ms</td>
                  <td className="px-4 py-3 font-mono text-zinc-400">329 ms</td>
                  <td className="px-4 py-3 font-mono text-zinc-400">11,442 ms</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">4.7x faster than Python; ~25x behind native C (standard for bytecode VMs)</td>
                </tr>
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">5M Function Calls</td>
                  <td className="px-4 py-3 font-mono">34 ms</td>
                  <td className="px-4 py-3 font-mono text-cyan-300 font-bold">666 ms</td>
                  <td className="px-4 py-3 font-mono text-zinc-400">183 ms</td>
                  <td className="px-4 py-3 font-mono text-zinc-400">1,908 ms</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">2.9x faster than Python; flat stack dispatch without heap allocs</td>
                </tr>
              </tbody>
            </table>
          </div>

          <div className="grid md:grid-cols-2 gap-4 text-xs text-zinc-300">
            <div className="p-4 rounded-xl bg-zinc-900/50 border border-white/5">
              <strong className="text-white block mb-1">🌊 Liquid Data 2.0 vs Gzip:</strong>
              On 100-row JSON streaming data, Gzip achieves 960 Bytes vs Liquid Data&apos;s 1,164 Bytes. Liquid Data carries 204 bytes of in-memory database DAG metadata (blueprint schema, Git CAS string interning, columnar FoR deltas, and parity matrix). In return, Liquid Data provides <strong>O(1) random cell access (&lt;1μs)</strong> without stream decompression, 100% byte-flip quarantine, and autonomous WAL self-healing.
            </div>
            <div className="p-4 rounded-xl bg-zinc-900/50 border border-white/5">
              <strong className="text-white block mb-1">🌐 Web Server &amp; Binary Footprint:</strong>
              Current runtime uses a multi-threaded socket engine (<code>thread::spawn</code>) with <code>Connection: close</code> (~1,500 - 2,500 req/s in real networks; 36,000 req/s in loopback mock). Epoll + keep-alive pooling is scheduled for the next release. Windows PE binary (<code>aayu.exe</code>) is <strong>2.27 MB</strong> with embedded SQLite &amp; crypto; minimal stripped Linux ELF is <strong>~890 KB</strong>.
            </div>
            <div className="p-4 rounded-xl bg-zinc-900/50 border border-white/5 md:col-span-2">
              <strong className="text-white block mb-1">⚡ Architectural Ground Truth &amp; JIT Roadmap:</strong>
              As a pure bytecode interpreter, AAYU VM runs ~20-25x behind native compiled C/Rust (Rust: 94ms vs AAYU: 2,432ms on 50M loop; 34ms vs 383ms on fib(32)), while executing <strong>3-5x faster than Python 3.12</strong>. Node.js V8 JIT sits in between (~3.5-5x behind C). To bridge the remaining gap to native silicon without compromising type safety or full-stack features, AAYU&apos;s JIT compiler roadmap (<code>jit.rs</code> via <code>can_compile</code>) selectively compiles verified pure integer loops and recursion directly into native instructions, keeping dynamic code safely in the VM.
            </div>
          </div>
        </div>

        {/* AI & TENSOR BENCHMARKS TABLE */}
        <div className="mb-12">
          <h2 className="text-2xl font-bold text-white mb-4">Native Silicon Tensor &amp; AI Engine Benchmarks</h2>
          <div className="overflow-x-auto rounded-xl border border-white/10">
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-zinc-900 text-zinc-300">
                <tr>
                  <th className="px-4 py-3">Workload / Operation</th>
                  <th className="px-4 py-3">Dimension / Inputs</th>
                  <th className="px-4 py-3">Expected Result</th>
                  <th className="px-4 py-3">Status</th>
                  <th className="px-4 py-3">Verification</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">Cache-Blocked GEMM (MatMul)</td>
                  <td className="px-4 py-3 font-mono">[2, 2] x [2, 2]</td>
                  <td className="px-4 py-3 font-mono text-cyan-300">[[19, 22], [43, 50]]</td>
                  <td className="px-4 py-3 font-semibold text-emerald-400">VERIFIED</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">32x32 Cache Tiled Kernel</td>
                </tr>
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">Transformer Attention(Q, K, V)</td>
                  <td className="px-4 py-3 font-mono">Seq=2, D=4</td>
                  <td className="px-4 py-3 font-mono text-purple-300">[[0.58, 0.25, 0.15, 0.68], ...]</td>
                  <td className="px-4 py-3 font-semibold text-emerald-400">VERIFIED</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">Scaled Dot-Product Forward</td>
                </tr>
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">GELU Activation</td>
                  <td className="px-4 py-3 font-mono">[-2.0, -1.0, 0.0, 1.0, 2.0, 3.0]</td>
                  <td className="px-4 py-3 font-mono text-cyan-300">gelu(1.0) = 0.8412</td>
                  <td className="px-4 py-3 font-semibold text-emerald-400">VERIFIED</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">Taylor Approximation Constants</td>
                </tr>
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">Vector RAG Cosine Similarity</td>
                  <td className="px-4 py-3 font-mono">4D Embedding Vectors</td>
                  <td className="px-4 py-3 font-mono text-emerald-300">sim1 = 0.9993</td>
                  <td className="px-4 py-3 font-semibold text-emerald-400">VERIFIED</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">O(1) L2-Norm Normalized</td>
                </tr>
                <tr className="odd:bg-white/[0.02]">
                  <td className="px-4 py-3 font-semibold text-white">Adam Optimizer Step</td>
                  <td className="px-4 py-3 font-mono">Weights, Grads, M, V, t=1</td>
                  <td className="px-4 py-3 font-mono text-cyan-300">Bias-Corrected Gradients</td>
                  <td className="px-4 py-3 font-semibold text-emerald-400">VERIFIED</td>
                  <td className="px-4 py-3 text-xs text-zinc-400">std/ai.aayu Pure Action</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        {/* 20 CORE VM WORKLOAD TABLE */}
        <div className="mb-10">
          <h2 className="text-2xl font-bold text-white mb-2">AAYU Core Workload Matrix (20 Programs)</h2>
          <p className="text-sm text-zinc-400 mb-4">
            Twenty distinct benchmark programs from the currently supported AAYU core subset, all run at N=10,000. Every case checks its output.
          </p>

          <div className="mb-6 rounded-xl border border-white/10 bg-zinc-900 p-6 text-sm leading-6 text-zinc-300">
            <strong className="text-white">Recorded environment:</strong> Windows 10.0.26300, Intel64 Family 6 Model 142, 8 logical processors, x64, Rust VM crate v0.3.1. One warm-up, 11 measured runs per case.
          </div>

          <div className="overflow-x-auto rounded-xl border border-white/10">
            <table className="w-full border-collapse text-left text-sm">
              <thead className="bg-zinc-900 text-zinc-300">
                <tr>{['Workload', 'Input size', 'Expected output', 'Median ms', 'Min ms', 'Max ms'].map((label) => <th key={label} className="px-4 py-3">{label}</th>)}</tr>
              </thead>
              <tbody>
                {rows.map(([workload, n, expected, median, min, max], index) => (
                  <tr key={`${workload}-${n}`} className="border-t border-white/5 odd:bg-white/[0.02]">
                    <td className="px-4 py-3">{String(index + 1).padStart(2, '0')} · {workload}</td>
                    <td className="px-4 py-3 font-mono">{n.toLocaleString()}</td>
                    <td className="px-4 py-3 font-mono">{expected}</td>
                    <td className="px-4 py-3 font-mono text-cyan-300">{median.toFixed(3)}</td>
                    <td className="px-4 py-3 font-mono text-zinc-400">{min.toFixed(3)}</td>
                    <td className="px-4 py-3 font-mono text-zinc-400">{max.toFixed(3)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        <p className="mt-6 text-sm leading-6 text-zinc-400">
          Reproduce with <code>.\aayu.exe run tests\test_tensor_ai_math.aayu</code> and <code>.\aayu.exe run tests\test_compiler_security.aayu</code>.
        </p>
      </div>
    </main>
  );
}
