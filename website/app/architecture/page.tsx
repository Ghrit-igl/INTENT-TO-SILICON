'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import { 
  Cpu, 
  Layers, 
  Terminal, 
  ShieldCheck, 
  Zap, 
  Database, 
  Network, 
  Code2, 
  GitBranch, 
  CheckCircle2, 
  ArrowRight,
  Sparkles,
  Server,
  FileCode2
} from 'lucide-react';

const OPCODES = [
  { op: '0x00', name: 'OP_HALT', width: '1 byte', stack: '[] -> []', desc: 'Terminates execution cleanly' },
  { op: '0x01', name: 'OP_PUSH_CONST', width: '3 bytes', stack: '[] -> [val]', desc: 'Pushes 16-bit constant from pool' },
  { op: '0x02', name: 'OP_POP', width: '1 byte', stack: '[val] -> []', desc: 'Pops top stack value' },
  { op: '0x05', name: 'OP_LOAD_VAR', width: '2 bytes', stack: '[] -> [val]', desc: 'Loads local variable from slot 0..255' },
  { op: '0x06', name: 'OP_STORE_VAR', width: '2 bytes', stack: '[val] -> []', desc: 'Stores top stack item to local slot' },
  { op: '0x10', name: 'OP_ADD', width: '1 byte', stack: '[a, b] -> [a+b]', desc: 'Polymorphic add (Int, Float, Str, Arr)' },
  { op: '0x11', name: 'OP_SUB', width: '1 byte', stack: '[a, b] -> [a-b]', desc: 'Numeric subtract (Int, Float)' },
  { op: '0x12', name: 'OP_MUL', width: '1 byte', stack: '[a, b] -> [a*b]', desc: 'Numeric multiply (Int, Float)' },
  { op: '0x13', name: 'OP_DIV', width: '1 byte', stack: '[a, b] -> [a/b]', desc: 'Numeric division with zero check' },
  { op: '0x20', name: 'OP_JMP', width: '3 bytes', stack: '[] -> []', desc: 'Unconditional jump to 16-bit offset' },
  { op: '0x21', name: 'OP_JMP_IF_FALSE', width: '3 bytes', stack: '[cond] -> []', desc: 'Jumps if popped condition is false' },
  { op: '0x22', name: 'OP_CALL', width: '3 bytes', stack: '[fn, args] -> [res]', desc: 'Invokes function or action frame' },
  { op: '0x23', name: 'OP_RET', width: '1 byte', stack: '[res] -> [res]', desc: 'Returns from action frame' },
  { op: '0x26', name: 'OP_CMP_EQ', width: '1 byte', stack: '[a, b] -> [bool]', desc: 'Equality comparison (a == b)' },
  { op: '0x29', name: 'OP_CMP_LT', width: '1 byte', stack: '[a, b] -> [bool]', desc: 'Less-than comparison (a < b)' },
  { op: '0x30', name: 'OP_STORE_STATE', width: '3 bytes', stack: '[val] -> []', desc: 'Stores runtime state; no WAL guarantee' },
  { op: '0x31', name: 'OP_LOAD_STATE', width: '3 bytes', stack: '[] -> [val]', desc: 'Reads key-value state from RAM' },
  { op: '0x51', name: 'OP_PRINT', width: '1 byte', stack: '[val] -> []', desc: 'Native stdout print with newline' },
];

