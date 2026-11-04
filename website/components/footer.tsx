/* eslint-disable */

import Link from "next/link";
import Image from "next/image";

export function Footer() {
  return (
    <footer className="border-t border-white/10 bg-black py-16 md:py-24">
      <div className="container mx-auto px-4 max-w-7xl">
        <div className="grid gap-12 md:grid-cols-4 lg:grid-cols-5">
          <div className="lg:col-span-2">
            <Link href="/" className="flex items-center space-x-3 mb-6">
              <Image src="/aayu-logo.png" alt="AAYU Logo" width={32} height={32} />
              <span className="font-bold text-xl tracking-tight text-white">AAYU</span>
            </Link>
            <p className="text-zinc-400 max-w-xs leading-relaxed mb-6">
              The Application Language. Write apps, not configurations.
            </p>
            <div className="flex gap-4">
              <Link href="https://github.com/Ghrit-igl/INTENT-TO-SILICON" target="_blank" rel="noopener noreferrer" className="text-zinc-400 hover:text-white transition-colors">GitHub</Link>
              
              
            </div>
          </div>
          <div>
            <h3 className="mb-4 text-sm font-semibold text-white tracking-wider uppercase">Platform</h3>
            <ul className="space-y-3 text-sm text-zinc-400">
              <li><Link href="/language" className="hover:text-blue-400 transition-colors">AAYU Language</Link></li>
              <li><Link href="/architecture" className="hover:text-blue-400 transition-colors">Runtime Architecture</Link></li>
              <li><Link href="/docs/cli" className="hover:text-blue-400 transition-colors">CLI &amp; Tools</Link></li>
              <li><Link href="/playground" className="hover:text-white transition-colors">Playground</Link></li>
              <li><Link href="/download" className="hover:text-white transition-colors">Download</Link></li>
            </ul>
          </div>
          <div>
            <h3 className="mb-4 text-sm font-semibold text-white tracking-wider uppercase">Resources</h3>
            <ul className="space-y-3 text-sm text-zinc-400">
              <li><Link href="/docs" className="hover:text-white transition-colors">Documentation</Link></li>
              <li><Link href="/examples" className="hover:text-white transition-colors">Examples Gallery</Link></li>
              <li><Link href="/packages" className="hover:text-white transition-colors">Packages</Link></li>
              <li><Link href="/blog" className="hover:text-white transition-colors">Blog</Link></li>
              <li><Link href="/roadmap" className="hover:text-white transition-colors">Roadmap</Link></li>
            </ul>
          </div>
          <div>
            <h3 className="mb-4 text-sm font-semibold text-white tracking-wider uppercase">Company</h3>
            <ul className="space-y-3 text-sm text-zinc-400">
              <li><Link href="/about" className="hover:text-white transition-colors">About</Link></li>
              <li><Link href="https://github.com/Ghrit-igl/INTENT-TO-SILICON" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">GitHub</Link></li>
              <li><Link href="/privacy" className="hover:text-white transition-colors">Privacy Policy</Link></li>
              <li><Link href="/terms" className="hover:text-white transition-colors">Terms of Service</Link></li>
            </ul>
          </div>
        </div>
        <div className="mt-16 flex flex-col items-center justify-between gap-4 border-t border-white/10 pt-8 text-sm text-zinc-500 md:flex-row">
          <p>© {new Date().getFullYear()} AAYU Foundation. All rights reserved.</p>
          <div className="flex gap-4">
            <span className="flex items-center gap-1">Status: <span className="text-green-500">All systems operational</span></span>
          </div>
        </div>
      </div>
    </footer>
  );
}
