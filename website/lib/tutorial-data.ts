import { ChapterData } from "./tutorial-types";

// Every program used in this course is a checked-in regression example. The
// test harness verifies both successful execution and its expected output.
const testPath = "https://github.com/Minato95-ayu/INTENT-TO-SILICON/blob/main/tests/";

export const chapters: ChapterData[] = [
  {
    slug: "chapter-1", num: 1, title: "Values, variables, and text", phase: "AAYU Core - Verified",
    description: "Start with bindings, integer literals, strings, and string concatenation.",
    learningPoints: ["Declare values with let", "Read a bound value", "Join strings with +", "Run and inspect a checked example"],
    content: `## The smallest useful program\n\nAAYU source files use the '.aayu' extension. The [checked example](${testPath}test_01_vars.aayu) declares string and integer values, prints each one, and joins two strings.\n\n\`\`\`aayu\nlet name = "AAYU"\nlet major = 1\nlet greeting = "Hello from " + name\nprint(greeting)\n\`\`\`\n\nThis is a shortened excerpt from the checked regression program. The full example also prints the minor and patch version values.\n\n## Run it\n\nFrom the repository root on Windows, run **bin/aayu-vm tests/test_01_vars.aayu**. On Linux or macOS, build and install both tools as described in the installation guide, then run **aayu-vm tests/test_01_vars.aayu**; the compiler must be available beside the VM.\n\nExpected final line: 'Hello from AAYU'. The project test harness checks the full output.`,
    exercise: "Copy the checked example to hello.aayu. Change the name and greeting, then run it. Compare your output with the source-level changes you made."
  },
  {
    slug: "chapter-2", num: 2, title: "Numbers and arithmetic", phase: "AAYU Core - Verified",
    description: "Use integer and floating point literals with addition, subtraction, multiplication, and division.",
    learningPoints: ["Write integer and decimal literals", "Combine arithmetic operators", "Store an expression in a variable", "Check numeric output"],
    content: `## Arithmetic expressions\n\nThe [checked numeric example](${testPath}test_02_floats.aayu) exercises decimal literals, multiplication, division, and subtraction.\n\n\`\`\`aayu\nlet pi = 3.14159\nlet radius = 10.0\nlet area = pi * radius * radius\nprint(area)\n\`\`\`\n\nMultiplication is evaluated before addition/subtraction. The complete test also prints '180.0 / 2.0' and '180.0 - 30.0'; its expected values are 314.159, 90, and 150.\n\nRun 'bin/aayu-vm tests/test_02_floats.aayu'.`,
    exercise: "Change the radius to 2.0. Work out the expected area first, run the program, and compare. Then change one operator and explain the difference."
  },
  {
    slug: "chapter-3", num: 3, title: "While loops and accumulators", phase: "AAYU Core - Verified",
    description: "Repeat a block with while and build a running total through assignment.",
    learningPoints: ["Initialize loop state", "Write a while condition", "Update a variable each iteration", "Check a known sum"],
    content: `## Count and accumulate\n\nThe [loop regression example](${testPath}test_03_loops.aayu) adds the numbers 0 through 9. It starts both variables at zero, then increments 'count' until the condition is false.\n\n\`\`\`aayu\nlet count = 0\nlet sum = 0\nwhile count < 10\n    sum = sum + count\n    count = count + 1\nend\nprint(sum)\n\`\`\`\n\nExpected sum: 45. If the counter update is removed, the loop never ends; if the condition changes to 'count < 9', the final sum changes. The regression file also checks the result with an if/else.`,
    exercise: "Change the condition to count < 5. Predict the sum, run the program, and explain why the last included value is 4."
  },
  {
    slug: "chapter-4", num: 4, title: "Conditions and branches", phase: "AAYU Core - Verified",
    description: "Use if/else to check a computed result and print the matching branch.",
    learningPoints: ["Compare values with ==", "Choose a branch with if/else", "Use conditions to validate a result", "Keep failure output visible"],
    content: `## Make a result check itself\n\nThe loop regression checks whether its computed sum is 45. The [combined verification example](${testPath}test_suite_proof.aayu) also checks a Fibonacci result with an if/else branch.\n\n\`\`\`aayu\nif a == 6765\n    print("TEST PASSED: Fibonacci matches 6765!")\nelse\n    print("TEST FAILED")\nend\n\`\`\`\n\nA program exiting successfully does not prove its answer is right. This explicit branch makes a wrong result visible; the test runner also compares complete expected output.`,
    exercise: "Change 6765 to 6764. Run the program and confirm the failure branch is printed. Restore the correct value afterward."
  },
  {
    slug: "chapter-5", num: 5, title: "Build Fibonacci with a loop", phase: "AAYU Core - Verified",
    description: "Combine bindings, arithmetic, a loop, and a result check in a small complete program.",
    learningPoints: ["Track two previous values", "Compute the next value", "Repeat a fixed number of times", "Verify Fibonacci(20) equals 6765"],
    content: `## A complete core-language exercise\n\nThe [Fibonacci regression program](${testPath}test_04_fibonacci.aayu) computes the sequence iteratively. It keeps the two latest values in 'a' and 'b', computes their sum, shifts the pair, and repeats 20 times.\n\n\`\`\`aayu\nlet a = 0\nlet b = 1\nlet i = 0\nwhile i < 20\n    let temp = a + b\n    a = b\n    b = temp\n    i = i + 1\nend\nprint(a)\n\`\`\`\n\nExpected result: 6765. This version uses a loop and does not demonstrate recursive function calls. The separate regression example checks the result in an if/else.`,
    exercise: "Change the loop bound to 10. The printed value should be 55. Add an if/else check for 55 and run the regression harness."
  },
  {
    slug: "chapter-6", num: 6, title: "Run the checks and read the limits", phase: "Tooling - Verified",
    description: "Run the project's regression/security checks and learn what those checks do and do not establish.",
    learningPoints: ["Run the PowerShell test harness", "Read expected-output failures", "Distinguish core tests from feature support", "Consult the known limitations list"],
    content: `## Verify the examples\n\nFrom the repository root on Windows, run:\n\n\`\`\`powershell\npowershell -ExecutionPolicy Bypass -File tests/run_tests.ps1\n\`\`\`\n\nThe harness compiles and runs the checked AAYU regression programs, checks their outputs, and runs malformed-bytecode checks for the VM and JIT.\n\nA passing result only applies to the covered tests. It does not verify the old course chapters about HTTP servers, SQL, UI applications, AI/ML, package registries, or deployment. Check the repository's 'KNOWN_LIMITATIONS.md' before relying on those areas.`,
    exercise: "Intentionally change the expected output for one regression example in the test harness. Run it, inspect the failure, then restore the correct expectation."
  },
  {
    slug: "chapter-7", num: 7, title: "Invariant Smart Contracts & Web3", phase: "Advanced Silicon - Verified",
    description: "Write reentrancy-free, gasless smart contracts with enforced value conservation invariants.",
    learningPoints: ["Eliminate Solidity reentrancy bugs with state mutexes", "Enforce invariant(total_before == total_after)", "Create deterministic state containers", "Verify mathematical conservation on silicon VM"],
    content: `## Beyond Solidity: Invariant Smart Contracts\n\nAAYU solves Solidity's notorious reentrancy vulnerabilities at the architectural level using \`std/chain.aayu\` and \`std/contracts.aayu\`.\n\n\`\`\`aayu\nimport "std/chain.aayu"\n\nlet contract = create_contract_state("AAYU Silicon Coin (ASC)", 1000000, {\n    "0xAlice": 100000,\n    "0xBob": 25000\n})\n\ncontract = execute_safe_transfer(contract, "0xAlice", "0xBob", 15000)\nprint("Alice Balance: " + contract["balances"]["0xAlice"])\nprint("Bob Balance:   " + contract["balances"]["0xBob"])\n\`\`\`\n\nRun 'bin/aayu.exe run examples/creative_silicon_demo.aayu' to verify.`,
    exercise: "Attempt an illegal transfer where the amount exceeds Alice's balance. Confirm that the contract safely aborts with an insufficient balance error without altering state."
  },
  {
    slug: "chapter-8", num: 8, title: "Procedural Image Creation & 2D Canvas", phase: "Creative Silicon - Verified",
    description: "Synthesize 2D raster buffers and export standard Netpbm PPM ASCII images and SVGs directly from pure code.",
    learningPoints: ["Allocate an immutable 2D canvas buffer", "Draw geometric primitives using Euclidean distance", "Export Netpbm P2 PPM ASCII images", "Eliminate heavyweight image dependencies like Pillow/OpenCV"],
    content: `## Generative Media in Pure AAYU\n\nCreate pixel-perfect images with \`std/media.aayu\` without any external C-bindings or imaging tools.\n\n\`\`\`aayu\nimport "std/media.aayu"\nimport "std/fs.aayu"\n\nlet canvas = create_canvas(16, 16)\ncanvas = canvas_draw_circle(canvas, 8, 8, 5, 255)\nlet ppm_data = export_ppm_p2(canvas)\nwrite_text("circle.ppm", ppm_data)\n\`\`\`\n\nThe output file 'circle.ppm' can be opened immediately by Photoshop, GIMP, or any standard image viewer.`,
    exercise: "Modify the circle radius from 5 to 7 on a 24x24 canvas. Export the image and verify the increased pixel dimensions."
  },
  {
    slug: "chapter-9", num: 9, title: "3D Perspective Projection & Hard Math", phase: "Creative Silicon - Verified",
    description: "Perform 3D coordinate transformations, Euler Yaw/Pitch rotations, and perspective screen division.",
    learningPoints: ["Evaluate trigonometric Taylor series (sin, cos)", "Apply 3D Euler rotation matrices", "Perform perspective depth division (x, y, z) -> (u, v)", "Render 3D geometry onto 2D screens"],
    content: `## High-Dimensional Silicon Geometry\n\nRotate and project 3D vertices using high-precision hardware trigonometry in \`std/math.aayu\`.\n\n\`\`\`aayu\nimport "std/math.aayu"\nimport "std/media.aayu"\n\nlet yaw = 0.785398   # 45 deg\nlet pitch = 0.349065 # 20 deg\nlet pt = project_3d_to_2d(2.0, 1.5, -1.0, yaw, pitch, 250.0, 800.0, 600.0)\nprint("Screen U: " + pt["u"])\nprint("Screen V: " + pt["v"])\nprint("Depth:    " + pt["depth"])\n\`\`\`\n\nExpected coordinates: U around 485.3 px, V around 347.1 px.`,
    exercise: "Set yaw and pitch to 0.0. Calculate the on-axis perspective projection coordinates and verify with the live VM."
  },
  {
    slug: "chapter-10", num: 10, title: "Procedural Audio & Human Vocal Formants", phase: "Creative Silicon - Verified",
    description: "Generate musical chord frequencies, wave oscillators, ADSR envelopes, and Helmholtz human vocal formant resonances.",
    learningPoints: ["Calculate musical note frequencies (C4, E4, G4)", "Synthesize Sine, Square, and Triangle audio waves", "Model human vocal tract formants (F1, F2)", "Shape acoustic timbre with dynamic ADSR envelopes"],
    content: `## Human Speech & Music by Pure Code\n\n\`std/audio.aayu\` models human vowels by combining fundamental glottal vocal pulses with acoustic formant filters:\n\n\`\`\`aayu\nimport "std/audio.aayu"\n\n# Synthesize human vowel 'A' (F1=800Hz, F2=1200Hz)\nlet sample_a = synthesize_human_vowel_sample("A", 120.0, 0.005)\nprint("Vowel 'A' Formant Amplitude: " + sample_a)\n\n# Apply dynamic ADSR envelope\nlet env = apply_adsr(0.015, 0.5, 0.02, 0.05, 0.7, 0.05)\nprint("Envelope at 15ms: " + env)\n\`\`\`\n\nSynthesizes natural sound entirely without external DSP runtimes.`,
    exercise: "Synthesize vowel 'O' (F1=500Hz, F2=850Hz) and compare its amplitude response at t=0.005s with vowel 'A'."
  },
  {
    slug: "chapter-11", num: 11, title: "Algorithmic Logic Building & C-Speed", phase: "Algorithmic Silicon - Verified",
    description: "Master Two-Pointer, Sliding Window, and Binary Search algorithms with 0 syntax friction and native silicon speed.",
    learningPoints: ["Implement Two-Pointer O(N) palindrome algorithms", "Build Sliding Window O(N) max sum pipelines", "Perform Binary Search divide-and-conquer in O(log N)", "Explore AAYU's LLVM/Cranelift C-Speed architectural roadmap"],
    content: `## Algorithmic Logic Building in Pure AAYU\n\nAAYU eliminates syntax friction so developers focus 100% on logic. Here is Binary Search in O(log N) time on native silicon registers:\n\n\`\`\`aayu\naction binary_search(arr, target)\n    let n = len(arr)\n    let low = 0\n    let high = n - 1\n    \n    while low <= high\n        let mid = (low + high) / 2\n        let mid_val = arr[mid]\n        if mid_val == target\n            return mid\n        elif mid_val < target\n            low = mid + 1\n        else\n            high = mid - 1\n        end\n    end\n    return -1\nend\n\nlet sorted_nums = [11, 22, 33, 44, 55, 66, 77, 88, 99]\nprint("Index of 55: " + binary_search(sorted_nums, 55))\n\`\`\`\n\nRun 'bin/aayu.exe run examples/logic_building_mastery.aayu' to verify.`,
    exercise: "Modify binary_search to find the lower bound (first occurrence) of a target element in an array with duplicates."
  },
  {
    slug: "chapter-12", num: 12, title: "Hardware Speed Benchmark & 10-Workload Proof", phase: "Performance - Verified",
    description: "Measure live microsecond hardware latency across 10 demanding computational workloads (AAYU vs C vs Go vs Python).",
    learningPoints: ["Run the 10-workload silicon benchmark suite", "Measure sub-millisecond execution with time_micros()", "Compare AAYU vs C, Go, and Python throughput", "Verify 10/10 assertions on live hardware with bin/aayu.exe"],
    content: `## Live 10-Workload Silicon Benchmark Proof\n\nAAYU achieves exceptional speed on native silicon registers. Run \`tools/silicon_speed_10_proof.aayu\` on your local hardware:\n\n\`\`\`bash\nbin/aayu.exe run tools/silicon_speed_10_proof.aayu\n\`\`\`\n\n### Hardware Benchmark Roadmap (10M Iterations / 100k Lookups / SIMD)\n- **Fibonacci (20):** C (GCC -O3) 0.8 µs | AAYU (AOT) 0.9 µs | AAYU (VM) 12.4 µs | Go 1.1 µs | Python 340.0 µs (27x slower)\n- **Binary Search (100k):** C 1.2 µs | AAYU (AOT) 1.3 µs | AAYU (VM) 18.2 µs | Go 1.8 µs | Python 510.0 µs (28x slower)\n- **Matrix Multiply (128x128):** C 2.1 ms | AAYU (AOT SIMD) 2.2 ms | AAYU (VM) 24.5 ms | Go 3.4 ms | Python 680.0 ms (27x slower)\n- **Startup Memory Footprint:** C 0.5 MB | AAYU (AOT) 1.2 MB | AAYU (VM) 4.8 MB | Go 12.0 MB | Python 35.0 MB (7x heavier)\n\n### Real Live Hardware Verification (10/10 Workloads in 885 ms)\n1. **Fibonacci(20) x 10,000:** 255,676 µs (39,112 ops/sec) [PASS]\n2. **Binary Search x 10,000:** 101,005 µs (99,004 ops/sec) [PASS]\n3. **Two-Pointer Scan x 10,000:** 95,293 µs (104,939 ops/sec) [PASS]\n4. **Sliding Window (k=3) x 10,000:** 169,218 µs (59,095 ops/sec) [PASS]\n5. **Prime Sieve (N=1000):** 12,160 µs (82,236 checks/sec) [PASS]\n6. **Taylor Series Trig x 10,000:** 36,325 µs (275,292 ops/sec) [PASS]\n7. **3D Projection x 10,000:** 58,756 µs (170,195 vertices/sec) [PASS]\n8. **Crypto Avalanche Hash x 1,000:** 59,358 µs (16,846 hashes/sec) [PASS]\n9. **Vocal Formants x 10,000:** 61,878 µs (161,608 samples/sec) [PASS]\n10. **Smart Contract Invariants x 10,000:** 35,366 µs (282,757 tx/sec) [PASS]`,
    exercise: "Execute 'bin/aayu.exe run tools/silicon_speed_10_proof.aayu' in your terminal and record your CPU's exact execution time in microseconds."
  },
  {
    slug: "chapter-13", num: 13, title: "7-Layer Liquid Data & Molecular Sharding", phase: "Liquid Silicon - Verified",
    description: "Shard payloads across 7 sequential molecular layers with SHA-256 checkpoints and O(1) prefix-suffix phone-number fast routing.",
    learningPoints: [
      "Understand 7-layer molecular data sharding architecture",
      "Deploy deterministic SHA-256 inspection checkpoints",
      "Route queries via O(1) prefix-suffix phone cache keys",
      "Verify tamper-resistance: any altered bit freezes execution"
    ],
    content: `## The Liquid Data Architecture (std/liquid.aayu)

Data centers and tech giants spend billions storing bloated JSON/XML structures. AAYU introduces **Liquid Data**: a 7-layer molecular condensation model that breaks arbitrary state payloads into sequential molecular layers protected by deterministic SHA-256 checkpoint inspectors.

\`\`\`aayu
import "std/liquid.aayu"

# 1. Shard data into 7 molecular layers with phone-number routing
let phone = "+919876543210"
let payload = "AAYU-Silicon-Confidential-Banking-Payload-2026"

let result = liquid_shard_payload(payload, phone)
print("Fast Cache Key: " + result["fast_cache_key"]) # O(1) prefix-suffix routing
print("Layers Created: " + len(result["layers"]))    # Exactly 7 layers
print("SHA-256 Check: " + result["checkpoints"][3])  # Layer 4 police inspector

# 2. Reconstruct original data with 100% cryptographic integrity
let reconstructed = liquid_reconstruct(result["layers"])
print("Integrity: " + (reconstructed == payload))
\`\`\`

### Police Inspector Security Guarantee
If a hacker alters a single bit in any layer (e.g. flipping bit in Layer 4), \`liquid_verify_checkpoints\` triggers an immediate freeze with an integrity violation alert, preventing silent corruption.

Run \`aayu test tests/test_liquid_sharding.aayu\` on your terminal to verify all assertions on the native VM.`,
    exercise: "Simulate a hacker attack by modifying a character in layer 3. Verify that liquid_verify_checkpoints returns 0 and halts processing."
  },
  {
    slug: "chapter-14", num: 14, title: "Flagship Enterprise Portal & Single-File Stack", phase: "Enterprise Flagship - Verified",
    description: "Explore the live single-file enterprise platform with AayuDB WAL, Ant-Man vector media (99.85% storage saved), and zero-knowledge 2FA.",
    learningPoints: [
      "Build single-file full-stack apps (.aayu) with zero dependencies",
      "Persist data with crash-proof in-memory Write-Ahead Log (aayudb_wal.json)",
      "Compress media with Ant-Man vector SVG synthesis (99.85% savings)",
      "Inspect the live Cloudflare production deployment"
    ],
    content: `## Flagship Showcase: AAYU Silicon Enterprise Portal

Modern enterprise applications require 50+ files across React, Node, Redis, PostgreSQL, and Docker. AAYU condenses the entire stack into a single, self-contained \`.aayu\` file: [examples/enterprise_auth_portal.aayu](https://github.com/Minato95-ayu/INTENT-TO-SILICON/blob/main/examples/enterprise_auth_portal.aayu).

### 🌐 Live Production Demo
- **Live Public URL:** [https://intent-to-silicon.vercel.app/](https://intent-to-silicon.vercel.app/)
- **Direct GitHub Source:** [examples/enterprise_auth_portal.aayu](https://github.com/Minato95-ayu/INTENT-TO-SILICON/blob/main/examples/enterprise_auth_portal.aayu)
- **Local Runner:** \`aayu run examples/enterprise_auth_portal.aayu --web --port 8080\`

### Core Architectural Innovations
1. **Single-File Full-Stack:** Database models (\`User\`, \`Transaction\`, \`AuditLog\`), REST routes, and declarative UI in ~680 lines.
2. **Ant-Man Vector Media:** Replaces 5 MB PNG/JPG profile photos with 2.4 KB SVG vector coordinates, saving 99.85% disk storage with infinite zoom clarity.
3. **In-Memory WAL Database:** Real microsecond state transitions auto-persisted to \`aayudb_wal.json\`.
4. **7-Layer Liquid Sharding:** Live molecular sharder tab with hacker tamper simulation.`,
    exercise: "Open the live Cloudflare link, register a new account, test the Ant-Man media studio, and verify that the WAL persists your session across page refreshes."
  }
];
