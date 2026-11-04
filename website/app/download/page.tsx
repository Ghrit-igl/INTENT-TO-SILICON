"use client";
import { useState } from 'react';
import Link from 'next/link';
import { Terminal, Copy, Download, Cpu, Check, Smartphone, BookOpen, ExternalLink, Sparkles, ShieldCheck, Play, ArrowRight } from 'lucide-react';

export default function DownloadPage() {
  const [copiedBash, setCopiedBash] = useState(false);
  const [copiedPs, setCopiedPs] = useState(false);

  const handleCopyBash = () => {
    navigator.clipboard.writeText("curl -fsSL https://intent-to-silicon.vercel.app/install.sh | bash");
    setCopiedBash(true);
    setTimeout(() => setCopiedBash(false), 2000);
  };

  const handleCopyPs = () => {
    navigator.clipboard.writeText("iwr -useb https://intent-to-silicon.vercel.app/install.ps1 -OutFile $env:TEMP\\aayu-install.ps1; & $env:TEMP\\aayu-install.ps1");
    setCopiedPs(true);
    setTimeout(() => setCopiedPs(false), 2000);
  };

  const sdkDownloads = [
    { 
      os: 'Android Mobile (APK)', 
      icon: <Smartphone className="w-8 h-8 text-emerald-400" />, 
      desc: 'Official Android Mobile IDE v1.4.0 · 12.6 MB. Code, compile & execute AAYU bytecode on your phone with cursor lock & symbol strip.', 
      file: 'aayu-android.apk', 
      localUrl: '/downloads/aayu-android.apk', 
      color: 'hover:border-emerald-500/60 border-emerald-500/20 bg-emerald-950/10',
      badge: 'v1.4.0 APK'
    },
    { 
      os: 'Master Handbook (PDF)', 
      icon: <BookOpen className="w-8 h-8 text-purple-400" />, 
      desc: 'Complete 42-Chapter Engineering Specification (4.4 MB · 98 Pages). Verified against 1,002 / 1,002 Silicon VM assertions.', 
      file: 'The_Ultimate_AAYU_Handbook_Ayush_Ghrit_Kaushik.pdf', 
      localUrl: '/aayu-handbook.pdf', 
      color: 'hover:border-purple-500/60 border-purple-500/20 bg-purple-950/10',
      badge: '42 Chapters PDF'
    },
    { 
      os: 'Handbook (Read Online)', 
      icon: <BookOpen className="w-8 h-8 text-amber-300" />, 
      desc: 'Interactive Dark-Mode Web Edition. Read all 42 chapters and system design diagrams instantly in your browser.', 
      file: 'handbook.html', 
      localUrl: '/handbook.html', 
      color: 'hover:border-amber-500/60 border-amber-500/20 bg-amber-950/10',
      badge: 'Interactive HTML',
      isWeb: true
    },
    { 
      os: 'AAYU Mobile IDE (Web)', 
      icon: <Play className="w-8 h-8 text-cyan-400" />, 
      desc: 'Launch the touch-optimized Mobile IDE directly in any browser or install as an Android PWA app.', 
      file: 'mobile-ide', 
      localUrl: '/mobile-ide/index.html', 
      color: 'hover:border-cyan-500/60 border-cyan-500/20 bg-cyan-950/10',
      badge: 'Live Web PWA',
      isWeb: true
    },
    { 
      os: 'Windows (SDK)', 
      icon: <Cpu className="w-8 h-8 text-blue-400" />, 
      desc: 'Native Rust Silicon Engine (aayu.exe & aayuc.exe) · 2.3 MB standalone statically-linked binary.', 
      file: 'aayu-windows.zip', 
      localUrl: '/downloads/aayu-windows.zip', 
      color: 'hover:border-blue-500/50 border-white/5 bg-[#0a0a0a]',
      badge: 'x86_64 Native'
    },
    { 
      os: 'VS Code Extension (.vsix)', 
      icon: <Terminal className="w-8 h-8 text-pink-400" />, 
      desc: 'Official AAYU Pro Language Server with syntax highlighting, autocomplete, and live error diagnostic underlines.', 
      file: 'aayu-vscode.vsix', 
      localUrl: '/downloads/aayu-vscode.vsix', 
      color: 'hover:border-pink-500/50 border-white/5 bg-[#0a0a0a]',
      badge: 'v1.3.0 VSIX'
    },
    { 
      os: 'Linux (SDK)', 
      icon: <Cpu className="w-8 h-8 text-orange-400" />, 
      desc: 'Precompiled ELF binary for Ubuntu, Debian, Arch, and Alpine servers. Zero libc issues.', 
      file: 'aayu-linux.tar.gz', 
      localUrl: 'https://github.com/Ghrit-igl/INTENT-TO-SILICON/releases/latest/download/aayu-linux.tar.gz', 
      color: 'hover:border-orange-500/50 border-white/5 bg-[#0a0a0a]',
      badge: 'Linux ELF'
    },
    { 
      os: 'macOS (SDK)', 
      icon: <Cpu className="w-8 h-8 text-zinc-300" />, 
      desc: 'Universal binary for Apple Silicon (M1/M2/M3/M4) and Intel Macs.', 
      file: 'aayu-macos.zip', 
      localUrl: 'https://github.com/Ghrit-igl/INTENT-TO-SILICON/releases/latest/download/aayu-macos.zip', 
      color: 'hover:border-zinc-500/50 border-white/5 bg-[#0a0a0a]',
      badge: 'macOS Universal'
    },
    { 
      os: 'Mobile IDE Bundle (.zip)', 
      icon: <Smartphone className="w-8 h-8 text-teal-400" />, 
      desc: 'Complete offline Mobile IDE web bundle with local service worker and touch UI assets (150 KB).', 
      file: 'aayu-mobile-ide.zip', 
      localUrl: '/downloads/aayu-mobile-ide.zip', 
      color: 'hover:border-teal-500/50 border-white/5 bg-[#0a0a0a]',
      badge: 'ZIP Bundle'
    }
  ];

  return (
    <main className="min-h-screen bg-[#050505] text-white pt-24 pb-24 selection:bg-purple-500/30">
      <div className="container mx-auto px-4 max-w-6xl">
        
        {/* Header */}
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-purple-500/10 border border-purple-500/30 text-xs font-semibold text-purple-300 mb-4">
            <Sparkles className="w-3.5 h-3.5 text-purple-400" />
            Official Release Hub · 42 Chapters Handbook · Native Android IDE v1.4.0 APK
          </div>
          <h1 className="text-4xl md:text-6xl font-extrabold mb-6 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-white via-zinc-200 to-zinc-500">
            Download <span className="text-purple-400">AAYU</span>
          </h1>
          <p className="text-lg text-zinc-400 max-w-3xl mx-auto leading-relaxed">
            Download the native AAYU Silicon Engine, install the Android Mobile IDE (.apk) on your phone, or read the Official 42-Chapter Complete Engineering Handbook backed by <strong>1,002 / 1,002 passing silicon assertions</strong>.
          </p>
        </div>

        {/* TOP SPOTLIGHT ROW: APK & 42-CHAPTER HANDBOOK */}
        <div className="grid md:grid-cols-2 gap-8 mb-20">
          
          {/* ANDROID APK CARD */}
          <div className="relative rounded-2xl bg-gradient-to-b from-emerald-950/30 to-[#0a0c10] border border-emerald-500/40 p-8 shadow-2xl flex flex-col justify-between group hover:border-emerald-500/70 transition-all">
            <div className="absolute top-4 right-4 px-3 py-1 rounded-full bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 text-xs font-mono font-bold">
              v1.4.0 · 12.6 MB
            </div>
            <div>
              <div className="w-12 h-12 rounded-xl bg-emerald-500/10 border border-emerald-500/30 flex items-center justify-center mb-5">
                <Smartphone className="w-6 h-6 text-emerald-400" />
              </div>
              <h2 className="text-2xl font-bold text-white mb-2 flex items-center gap-2">
                AAYU Mobile IDE for Android
              </h2>
              <p className="text-zinc-300 text-sm mb-4 leading-relaxed">
                Run, edit, and evaluate real AAYU bytecode on your Android device (Android 8.0+). Includes cursor touch-lock bar, auto-pairing brackets, Git snapshot rollback, and sub-millisecond on-device VM execution.
              </p>
              <div className="bg-black/50 border border-emerald-500/20 rounded-xl p-3 text-xs text-zinc-400 mb-6 font-mono space-y-1">
                <div className="text-emerald-400 font-bold">How to Install on Phone:</div>
                <div>1. Click download below to get <code className="text-white">aayu-android.apk</code></div>
                <div>2. Tap notification and allow &quot;Install from unknown sources&quot;</div>
                <div>3. Open AAYU IDE and start compiling on mobile!</div>
              </div>
            </div>

            <div className="flex flex-wrap gap-3 pt-2">
              <a
                href="/downloads/aayu-android.apk"
                download="aayu-android.apk"
                className="inline-flex items-center gap-2 px-6 py-3.5 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500 hover:from-emerald-400 hover:to-teal-400 text-black font-extrabold text-sm shadow-xl shadow-emerald-500/20 transition hover:scale-105"
              >
                <Download className="w-4 h-4 text-black" />
                <span>Download Android APK (12.6 MB)</span>
              </a>
              <Link
                href="/mobile-ide/index.html"
                target="_blank"
                className="inline-flex items-center gap-2 px-5 py-3.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 border border-emerald-500/30 text-emerald-300 font-bold text-sm transition hover:scale-105"
              >
                <Play className="w-4 h-4" />
                <span>Launch Web IDE</span>
                <ExternalLink className="w-3.5 h-3.5 text-zinc-500" />
              </Link>
            </div>
          </div>

          {/* 42-CHAPTER HANDBOOK CARD */}
          <div className="relative rounded-2xl bg-gradient-to-b from-purple-950/30 to-[#0a0c10] border border-purple-500/40 p-8 shadow-2xl flex flex-col justify-between group hover:border-purple-500/70 transition-all">
            <div className="absolute top-4 right-4 px-3 py-1 rounded-full bg-purple-500/20 text-purple-300 border border-purple-500/40 text-xs font-mono font-bold">
              42 Chapters · 4.4 MB
            </div>
            <div>
              <div className="w-12 h-12 rounded-xl bg-purple-500/10 border border-purple-500/30 flex items-center justify-center mb-5">
                <BookOpen className="w-6 h-6 text-purple-400" />
              </div>
              <h2 className="text-2xl font-bold text-white mb-2 flex items-center gap-2">
                Official 42-Chapter Master Handbook
              </h2>
              <p className="text-zinc-300 text-sm mb-4 leading-relaxed">
                The authoritative specification authored by <strong>Ayush Ghrit Kaushik</strong>. 98 high-density pages covering Volume I (Architecture &amp; Foundations) and Volume II (Quantum, Distributed Consensus, ReDoS-Free Regex, P2P DHT &amp; LZW Compression).
              </p>
              <div className="bg-black/50 border border-purple-500/20 rounded-xl p-3 text-xs text-zinc-400 mb-6 font-mono space-y-1">
                <div className="text-purple-400 font-bold">Silicon Verification Seal:</div>
                <div>• Total Chapters: <span className="text-white">42 Complete Chapters</span></div>
                <div>• Passing Assertions: <span className="text-emerald-400 font-bold">1,002 / 1,002 (100% PASS)</span></div>
                <div>• Verification Host: Native <code className="text-white">aayu.exe</code> on Windows 11</div>
              </div>
            </div>

            <div className="flex flex-wrap gap-3 pt-2">
              <a
                href="/aayu-handbook.pdf"
                download="The_Ultimate_AAYU_Handbook_Ayush_Ghrit_Kaushik.pdf"
                className="inline-flex items-center gap-2 px-6 py-3.5 rounded-xl bg-purple-600 hover:bg-purple-500 text-white font-extrabold text-sm shadow-xl shadow-purple-600/20 transition hover:scale-105"
              >
                <Download className="w-4 h-4 text-white" />
                <span>Download Handbook (PDF)</span>
              </a>
              <Link
                href="/handbook.html"
                target="_blank"
                className="inline-flex items-center gap-2 px-5 py-3.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 border border-purple-500/30 text-amber-300 font-bold text-sm transition hover:scale-105"
              >
                <BookOpen className="w-4 h-4" />
                <span>Read Online (HTML)</span>
                <ExternalLink className="w-3.5 h-3.5 text-zinc-500" />
              </Link>
            </div>
          </div>

        </div>

        {/* ALL RELEASES GRID */}
        <div className="mb-20">
          <div className="flex items-center justify-between mb-4">
            <div>
              <h2 className="text-2xl font-bold flex items-center gap-2">
                <Terminal className="w-6 h-6 text-cyan-400" /> Complete Release Catalog &amp; Platform SDKs
              </h2>
              <p className="text-zinc-400 text-sm mt-1">Statically compiled binaries and platform bundles with zero runtime overhead:</p>
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
            {sdkDownloads.map((d, i) => (
              <div key={i} className={"rounded-xl p-6 transition-all duration-300 flex flex-col justify-between border " + d.color}>
                <div>
                  <div className="flex justify-between items-start mb-4">
                    {d.icon}
                    <span className="text-[11px] font-mono px-2 py-0.5 rounded bg-white/5 border border-white/10 text-zinc-300">
                      {d.badge}
                    </span>
                  </div>
                  <h3 className="text-lg font-bold mb-1 text-white">{d.os}</h3>
                  <p className="text-xs text-zinc-400 leading-relaxed mb-6">{d.desc}</p>
                </div>

                <div className="pt-2 border-t border-white/5 flex items-center justify-between">
                  <span className="text-xs font-mono text-zinc-500">{d.file}</span>
                  {d.isWeb ? (
                    <Link 
                      href={d.localUrl} 
                      target="_blank"
                      className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-white/10 hover:bg-white/20 text-white text-xs font-bold transition-colors"
                    >
                      <span>Open</span>
                      <ExternalLink className="w-3 h-3" />
                    </Link>
                  ) : (
                    <a 
                      href={d.localUrl} 
                      download={d.file}
                      className="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-white/10 hover:bg-white/20 text-white text-xs font-bold transition-colors"
                    >
                      <span>Download</span>
                      <Download className="w-3 h-3" />
                    </a>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* QUICK CLI INSTALL SCRIPT BLOCK */}
        <div className="border-t border-white/10 pt-16">
          <h2 className="text-2xl font-bold mb-6 text-center">Quick Terminal Installation (Automated)</h2>
          
          <div className="grid md:grid-cols-2 gap-6 max-w-4xl mx-auto">
            {/* macOS / Linux */}
            <div className="bg-[#0a0a0a] border border-white/10 rounded-xl p-6 flex flex-col justify-between">
              <div>
                <div className="flex justify-between items-center mb-3">
                  <span className="text-xs font-mono text-cyan-400 uppercase tracking-wider font-bold">macOS &amp; Linux Terminal</span>
                  <button onClick={handleCopyBash} className="text-zinc-400 hover:text-white flex items-center gap-1.5 text-xs transition-colors">
                    {copiedBash ? <><Check className="w-3.5 h-3.5 text-green-400" /> Copied!</> : <><Copy className="w-3.5 h-3.5" /> Copy</>}
                  </button>
                </div>
                <pre className="text-zinc-300 font-mono text-xs overflow-x-auto p-3 bg-black/60 rounded-lg border border-white/5 whitespace-pre-wrap">
                  <code className="text-purple-300">curl</code> -fsSL https://intent-to-silicon.vercel.app/install.sh | bash
                </pre>
              </div>
              <p className="text-[11px] text-zinc-500 mt-4">Downloads release binary, validates SHA-256 hash, and links to <code className="text-zinc-400">/usr/local/bin/aayu</code>.</p>
            </div>

            {/* Windows PowerShell */}
            <div className="bg-[#0a0a0a] border border-white/10 rounded-xl p-6 flex flex-col justify-between">
              <div>
                <div className="flex justify-between items-center mb-3">
                  <span className="text-xs font-mono text-blue-400 uppercase tracking-wider font-bold">Windows PowerShell (Admin)</span>
                  <button onClick={handleCopyPs} className="text-zinc-400 hover:text-white flex items-center gap-1.5 text-xs transition-colors">
                    {copiedPs ? <><Check className="w-3.5 h-3.5 text-green-400" /> Copied!</> : <><Copy className="w-3.5 h-3.5" /> Copy</>}
                  </button>
                </div>
                <pre className="text-zinc-300 font-mono text-xs overflow-x-auto p-3 bg-black/60 rounded-lg border border-white/5 whitespace-pre-wrap">
                  <code className="text-purple-300">iwr</code> -useb https://intent-to-silicon.vercel.app/install.ps1 | iex
                </pre>
              </div>
              <p className="text-[11px] text-zinc-500 mt-4">Sets up <code className="text-zinc-400">aayu.exe</code>, configures User PATH, and registers <code className="text-zinc-400">.aayu</code> file association.</p>
            </div>
          </div>
        </div>

      </div>
    </main>
  );
}