const DIRECTORY_TREE = [
  { dir: 'compiler/', tech: 'Pure AAYU (.aayu)', desc: 'Self-hosted lexer, parser, semantic analyzer, and compiler written 100% in AAYU.' },
  { dir: 'runtime_rs/', tech: 'Native Rust', desc: 'Bytecode VM, flat-stack base pointer, JIT engine, 4KB slotted disk B-Tree engine, ring BoringSSL FIPS crypto, and bounded thread pool server.' },
  { dir: 'bin/', tech: 'Native Binaries & Seed', desc: 'Precompiled aayu-vm.exe, aayuc.exe, aayu.exe, and aayuc.ayc bytecode seed.' },
  { dir: 'std/', tech: 'Pure AAYU (.aayu)', desc: 'Standard libraries: std/crypto.aayu (audited FIPS), std/payment.aayu, std/cloud_storage.aayu, std/oauth.aayu, std/db.aayu (4KB B-Tree), std/llm.aayu, std/quant.aayu, std/autograd.aayu, std/scientific.aayu, std/vector_db.aayu, std/tensor.aayu, std/liquid2.aayu, std/intent.aayu.' },
  { dir: 'tests/', tech: 'Pure AAYU (.aayu)', desc: 'Verification suites: 365 passing assertions across 20 suites (100% pass: Audited FIPS crypto, 4KB disk B-Tree DB, turnkey cloud/payment SDKs, Next-Gen AI/Math, compiler security, module namespaces, break/continue).' },
  { dir: 'benchmarks/', tech: 'Pure AAYU (.aayu)', desc: 'Empirical benchmark programs measuring loops, recursion, tensors, and memory compaction.' },
  { dir: 'examples/', tech: 'Pure AAYU (.aayu)', desc: 'Full-stack application examples: Conduit Realworld, Todo MVC, URL Shortener, Chat Server.' },
  { dir: 'website/', tech: 'Next.js, React, TypeScript', desc: 'Official documentation site, live benchmarks, architecture blueprint, and interactive tutorial.' },
  { dir: 'vscode-aayu/', tech: 'TypeScript / JSON', desc: 'VS Code syntax highlighting, code snippets, and language extension.' },
];

