"use client";

import Link from "next/link";
import Image from "next/image";
import { GitBranch, Menu, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useEffect, useState } from "react";

export function Navbar() {
  const [scrolled, setScrolled] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      setScrolled(window.scrollY > 20);
    };
    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  return (
    <header className={`fixed top-0 z-50 w-full transition-all duration-300 ${scrolled || mobileMenuOpen ? "border-b border-white/10 bg-black/95 backdrop-blur-md" : "bg-transparent"}`}>
      <div className="container mx-auto flex h-16 items-center justify-between px-4 max-w-7xl">
        <div className="flex items-center gap-8">
          <Link href="/" className="flex items-center space-x-3 group">
            <Image src="/aayu-logo.png" alt="AAYU Logo" width={32} height={32} className="object-contain group-hover:scale-110 transition-transform" />
            <span className="font-bold text-xl tracking-tight text-white">AAYU</span>
          </Link>
          <nav className="hidden lg:flex gap-5 text-sm font-medium text-zinc-400 items-center">
            <Link href="/" className="hover:text-white transition-colors">Home</Link>
            <Link href="/docs" className="hover:text-white transition-colors">Docs</Link>
            <Link href="/tutorial" className="hover:text-white transition-colors text-purple-400 font-bold">Tutorial</Link>
            <Link href="/handbook.html" target="_blank" className="hover:text-white transition-colors text-amber-300 font-bold flex items-center gap-1">Handbook (42 Ch)</Link>
            <Link href="/mobile-ide/index.html" target="_blank" className="hover:text-white transition-colors text-emerald-400 font-bold flex items-center gap-1">Mobile IDE</Link>
            <Link href="/reports" className="hover:text-white transition-colors text-cyan-400 font-bold">Benchmarks</Link>
            <Link href="/download" className="hover:text-white transition-colors flex items-center gap-1.5 bg-white/10 hover:bg-white/15 px-3 py-1.5 rounded-lg border border-white/10 text-white font-semibold">
              <span className="relative flex h-2 w-2 mr-1">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
              </span>
              Download (APK · CLI)
            </Link>
          </nav>
        </div>
        <div className="flex items-center gap-4">
          <Link href="https://github.com/Ghrit-igl/INTENT-TO-SILICON" target="_blank" rel="noreferrer" className="hidden sm:flex">
            <div className="flex h-9 w-9 items-center justify-center rounded-md border border-white/10 hover:bg-white/10 transition-colors">
              <GitBranch className="h-4 w-4 text-white" />
              <span className="sr-only">GitHub</span>
            </div>
          </Link>
          <Link href="https://github.com/Ghrit-igl/INTENT-TO-SILICON" target="_blank" className="hidden sm:flex">
            <Button className="bg-white text-black hover:bg-zinc-200 font-semibold shadow-[0_0_15px_rgba(255,255,255,0.3)]">
              View on GitHub
            </Button>
          </Link>
          <button
            className="lg:hidden text-zinc-400 hover:text-white p-2"
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
          >
            {mobileMenuOpen ? <X className="h-6 w-6" /> : <Menu className="h-6 w-6" />}
          </button>
        </div>
      </div>

      {/* Mobile Menu */}
      {mobileMenuOpen && (
        <div className="lg:hidden bg-black/95 border-b border-white/10 px-4 py-6 flex flex-col gap-5 text-base font-medium">
          <Link href="/" onClick={() => setMobileMenuOpen(false)} className="text-zinc-400 hover:text-white transition-colors block">Home</Link>
          <Link href="/docs" onClick={() => setMobileMenuOpen(false)} className="text-zinc-400 hover:text-white transition-colors block">Docs</Link>
          <Link href="/tutorial" onClick={() => setMobileMenuOpen(false)} className="text-purple-400 font-bold hover:text-purple-300 transition-colors block">Tutorial</Link>
          <Link href="/handbook.html" target="_blank" onClick={() => setMobileMenuOpen(false)} className="text-amber-300 font-bold hover:text-amber-200 transition-colors block">Master Handbook (42 Chapters HTML)</Link>
          <Link href="/mobile-ide/index.html" target="_blank" onClick={() => setMobileMenuOpen(false)} className="text-emerald-400 font-bold hover:text-emerald-300 transition-colors block">AAYU Mobile IDE (Web / PWA)</Link>
          <Link href="/reports" onClick={() => setMobileMenuOpen(false)} className="text-cyan-400 font-bold hover:text-cyan-300 transition-colors block">Benchmarks &amp; Proofs</Link>
          <Link href="/examples" onClick={() => setMobileMenuOpen(false)} className="text-zinc-400 hover:text-white transition-colors block">Examples</Link>
          <Link href="/download" onClick={() => setMobileMenuOpen(false)} className="text-emerald-400 font-bold hover:text-emerald-300 transition-colors flex items-center gap-2">
            <span className="relative flex h-2 w-2 mr-1">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
            </span>
            Download (Android APK &amp; Native CLI)
          </Link>
          <div className="pt-4 border-t border-zinc-800 flex flex-col gap-4">
            <Link href="https://github.com/Ghrit-igl/INTENT-TO-SILICON" target="_blank" className="flex items-center gap-2 text-zinc-400 hover:text-white">
              <GitBranch className="h-5 w-5" />
              GitHub Repository
            </Link>
          </div>
        </div>
      )}
    </header>
  );
}
