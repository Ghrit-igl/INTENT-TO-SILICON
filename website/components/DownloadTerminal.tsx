"use client";
import React, { useState } from "react";

export default function DownloadTerminal() {
  const [os] = useState(() => {
    if (typeof window !== "undefined") {
      const userAgent = window.navigator.userAgent.toLowerCase();
      if (userAgent.includes("android") || userAgent.includes("iphone") || userAgent.includes("ipad")) {
        return "mobile";
      } else if (userAgent.includes("mac")) {
        return "mac";
      } else if (userAgent.includes("linux")) {
        return "linux";
      }
    }
    return "windows";
  });

  if (os === "mobile") {
    return (
      <div className="bg-black border border-white/10 rounded-xl p-6 max-w-lg mx-auto text-left font-mono text-sm mb-10 space-y-3">
        <div className="flex items-center gap-3">
          <span className="text-zinc-600 select-none">$</span>
          <span className="text-zinc-400"># 1. Mobile Device Detected!</span>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-zinc-600 select-none">$</span>
          <span className="text-blue-400"># AAYU requires a Desktop OS to compile natively.</span>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-zinc-600 select-none">$</span>
          <span className="text-green-400"># Please visit this site on Windows, Mac, or Linux!</span>
        </div>
        <div className="border-t border-white/5 pt-3 mt-3">
          <span className="text-zinc-500">Note: You can run AAYU on Android via Termux + Python.</span>
        </div>
      </div>
    );
  }

  const isWindows = os === "windows";

  return (
    <div className="bg-black border border-white/10 rounded-xl p-6 max-w-lg mx-auto text-left font-mono text-sm mb-10 space-y-3">
      <div className="flex items-center gap-3">
        <span className="text-zinc-600 select-none">$</span>
        <span className="text-zinc-400"># 1. Install AAYU Globally for {os.toUpperCase()}</span>
      </div>
      <div className="flex items-center gap-3">
        <span className="text-zinc-600 select-none">$</span>
        {isWindows ? (
            <span className="text-green-400 break-all">irm https://intent-to-silicon.vercel.app/install.ps1 | iex</span>
        ) : (
            <span className="text-green-400 break-all">curl -sSL https://intent-to-silicon.vercel.app/install.sh | bash</span>
        )}
      </div>
      <div className="flex items-center gap-3">
        <span className="text-zinc-600 select-none">$</span>
        <span className="text-blue-400">aayu run main.aayu</span>
      </div>
      <div className="border-t border-white/5 pt-3 mt-3">
        <span className="text-emerald-400">⚡ Compiled to Native machine code in 12ms</span>
      </div>
      <div>
        <span className="text-emerald-400">✨ Execution output: Hello from AAYU</span>
      </div>
    </div>
  );
}