export default function ArchitecturePage() {
  const [activeTab, setActiveTab] = useState<'compiler' | 'vm' | 'bootstrap'>('compiler');

  return (
    <main className="min-h-screen pt-28 pb-24 bg-black text-white selection:bg-indigo-500/30">
      <div className="container mx-auto px-4 max-w-6xl">
        
        {/* Header Banner */}
        <div className="text-center max-w-3xl mx-auto mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-indigo-950/60 border border-indigo-500/30 text-indigo-400 text-xs font-semibold mb-6">
            <Sparkles className="w-3.5 h-3.5" />
            INTENT-TO-SILICON ARCHITECTURAL BLUEPRINT
          </div>
          <h1 className="text-4xl sm:text-6xl font-extrabold tracking-tight mb-6 bg-gradient-to-b from-white via-zinc-200 to-zinc-500 bg-clip-text text-transparent">
            System Design & Architecture
          </h1>
          <p className="text-lg text-zinc-400 leading-relaxed">
            An overview of the compiler sources, Rust bootstrap compiler, bytecode VM, and related tools in this repository.
          </p>
          <div className="flex flex-wrap items-center justify-center gap-4 mt-8">
            <a 
              href="https://github.com/Minato95-ayu/INTENT-TO-SILICON/blob/main/COMPILER_CONFIDENCE_REPORT.md" 
              target="_blank" 
              rel="noreferrer"
              className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-indigo-600 text-white font-semibold text-sm hover:bg-indigo-500 transition-colors shadow-lg shadow-indigo-500/20"
            >
              <ShieldCheck className="w-4 h-4" />
              Compiler Confidence Report
            </a>
            <a 
              href="https://github.com/Minato95-ayu/INTENT-TO-SILICON/blob/main/ARCHITECTURE.md" 
              target="_blank" 
              rel="noreferrer"
              className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-white text-black font-semibold text-sm hover:bg-zinc-200 transition-colors"
            >
              <FileCode2 className="w-4 h-4" />
              View Full ARCHITECTURE.md on GitHub
            </a>
            <Link
              href="/playground"
              className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-zinc-900 border border-white/10 text-zinc-200 font-semibold text-sm hover:bg-zinc-800 transition-colors"
            >
              Test in Live Playground
              <ArrowRight className="w-4 h-4" />
            </Link>
          </div>
        </div>

        {/* 3 Core Layers Interactive Switcher */}
        <div className="mb-20">
          <div className="flex border-b border-zinc-800 gap-2 mb-8 overflow-x-auto pb-2">
            <button
              onClick={() => setActiveTab('compiler')}
              className={`flex items-center gap-2 px-5 py-3 rounded-lg text-sm font-semibold transition-all ${
                activeTab === 'compiler'
                  ? 'bg-indigo-600/20 text-indigo-400 border border-indigo-500/40'
                  : 'text-zinc-400 hover:text-white hover:bg-zinc-900'
              }`}
            >
              <Code2 className="w-4 h-4" />
              1. Compiler Sources (.aayu)
            </button>
            <button
              onClick={() => setActiveTab('vm')}
              className={`flex items-center gap-2 px-5 py-3 rounded-lg text-sm font-semibold transition-all ${
                activeTab === 'vm'
                  ? 'bg-cyan-600/20 text-cyan-400 border border-cyan-500/40'
                  : 'text-zinc-400 hover:text-white hover:bg-zinc-900'
              }`}
            >
              <Cpu className="w-4 h-4" />
              2. Silicon Virtual Machine (Rust)
            </button>
            <button
              onClick={() => setActiveTab('bootstrap')}
              className={`flex items-center gap-2 px-5 py-3 rounded-lg text-sm font-semibold transition-all ${
                activeTab === 'bootstrap'
                  ? 'bg-emerald-600/20 text-emerald-400 border border-emerald-500/40'
                  : 'text-zinc-400 hover:text-white hover:bg-zinc-900'
              }`}
            >
              <GitBranch className="w-4 h-4" />
              3. Reproducibility & Bootstrap
            </button>
          </div>

          {activeTab === 'compiler' && (
            <div className="bg-zinc-950 border border-zinc-800 rounded-2xl p-6 sm:p-8">
              <div className="flex items-center justify-between mb-6 flex-wrap gap-4">
                <div>
                  <span className="text-xs uppercase font-bold tracking-widest text-indigo-400">Layer 1: AAYU Compiler Sources</span>
                  <h3 className="text-2xl font-bold text-white mt-1">compiler/*.aayu</h3>
                </div>
                <span className="px-3 py-1 rounded-full text-xs font-semibold bg-indigo-950 text-indigo-300 border border-indigo-800/60">
                  Source implementation
                </span>
              </div>
              <p className="text-zinc-300 text-sm leading-relaxed mb-6">
                The repository includes lexer, parser, analyzer, compiler, and UI-renderer sources in AAYU. The current CLI uses a separate Rust bootstrap compiler; a reproducible self-hosting comparison remains unfinished:
              </p>
              <div className="grid grid-cols-1 md:grid-cols-5 gap-3 mb-6">
                <div className="p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                  <div className="text-xs font-mono text-indigo-400 mb-1">01. LEXER</div>
                  <h4 className="font-semibold text-white text-sm">lexer.aayu</h4>
                  <p className="text-xs text-zinc-400 mt-2">LZR Tokenizer with line & column tracking. Skips comments and normalizes tokens.</p>
                </div>
                <div className="p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                  <div className="text-xs font-mono text-indigo-400 mb-1">02. PARSER</div>
                  <h4 className="font-semibold text-white text-sm">parser.aayu</h4>
                  <p className="text-xs text-zinc-400 mt-2">Recursive descent AST generator for Models, Routes, Pages, Actions, Loops.</p>
                </div>
                <div className="p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                  <div className="text-xs font-mono text-indigo-400 mb-1">03. ANALYZER</div>
                  <h4 className="font-semibold text-white text-sm">analyzer.aayu</h4>
                  <p className="text-xs text-zinc-400 mt-2">Scope resolution, symbol table binding, and compile-time invariant checking.</p>
                </div>
                <div className="p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                  <div className="text-xs font-mono text-indigo-400 mb-1">04. EMITTER</div>
                  <h4 className="font-semibold text-white text-sm">compiler.aayu</h4>
                  <p className="text-xs text-zinc-400 mt-2">Variable-length bytecode emission with two-pass jump offset patching.</p>
                </div>
                <div className="p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                  <div className="text-xs font-mono text-indigo-400 mb-1">05. UI ENGINE</div>
                  <h4 className="font-semibold text-white text-sm">ui_renderer.aayu</h4>
                  <p className="text-xs text-zinc-400 mt-2">Transpiles declarative widget trees directly into DOM structures.</p>
                </div>
              </div>
              <div className="p-4 bg-black/60 border border-zinc-800 rounded-xl font-mono text-xs text-zinc-300">
                <span className="text-zinc-500"># Build the Rust bootstrap compiler:</span><br/>
                <span className="text-indigo-400">cargo build --release --manifest-path compiler_bootstrap/Cargo.toml</span>
              </div>
            </div>
          )}

          {activeTab === 'vm' && (
            <div className="bg-zinc-950 border border-zinc-800 rounded-2xl p-6 sm:p-8">
              <div className="flex items-center justify-between mb-6 flex-wrap gap-4">
                <div>
                  <span className="text-xs uppercase font-bold tracking-widest text-cyan-400">Layer 2: Execution Engine</span>
                  <h3 className="text-2xl font-bold text-white mt-1">runtime_rs/ (Silicon Virtual Machine &amp; JIT)</h3>
                </div>
                <span className="px-3 py-1 rounded-full text-xs font-semibold bg-cyan-950 text-cyan-300 border border-cyan-800/60">
                  Rust For Speed Only • NanBox 64-Bit Stack
                </span>
              </div>
              <p className="text-zinc-300 text-sm leading-relaxed mb-6">
                The Rust VM represents all primitives in 64-bit NaN-boxed words (48-bit integer payload in VM mode: [-1.4×10<sup>14</sup>, +1.4×10<sup>14</sup>], full 64-bit hardware registers in JIT mode). Flat-stack base pointer eliminates CallFrame cloning:
              </p>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                <div className="p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                  <div className="flex items-center gap-2 text-cyan-400 text-sm font-semibold mb-2">
                    <Zap className="w-4 h-4" />
                    Flat-Stack Execution
                  </div>
                  <p className="text-xs text-zinc-400 leading-relaxed">
                    Preallocated base pointers replace 2KB CallFrame cloning, eliminating 28 GB of heap allocations and running <code className="text-cyan-300">fib(32)</code> in 0.99s.
                  </p>
                </div>
                <div className="p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                  <div className="flex items-center gap-2 text-cyan-400 text-sm font-semibold mb-2">
                    <Network className="w-4 h-4" />
                    Bounded ThreadPool Server
                  </div>
                  <p className="text-xs text-zinc-400 leading-relaxed">
                    TCP requests are dispatched across a bounded worker pool (32 to 256 threads) with 16,384 queue depth and HTTP 503 backpressure protection.
                  </p>
                </div>
                <div className="p-4 bg-zinc-900/60 border border-zinc-800 rounded-xl">
                  <div className="flex items-center gap-2 text-cyan-400 text-sm font-semibold mb-2">
                    <Database className="w-4 h-4" />
                    4KB Disk-Paged B-Tree DB
                  </div>
                  <p className="text-xs text-zinc-400 leading-relaxed">
                    Slotted 4KB binary disk pages, binary search cell offsets, 256-page LRU cache, and sequential WAL for SSD scalability across millions of records.
                  </p>
                </div>
              </div>
              <div className="p-4 bg-black/60 border border-zinc-800 rounded-xl font-mono text-xs text-zinc-300">
                <span className="text-zinc-500"># Direct bytecode execution:</span><br/>
                <span className="text-cyan-400">.\bin\aayu-vm.exe</span> run tests\test_01_vars.aayu <span className="text-emerald-400"># Compile with nearby aayuc and run in the VM</span>
              </div>
            </div>
          )}

          {activeTab === 'bootstrap' && (
            <div className="bg-zinc-950 border border-zinc-800 rounded-2xl p-6 sm:p-8">
              <div className="flex items-center justify-between mb-6 flex-wrap gap-4">
                <div>
                  <span className="text-xs uppercase font-bold tracking-widest text-emerald-400">Layer 3: Self-Hosting &amp; Silicon VM</span>
                  <h3 className="text-2xl font-bold text-white mt-1">compiler/aayuc.aayu &amp; runtime_rs/</h3>
                </div>
                <span className="px-3 py-1 rounded-full text-xs font-semibold bg-emerald-950 text-emerald-300 border border-emerald-800/60">
                  Pure AAYU Self-Hosted Pipeline
                </span>
              </div>
              <p className="text-zinc-300 text-sm leading-relaxed mb-6">
                The entire compiler pipeline (Lexer, Parser, AST Semantic Analyzer, and Bytecode Emitter) is written in 100% pure AAYU (<code>compiler/aayuc.aayu</code>). The Rust VM (<code>runtime_rs/</code>) provides the high-performance C-level Silicon execution layer:
              </p>
              <div className="space-y-3 mb-6 font-mono text-xs">
                <div className="p-3 bg-zinc-900 border border-zinc-800 rounded-lg flex items-center gap-3">
                  <span className="px-2 py-0.5 rounded bg-emerald-950 text-emerald-400 font-bold">SELF-HOSTED COMPILER</span>
                  <span className="text-zinc-300">.\bin\aayu-vm.exe run compiler/aayuc.aayu</span>
                </div>
                <div className="p-3 bg-zinc-900 border border-zinc-800 rounded-lg flex items-center gap-3">
                  <span className="px-2 py-0.5 rounded bg-emerald-950 text-emerald-400 font-bold">BUILD SILICON VM</span>
                  <span className="text-zinc-300">cargo build --release --manifest-path runtime_rs/Cargo.toml</span>
                </div>
                <div className="p-3 bg-zinc-900 border border-zinc-800 rounded-lg flex items-center gap-3">
                  <span className="px-2 py-0.5 rounded bg-emerald-950 text-emerald-400 font-bold">RUN TEST SUITE</span>
                  <span className="text-zinc-300">.\bin\aayu-vm.exe run tests/test_module_namespaces.aayu</span>
                </div>
              </div>
              <p className="text-xs text-zinc-400 leading-relaxed">
                Zero multi-file boilerplate: all modules, stdlib packages, and user applications compile directly into portable <code>.ayc</code> bytecode executed with microsecond cold starts.
              </p>
            </div>
          )}
        </div>

        {/* Directory Map Table */}
        <div className="mb-20">
          <div className="mb-6">
            <h2 className="text-2xl font-bold text-white">Repository Directory Layout</h2>
            <p className="text-zinc-400 text-sm mt-1">Crystalline organization ensuring zero confusion for contributors and AI agents.</p>
          </div>
          <div className="border border-zinc-800 rounded-2xl overflow-hidden bg-zinc-950">
            <div className="overflow-x-auto">
              <table className="w-full text-left text-sm">
                <thead className="bg-zinc-900/80 border-b border-zinc-800 text-zinc-300 font-mono text-xs uppercase">
                  <tr>
                    <th className="py-3 px-4">Directory</th>
                    <th className="py-3 px-4">Core Technology</th>
                    <th className="py-3 px-4">Architectural Purpose</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-zinc-800/60 font-mono text-xs">
                  {DIRECTORY_TREE.map((item, idx) => (
                    <tr key={idx} className="hover:bg-zinc-900/30 transition-colors">
                      <td className="py-3.5 px-4 font-bold text-indigo-400">{item.dir}</td>
                      <td className="py-3.5 px-4 text-zinc-300">{item.tech}</td>
                      <td className="py-3.5 px-4 text-zinc-400 font-sans text-xs">{item.desc}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>

        {/* NaN-Box Memory Model */}
        <div className="mb-20">
          <div className="mb-6">
            <h2 className="text-2xl font-bold text-white">The 64-Bit NaN-Box Memory Model</h2>
            <p className="text-zinc-400 text-sm mt-1">A tagged value representation for primitives and references to heap objects.</p>
          </div>
          <div className="bg-zinc-950 border border-zinc-800 rounded-2xl p-6 sm:p-8">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <h4 className="font-semibold text-white text-base mb-3">IEEE-754 Quiet NaN Bit Allocation</h4>
                <p className="text-zinc-400 text-xs leading-relaxed mb-4">
                  The VM stores tagged values in a 64-bit word. Primitive values and heap references use distinct tags; see `runtime_rs/src/vm.rs` for the implementation:
                </p>
                <div className="space-y-2 font-mono text-xs">
                  <div className="p-2.5 rounded bg-zinc-900 border border-zinc-800 flex justify-between">
                    <span className="text-zinc-400">Float64</span>
                    <span className="text-cyan-400">val &lt; 0x7FF8_0000_0000_0000</span>
                  </div>
                  <div className="p-2.5 rounded bg-zinc-900 border border-zinc-800 flex justify-between">
                    <span className="text-zinc-400">Int48</span>
                    <span className="text-indigo-400">0x7FF8 | (value & 0xFFFF_FFFF_FFFF)</span>
                  </div>
                  <div className="p-2.5 rounded bg-zinc-900 border border-zinc-800 flex justify-between">
                    <span className="text-zinc-400">Bool</span>
                    <span className="text-emerald-400">0x7FFA | (0 or 1)</span>
                  </div>
                  <div className="p-2.5 rounded bg-zinc-900 border border-zinc-800 flex justify-between">
                    <span className="text-zinc-400">Null</span>
                    <span className="text-zinc-500">0x7FFB | 0</span>
                  </div>
                  <div className="p-2.5 rounded bg-zinc-900 border border-zinc-800 flex justify-between">
                    <span className="text-zinc-400">Heap Ptrs (Str, Arr, Dict)</span>
                    <span className="text-purple-400">0x7FFC..0x7FFE | 48-bit Heap Index</span>
                  </div>
                </div>
              </div>

              <div>
                <h4 className="font-semibold text-white text-base mb-3">Why This Dominates Traditional VMs</h4>
                <ul className="space-y-3 text-xs text-zinc-300">
                  <li className="flex items-start gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400 mt-0.5 shrink-0" />
                    <span><strong>Tagged values:</strong> Primitive values and references are represented within the VM&apos;s 64-bit value format.</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400 mt-0.5 shrink-0" />
                    <span><strong>Heap objects:</strong> Strings, arrays, and dictionaries are tracked by the runtime memory manager.</span>
                  </li>
                  <li className="flex items-start gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400 mt-0.5 shrink-0" />
                    <span><strong>Performance:</strong> Use the checked-in benchmarks for workload-specific measurements; results vary by machine and program.</span>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>

        {/* Bytecode ISA Table */}
        <div className="mb-20">
          <div className="mb-6">
            <h2 className="text-2xl font-bold text-white">Bytecode Instruction Set (ISA)</h2>
            <p className="text-zinc-400 text-sm mt-1">Silicon VM opcode definitions, widths, and stack effects.</p>
          </div>
          <div className="border border-zinc-800 rounded-2xl overflow-hidden bg-zinc-950">
            <div className="overflow-x-auto max-h-96">
              <table className="w-full text-left text-sm">
                <thead className="bg-zinc-900/90 border-b border-zinc-800 text-zinc-300 font-mono text-xs uppercase sticky top-0">
                  <tr>
                    <th className="py-3 px-4">Hex</th>
                    <th className="py-3 px-4">Instruction</th>
                    <th className="py-3 px-4">Width</th>
                    <th className="py-3 px-4">Stack Transition</th>
                    <th className="py-3 px-4">Description</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-zinc-800/60 font-mono text-xs">
                  {OPCODES.map((item, idx) => (
                    <tr key={idx} className="hover:bg-zinc-900/30 transition-colors">
                      <td className="py-3 px-4 text-cyan-400 font-bold">{item.op}</td>
                      <td className="py-3 px-4 text-white font-bold">{item.name}</td>
                      <td className="py-3 px-4 text-zinc-400">{item.width}</td>
                      <td className="py-3 px-4 text-indigo-300">{item.stack}</td>
                      <td className="py-3 px-4 text-zinc-400 font-sans text-xs">{item.desc}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>

        {/* Bottom CTA */}
        <div className="text-center p-12 bg-gradient-to-b from-zinc-900/60 to-black border border-zinc-800 rounded-3xl">
          <h3 className="text-2xl font-bold text-white mb-3">Ready to build on AAYU?</h3>
          <p className="text-zinc-400 text-sm max-w-xl mx-auto mb-6">
            Explore the tutorials, try the browser playground, or inspect the compiler and VM sources in the repository.
          </p>
          <div className="flex flex-wrap justify-center gap-4">
            <Link
              href="/tutorial"
              className="px-6 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-sm transition-colors"
            >
              Start 50-Chapter Masterclass
            </Link>
            <Link
              href="/download"
              className="px-6 py-3 rounded-xl bg-zinc-900 hover:bg-zinc-800 border border-white/10 text-zinc-200 font-semibold text-sm transition-colors"
            >
              Download Binaries (v1.2.0)
            </Link>
          </div>
        </div>

      </div>
    </main>
  );
}
