import React from 'react';
import { Code2, Zap, ShieldCheck, Terminal, BookOpen, Smartphone, Cpu, CheckCircle2 } from 'lucide-react';
import Link from 'next/link';
import { Button } from '@/components/ui/button';

export default function LanguagePage() {
  return (
    <main className="min-h-screen bg-[#050505] text-white pt-28 pb-20 selection:bg-purple-500/30">
      <div className="container mx-auto px-4 max-w-5xl">
        
        {/* Header */}
        <div className="mb-14">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-300 text-xs font-mono mb-4">
            <Code2 className="w-3.5 h-3.5" /> AAYU Language Specification v1.5.0
          </div>
          <h1 className="text-4xl md:text-6xl font-extrabold tracking-tight mb-4">
            The AAYU Language
          </h1>
          <p className="text-lg md:text-xl text-zinc-400 max-w-3xl leading-relaxed">
            A self-hosted, intent-first programming language with built-in database models, ASGI web routing, reactive UI widgets, and zero external runtime dependencies.
          </p>
        </div>

        {/* 4 Core Tenets Cards */}
        <div className="grid md:grid-cols-3 gap-5 mb-16">
          <div className="bg-[#0a0a0d] border border-white/10 rounded-2xl p-6">
            <ShieldCheck className="w-7 h-7 text-emerald-400 mb-3" />
            <h3 className="text-lg font-bold text-white mb-2">Self-Hosted Ecosystem</h3>
            <p className="text-zinc-400 text-xs leading-relaxed">
              The Lexer, Parser, AST Analyzer, and Standard Libraries are written in 100% pure AAYU (<code className="text-zinc-300">.aayu</code>).
            </p>
          </div>
          <div className="bg-[#0a0a0d] border border-white/10 rounded-2xl p-6">
            <Zap className="w-7 h-7 text-yellow-400 mb-3" />
            <h3 className="text-lg font-bold text-white mb-2">Silicon Stack VM</h3>
            <p className="text-zinc-400 text-xs leading-relaxed">
              A high-performance stack VM written strictly in Rust executes bytecode instructions with C-level speed and zero runtime overhead.
            </p>
          </div>
          <div className="bg-[#0a0a0d] border border-white/10 rounded-2xl p-6">
            <Smartphone className="w-7 h-7 text-cyan-400 mb-3" />
            <h3 className="text-lg font-bold text-white mb-2">Mobile &amp; Termux Ready</h3>
            <p className="text-zinc-400 text-xs leading-relaxed">
              Built statically with zero dependencies. Runs effortlessly on desktop Windows/Linux and Android Termux smartphones.
            </p>
          </div>
        </div>

        {/* SECTION 1: TYPES & MEMORY */}
        <section className="mb-16">
          <h2 className="text-2xl font-bold mb-4 border-b border-white/10 pb-3 flex items-center gap-2">
            <span className="text-purple-400 font-mono">01.</span> Values, Types &amp; Memory Architecture
          </h2>
          <p className="text-zinc-400 text-sm mb-6 leading-relaxed">
            AAYU uses dynamic typing backed by an in-memory NaN-boxing union layout. Primitive values (integers, floats, booleans) fit in a single 64-bit register without heap allocation. When integer arithmetic exceeds the signed 48-bit register boundary, the VM automatically promotes values to 128-bit heap BigInts without silent wrapping or precision loss. Collections and strings allocate on the managed mark-and-sweep heap.
          </p>
          <div className="bg-[#0a0a0f] border border-white/10 rounded-xl overflow-hidden mb-4">
            <div className="bg-[#111] px-4 py-2 border-b border-white/5 text-xs text-zinc-500 font-mono">types.aayu</div>
            <pre className="p-5 font-mono text-xs md:text-sm text-zinc-300 leading-relaxed overflow-x-auto">
{`let count = 42              // Int (48-bit inline register)
let price = 19.99           // Float
let is_active = 1           // Bool (1 / 0)
let title = "Silicon Core"  // String
let empty = null            // Null
let numbers = [1, 2, 3]     // Array
let config = {"debug": 0}   // Dict

// 128-bit BigInt Auto-Promotion (Zero Wrapping)
let large_int = 140737488355327
let safe_sum = large_int + 1        // 140737488355328 (Auto-promoted to 128-bit BigInt)

// Runtime Type Reflection
print("count is:", type(count))        // "Int"
print("safe_sum is:", type(safe_sum))  // "Int"
print("title is:", type(title))        // "String"
print("numbers is:", type(numbers))    // "Array"`}
            </pre>
          </div>
        </section>

        {/* SECTION 2: MULTI-LINE STRINGS */}
        <section className="mb-16">
          <h2 className="text-2xl font-bold mb-4 border-b border-white/10 pb-3 flex items-center gap-2">
            <span className="text-purple-400 font-mono">02.</span> Multi-Line Triple Strings
          </h2>
          <p className="text-zinc-400 text-sm mb-6 leading-relaxed">
            AAYU natively supports Python-style multi-line string literals enclosed in triple-quotes (<code className="text-purple-300">&quot;&quot;&quot;...&quot;&quot;&quot;</code>). Newlines, tabs, and indentation inside the block are preserved verbatim.
          </p>
          <div className="bg-[#0a0a0f] border border-white/10 rounded-xl overflow-hidden mb-4">
            <div className="bg-[#111] px-4 py-2 border-b border-white/5 text-xs text-zinc-500 font-mono">multiline.aayu</div>
            <pre className="p-5 font-mono text-xs md:text-sm text-zinc-300 leading-relaxed overflow-x-auto">
{`let poetry = """Twinkle, twinkle, little star,
How I wonder what you are!
Up above the world so high,
Like a diamond in the sky."""

print(poetry)`}
            </pre>
          </div>
        </section>

        {/* SECTION 3: CONTROL FLOW */}
        <section className="mb-16">
          <h2 className="text-2xl font-bold mb-4 border-b border-white/10 pb-3 flex items-center gap-2">
            <span className="text-purple-400 font-mono">03.</span> Conditionals &amp; Loops
          </h2>
          <p className="text-zinc-400 text-sm mb-6 leading-relaxed">
            Clean, keyword-delimited blocks without parentheses or braces. Every control block terminates with <code className="text-purple-300">end</code>.
          </p>
          <div className="bg-[#0a0a0f] border border-white/10 rounded-xl overflow-hidden mb-4">
            <div className="bg-[#111] px-4 py-2 border-b border-white/5 text-xs text-zinc-500 font-mono">control_flow.aayu</div>
            <pre className="p-5 font-mono text-xs md:text-sm text-zinc-300 leading-relaxed overflow-x-auto">
{`// 1. Conditionals (if / elif / else / end)
let score = 88

if score >= 90
    print("Grade: A")
elif score >= 80
    print("Grade: B")
else
    print("Grade: C")
end

// 2. Iteration (for in collection)
for item in ["Silicon", "Bytecode", "Intent"]
    print("Processing:", item)
end

// 3. Loops (while condition)
let i = 0
while i < 5
    print("Step:", i)
    i = i + 1
end`}
            </pre>
          </div>
        </section>

        {/* SECTION 4: ACTIONS & CALL FRAMES */}
        <section className="mb-16">
          <h2 className="text-2xl font-bold mb-4 border-b border-white/10 pb-3 flex items-center gap-2">
            <span className="text-purple-400 font-mono">04.</span> Actions (Functions) &amp; Execution
          </h2>
          <p className="text-zinc-400 text-sm mb-6 leading-relaxed">
            Functions in AAYU are declared with the <code className="text-purple-300">action</code> keyword and invoked with standard call notation. Top-level program entrypoints are triggered with <code className="text-purple-300">run &lt;action&gt;</code>.
          </p>
          <div className="bg-[#0a0a0f] border border-white/10 rounded-xl overflow-hidden mb-4">
            <div className="bg-[#111] px-4 py-2 border-b border-white/5 text-xs text-zinc-500 font-mono">actions.aayu</div>
            <pre className="p-5 font-mono text-xs md:text-sm text-zinc-300 leading-relaxed overflow-x-auto">
{`action calculate_tax(amount, rate)
    let tax = amount * rate
    return tax
end

action main
    let total = calculate_tax(100.0, 0.18)
    print("Total Tax:", total) # 18.0
end

run main`}
            </pre>
          </div>
        </section>

        {/* SECTION 5: PURE STANDARD LIBRARIES */}
        <section className="mb-16">
          <h2 className="text-2xl font-bold mb-4 border-b border-white/10 pb-3 flex items-center gap-2">
            <span className="text-purple-400 font-mono">05.</span> Standard Libraries (<code className="text-purple-300">std/</code>)
          </h2>
          <p className="text-zinc-400 text-sm mb-6 leading-relaxed">
            AAYU includes built-in standard modules written in pure AAYU. No external package manager or pip/npm downloads are required.
          </p>
          <div className="grid md:grid-cols-2 gap-4 mb-4">
            <div className="bg-[#0a0a0f] border border-white/10 rounded-xl p-5">
              <h4 className="text-emerald-400 font-bold text-sm mb-2 font-mono">std/collections.aayu</h4>
              <p className="text-zinc-400 text-xs mb-3">21 functional helpers: sum, min, max, mean, contains, index_of, slice, concat, reverse, range, fill, flatten, unique, zip, take, drop.</p>
              <pre className="text-[11px] font-mono text-zinc-300 bg-black/60 p-3 rounded-lg overflow-x-auto">
{`import "std/collections"

let vals = [10, 20, 30]
print("Sum:", sum(vals))       # 60
print("Mean:", mean(vals))     # 20
let r = range(1, 5, 1)         # [1, 2, 3, 4]`}
              </pre>
            </div>

            <div className="bg-[#0a0a0f] border border-white/10 rounded-xl p-5">
              <h4 className="text-cyan-400 font-bold text-sm mb-2 font-mono">std/fs.aayu &amp; std/os.aayu</h4>
              <p className="text-zinc-400 text-xs mb-3">Native file system and operating system query functions.</p>
              <pre className="text-[11px] font-mono text-zinc-300 bg-black/60 p-3 rounded-lg overflow-x-auto">
{`import "std/fs"
import "std/os"

write_file("data.log", "OK")
let text = read_file("data.log")
print("OS:", os_platform())`}
              </pre>
            </div>
          </div>
        </section>

        {/* Handbook Download Banner */}
        <div className="p-8 rounded-2xl bg-gradient-to-r from-purple-900/30 via-indigo-900/20 to-black border border-purple-500/40 flex flex-col md:flex-row items-center justify-between gap-6">
          <div>
            <h3 className="text-xl font-bold text-white mb-2">Read the Full 21-Chapter Handbook</h3>
            <p className="text-zinc-400 text-sm max-w-xl">
              Deep dive into compiler AST trees, bytecode ISA opcodes, SQLite storage engine, and master algorithms in the official 1.8 MB PDF.
            </p>
          </div>
          <a
            href="/aayu-handbook.pdf"
            download="The_Ultimate_AAYU_Handbook_Ayush_Ghrit_Kaushik.pdf"
            className="px-6 py-3 rounded-xl bg-purple-600 hover:bg-purple-500 text-white font-bold text-sm shrink-0 shadow-lg shadow-purple-600/30 transition hover:scale-105"
          >
            Download Handbook PDF
          </a>
        </div>

      </div>
    </main>
  );
}
