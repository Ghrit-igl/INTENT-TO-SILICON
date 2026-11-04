import Link from 'next/link';
import { BookOpen, Download, FileText, Sparkles, ExternalLink } from 'lucide-react';
import { chapters } from '@/lib/tutorial-data';

export default function TutorialPage() {
  return (
    <main className="min-h-screen bg-black px-4 pb-24 pt-32 text-white">
      <div className="mx-auto max-w-6xl">
        <header className="mb-10 max-w-3xl">
          <p className="mb-3 text-sm font-semibold uppercase tracking-widest text-cyan-300">Verified core course · 6 lessons</p>
          <h1 className="mb-5 text-5xl font-extrabold tracking-tight md:text-7xl">Learn AAYU by running it</h1>
          <p className="text-lg leading-8 text-zinc-400">
            A beginner path through the language subset exercised by the repository regressions: values, arithmetic, loops, conditions, and a small complete program. Each lesson links to checked-in source.
          </p>
        </header>

        {/* Master Handbook PDF Download Banner */}
        <div className="mb-12 flex flex-col md:flex-row items-start md:items-center justify-between gap-6 rounded-2xl border border-indigo-500/40 bg-gradient-to-r from-indigo-950/70 via-purple-950/50 to-zinc-950 p-6 md:p-8 shadow-2xl">
          <div className="max-w-2xl">
            <div className="inline-flex items-center gap-2 rounded-full border border-indigo-400/30 bg-indigo-500/20 px-3 py-1 text-xs font-bold text-indigo-300 uppercase tracking-widest mb-3">
              <Sparkles className="w-3.5 h-3.5 text-indigo-300" />
              Official 2026 Master Handbook PDF · 42 Chapters
            </div>
            <h2 className="text-2xl md:text-3xl font-extrabold text-white flex items-center gap-3">
              <BookOpen className="w-7 h-7 text-indigo-400" />
              The Ultimate AAYU Master Handbook
            </h2>
            <p className="mt-2 text-sm leading-6 text-zinc-300">
              The complete engineering reference by <strong>Ayush Ghrit Kaushik</strong>. 42 comprehensive chapters covering Language Fundamentals, Memory Architecture (Union-Style NaN-Boxing &amp; Stack-Style Execution), Native Loops, DSA, System Design, Quantum, Raft, ReDoS-Free Regex, P2P DHT, and Lossless Compression with <strong>1,002 / 1,002 verified silicon assertions</strong>.
            </p>
          </div>
          <div className="flex flex-col sm:flex-row items-center gap-3 shrink-0">
            <a
              href="/aayu-handbook.pdf"
              download="The_Ultimate_AAYU_Handbook_Ayush_Ghrit_Kaushik.pdf"
              className="inline-flex items-center justify-center gap-3 rounded-xl bg-gradient-to-r from-indigo-500 to-purple-600 px-6 py-4 text-sm font-bold text-white shadow-lg shadow-indigo-500/25 transition hover:scale-105 hover:from-indigo-400 hover:to-purple-500 whitespace-nowrap"
            >
              <Download className="w-4 h-4 text-white" />
              <span>Download Master Handbook</span>
              <span className="rounded bg-black/30 px-2 py-0.5 text-xs text-indigo-200">4.4 MB · 42 Ch</span>
            </a>
            <Link
              href="/handbook.html"
              target="_blank"
              className="inline-flex items-center justify-center gap-2 rounded-xl bg-zinc-900 hover:bg-zinc-800 border border-indigo-500/40 px-5 py-4 text-sm font-bold text-amber-300 transition hover:scale-105"
            >
              <BookOpen className="w-4 h-4 text-amber-400" />
              <span>Read Online (HTML)</span>
              <ExternalLink className="w-3.5 h-3.5 text-zinc-500" />
            </Link>
          </div>
        </div>

        <div className="mb-10 rounded-xl border border-amber-500/30 bg-amber-500/5 p-5 text-sm leading-6 text-amber-100">
          This course covers the verified core subset. HTTP/SQL apps, functions, async, AI/ML, UI interaction, and package publishing are not taught as working AAYU features here. See <Link className="underline" href="/docs/1-getting-started/1-installation">installation</Link> and the repository <Link className="underline" href="https://github.com/Ghrit-igl/INTENT-TO-SILICON/blob/main/KNOWN_LIMITATIONS.md">known limitations</Link>.
        </div>

        <section aria-label="Course lessons" className="grid gap-5 md:grid-cols-2 lg:grid-cols-3">
          {chapters.map((chapter) => (
            <Link key={chapter.slug} href={`/tutorial/${chapter.slug}`} className="group rounded-xl border border-zinc-800 bg-zinc-950 p-6 transition hover:border-cyan-500/40 hover:bg-zinc-900">
              <div className="mb-4 text-sm font-mono text-cyan-300">LESSON {String(chapter.num).padStart(2, '0')} · VERIFIED SOURCE</div>
              <h2 className="mb-3 text-xl font-bold group-hover:text-cyan-100">{chapter.title}</h2>
              <p className="text-sm leading-6 text-zinc-400">{chapter.description}</p>
              <p className="mt-5 text-sm text-cyan-300">Open lesson →</p>
            </Link>
          ))}
        </section>
      </div>
    </main>
  );
}
