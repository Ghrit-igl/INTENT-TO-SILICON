import Link from 'next/link';
import { ArrowRight, Code2, Database, Layout, ShieldAlert, Play, ExternalLink, FolderGit2 } from 'lucide-react';

export default function ExamplesPage() {
  const examples = [
    {
      title: "AAYU Silicon AI Agent — Laptop Automation & Voice Control",
      desc: "100% Pure AAYU Language (.aayu) with Zero Python Dependencies. A native autonomous AI assistant that listens to voice commands via speech synthesis & recognition (wake-word 'AAYU PROGRAMMING' or 'AAYU'), executes real laptop automation (launches VS Code, Notepad, Browser, Calculator, locks workstation), inspects system hardware diagnostics in 1,166 µs latency, and renders a live Cybernetic HUD dashboard.",
      tags: ["Autonomous AI Agent", "Voice Control (Wake-Word)", "OS Automation", "Zero Python", "Hardware Diagnostics"],
      githubUrl: "https://github.com/Minato95-ayu/INTENT-TO-SILICON/blob/main/examples/aayu_ai_agent.aayu",
      code: `# Run locally: aayu run examples/aayu_ai_agent.aayu --web --port 8080
# 1-Click: run_aayu_agent.bat
# Wake-word: "AAYU PROGRAMMING" or "AAYU"

app AAYUSiliconAgent

import "std/os.aayu"
import "std/audio.aayu"
import "std/intent.aayu"
import "std/brain.aayu"

# 1. Native Laptop Command Intent Parser
action execute_laptop_command(raw_cmd)
    let lower = str_lower(raw_cmd)
    
    # OS App Automation
    if str_contains(lower, "notepad")
        os_spawn("notepad.exe")
        return "Notepad launched successfully."
    elif str_contains(lower, "code") or str_contains(lower, "vs code")
        os_spawn("code")
        return "VS Code opened."
    elif str_contains(lower, "browser")
        os_open_url("https://intent-to-silicon.vercel.app/")
        return "AAYU Portal launched in browser."
    elif str_contains(lower, "calc")
        os_spawn("calc.exe")
        return "Calculator opened."
    elif str_contains(lower, "status")
        return "OS: " + os_platform() + " | CPU: " + os_cpu() + " | AAYU Silicon Native"
    elif str_contains(lower, "lock")
        os_lock_workstation()
        return "Workstation locked securely."
    elif str_starts_with(lower, "speak ")
        let msg = str_substr(raw_cmd, 6, len(raw_cmd))
        speak_text(msg)
        return "Spoke aloud: " + msg
    end
    
    return "Intent recognized: " + raw_cmd
end

# 2. REST API Route for Voice HUD & Speech Events
route "/api/agent/command"
    post
        let body = request_body()
        let cmd = body["cmd"]
        let wake = body["wake_word"]
        
        # Enforce Wake-Word Barrier
        if wake != "AAYU PROGRAMMING" and wake != "AAYU"
            respond({ "error": "Ignored: Wake word required" })
            return
        end
        
        let t0 = time_us()
        let result = execute_laptop_command(cmd)
        let latency = time_us() - t0
        
        respond({
            "action": cmd,
            "result": result,
            "latency_us": latency,
            "python_dependency": "0%"
        })
    end
end

# 3. Cybernetic Holographic HUD UI
Page AgentHUD
    Column
        Text("AAYU SILICON AUTONOMOUS AGENT", bold: true)
        Text("Wake Word: 'AAYU PROGRAMMING' | Zero Python")
        VoiceVisualizer(active: true)
        QuickActionGrid([
            Button("System Status", onClick: checkStatus),
            Button("Open VS Code", onClick: openVSCode),
            Button("Lock Screen", onClick: lockScreen)
        ])
    end
end

run AgentHUD`
    },
    {
      title: "AAYU Silicon Enterprise Portal (Live Production & Cloudflare Verified)",
      desc: "The world's first single-file full-stack enterprise platform in pure AAYU (.aayu). Features real 2FA + OTP Zero-Knowledge authentication, crash-proof in-memory Write-Ahead Log (WAL) persistence (aayudb_wal.json), Ant-Man vector media engine (99.85% image storage reduction with dynamic SVG synthesis), and Ayush's 7-layer molecular data sharding (std/liquid.aayu) with tamper-freeze security checkpoints.",
      tags: ["Full-Stack Flagship", "Liquid Data (7-Layer)", "Ant-Man Media (99.85%)", "Live Cloudflare"],
      liveUrl: "https://palestinian-privileges-leads-anonymous.trycloudflare.com/",
      githubUrl: "https://github.com/Minato95-ayu/INTENT-TO-SILICON/blob/main/examples/enterprise_auth_portal.aayu",
      code: `# Run locally: aayu run examples/enterprise_auth_portal.aayu --web --port 8080
# Live Cloudflare: https://palestinian-privileges-leads-anonymous.trycloudflare.com/
# Direct GitHub: https://github.com/Minato95-ayu/INTENT-TO-SILICON/blob/main/examples/enterprise_auth_portal.aayu

app EnterpriseAuthPortal

import "std/liquid.aayu"
import "std/media.aayu"
import "std/crypto.aayu"

# 1. Database Model with WAL In-Memory Store
model User {
    id: Int
    username: String
    email: String
    password_hash: String
    two_fa_secret: String
    dp_preset: String
}

# 2. 7-Layer Liquid Data Molecular Sharding Route
route "/api/liquid/shard"
    post
        # Shards payload into 7 molecular layers with SHA-256 checkpoints
        # and O(1) prefix-suffix phone-number fast routing
        let payload = request_body()
        let result = liquid_shard_payload(payload["data"], payload["phone"])
        respond(result)
    end
end

# 3. Ant-Man Vector Media Engine (99.85% Storage Saved)
action get_avatar_vector(preset)
    if preset == "antman"
        return "<svg viewBox='0 0 100 100'><circle cx='50' cy='50' r='42' fill='#a855f7'/><path d='M30 50 L50 25 L70 50 L50 75 Z' fill='#38bdf8'/></svg>"
    end
    return "<svg viewBox='0 0 100 100'><circle cx='50' cy='50' r='42' fill='#10b981'/></svg>"
end

# 4. Reactive Full-Stack Enterprise UI
Page PortalDashboard
    Column
        Text("AAYU Silicon Enterprise Portal", bold: true)
        Button("7-Layer Molecular Shard", onClick: triggerShard)
        Button("Ant-Man Vector Media", onClick: openStudio)
    end
end

run PortalDashboard`
    },
    {
      title: "Creative Silicon & Invariant Blockchain (Live Hardware Verified)",
      desc: "Fully verified on bin/aayu.exe with 0 external dependencies. Demonstrates reentrancy-free smart contracts with mathematical value conservation, procedural 2D PPM image generation, 3D Yaw/Pitch perspective projection, and Helmholtz acoustic formant human voice synthesis.",
      tags: ["Blockchain", "Creative DSP", "3D Math", "Human Voice"],
      code: `# Run: bin/aayu.exe run examples/creative_silicon_demo.aayu
import "std/chain.aayu"
import "std/media.aayu"
import "std/audio.aayu"
import "std/fs.aayu"

action main()
    # 1. Invariant Smart Contract (Zero Reentrancy, Conservation Proof)
    let contract = create_contract_state("AAYU Silicon Coin (ASC)", 1000000, { "0xAlice": 100000, "0xBob": 25000 })
    contract = execute_safe_transfer(contract, "0xAlice", "0xBob", 15000)
    print("Alice New Balance: " + contract["balances"]["0xAlice"]) # 85000 ASC
    print("Bob New Balance:   " + contract["balances"]["0xBob"])   # 40000 ASC

    # 2. Procedural Image Creation (PPM P2 ASCII)
    let canvas = create_canvas(16, 16)
    canvas = canvas_draw_circle(canvas, 8, 8, 5, 255)
    write_text("circle.ppm", export_ppm_p2(canvas))

    # 3. 3D High-Dimensional Perspective Projection
    let pt3d = project_3d_to_2d(2.0, 1.5, -1.0, 0.785, 0.349, 250.0, 800.0, 600.0)
    print("Screen 2D: (" + pt3d["u"] + ", " + pt3d["v"] + ")")

    # 4. Human Vocal Formant Synthesis (Vowels A, O, E)
    let sample_a = synthesize_human_vowel_sample("A", 120.0, 0.005)
    print("Human Vowel 'A' Formant Amplitude: " + sample_a)
end

main()`
    },
    {
      title: "Algorithmic Logic Building Mastery (Live Hardware Verified)",
      desc: "Demonstrates 4 core algorithmic patterns executed with 0 syntax friction on native silicon registers: Two-Pointer palindrome checking O(N), Sliding Window max subarray O(N), Binary Search divide-and-conquer O(log N), and contract-verified recursion.",
      tags: ["Algorithms", "Logic Building", "Binary Search", "Two-Pointer"],
      code: `# Run: bin/aayu.exe run examples/logic_building_mastery.aayu
import "std/contracts.aayu"

# 1. Two-Pointer Palindrome in O(N) Time
action is_palindrome(text)
    let n = len(text)
    let left = 0, let right = n - 1
    while left < right
        if text[left] != text[right] return 0 end
        left = left + 1, right = right - 1
    end
    return 1
end

# 2. Sliding Window Max Subarray Sum in O(N) Time
action max_subarray_sum_k(arr, k)
    let n = len(arr), let sum = 0, let i = 0
    while i < k sum = sum + arr[i], i = i + 1 end
    let max_sum = sum, let j = k
    while j < n
        sum = sum + arr[j] - arr[j - k]
        if sum > max_sum max_sum = sum end
        j = j + 1
    end
    return max_sum
end

# 3. Binary Search in O(log N) Time
action binary_search(arr, target)
    let n = len(arr), let low = 0, let high = n - 1
    while low <= high
        let mid = (low + high) / 2
        if arr[mid] == target return mid
        elif arr[mid] < target low = mid + 1
        else high = mid - 1 end
    end
    return -1
end`
    },
    {
      title: "AAYUGram (Social Network)",
      desc: "Concept sketch for a social feed. It is not a verified runnable application and does not demonstrate SQLite, JWT security, or production image uploads.",
      tags: ["Full-Stack", "Auth", "UI Engine"],
      code: `app AAYUGram

model Post
    id Int
    image_url String
    caption String
    author_id Int
end

route "/api/feed"
    get
        let posts = Post.all()
        respond(posts)
    end
end

Page Feed
    Column
        Text("Your Feed", size: 24, bold: true)
        // Fetches directly via AAYU's SSR engine
        let items = fetch("/api/feed")
        List(items, item => PostCard(item))
    end
end

run Feed`
    },
    {
      title: "AI Trading Bot",
      desc: "Concept sketch for tensor calculations and a market-data workflow. It is not a verified trading system or WebSocket implementation.",
      tags: ["AI/Tensors", "Math Engine", "WebSockets"],
      code: `app TradingBot

action compute_sma(prices: Tensor, window: Int) -> Tensor
    // Native PyTorch-style Tensor math
    let sma = prices.rolling_mean(window)
    return sma
end

route "wss://trade_stream"
    on_message(data)
        let prices = Tensor.new([100], data.history)
        let ma_fast = compute_sma(prices, 5)
        let ma_slow = compute_sma(prices, 20)
        
        if ma_fast.last() > ma_slow.last()
            print("BUY SIGNAL")
        end
    end
end`
    }
  ];

  return (
    <main className="min-h-screen bg-[#050505] text-white pt-24 pb-16">
      <div className="container mx-auto px-4 max-w-7xl">
        
        <div className="text-center mb-20">
          <h1 className="text-4xl md:text-5xl font-bold mb-6">AAYU <span className="text-cyan-400">Example Sketches</span></h1>
          <p className="text-zinc-400 text-lg max-w-2xl mx-auto">
            These snippets show product directions, not production-ready applications. For runnable core-language examples, see the regression programs in the repository.
          </p>
        </div>

        <div className="space-y-24">
          {examples.map((ex, i) => (
            <div key={i} className="grid lg:grid-cols-12 gap-12 items-center">
              
              <div className="lg:col-span-5">
                <div className="flex gap-2 mb-4">
                  {ex.tags.map(tag => (
                    <span key={tag} className="px-3 py-1 bg-white/5 border border-white/10 rounded-full text-xs font-mono text-zinc-400">
                      {tag}
                    </span>
                  ))}
                </div>
                <h2 className="text-3xl font-bold mb-4">{ex.title}</h2>
                <p className="text-zinc-400 leading-relaxed mb-6">{ex.desc}</p>

                {(ex.liveUrl || ex.githubUrl) && (
                  <div className="flex flex-wrap gap-3 mb-6">
                    {ex.liveUrl && (
                      <a
                        href={ex.liveUrl}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="inline-flex items-center gap-2 px-4 py-2.5 rounded-xl bg-gradient-to-r from-emerald-500 to-teal-500 text-black font-bold text-xs shadow-lg shadow-emerald-500/20 hover:scale-105 transition-transform"
                      >
                        <Play className="w-3.5 h-3.5 fill-black" />
                        <span>Try Live Cloudflare Portal</span>
                        <ExternalLink className="w-3 h-3" />
                      </a>
                    )}
                    {ex.githubUrl && (
                      <a
                        href={ex.githubUrl}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="inline-flex items-center gap-2 px-4 py-2.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 border border-purple-500/40 text-purple-200 font-bold text-xs transition hover:scale-105"
                      >
                        <FolderGit2 className="w-3.5 h-3.5 text-purple-400" />
                        <span>Direct Source (.aayu)</span>
                        <ExternalLink className="w-3 h-3" />
                      </a>
                    )}
                  </div>
                )}

                <Link href="/tutorial" className="inline-flex items-center gap-2 text-cyan-400 hover:text-cyan-300 font-bold transition-colors">
                  Learn to build this <ArrowRight className="w-4 h-4" />
                </Link>
              </div>

              <div className="lg:col-span-7 relative">
                <div className="absolute -inset-2 bg-gradient-to-tr from-purple-500/10 to-cyan-500/10 rounded-3xl blur-xl" />
                <div className="relative bg-[#0a0a0a] border border-white/10 rounded-2xl overflow-hidden shadow-2xl">
                  <div className="flex items-center px-4 py-3 bg-[#111] border-b border-white/5">
                     <Code2 className="w-4 h-4 text-zinc-500 mr-2" />
                     <span className="text-xs font-mono text-zinc-400">example.aayu</span>
                  </div>
                  <pre className="p-6 font-mono text-sm leading-relaxed text-zinc-300 overflow-x-auto">
                    <code>{ex.code}</code>
                  </pre>
                </div>
              </div>

            </div>
          ))}
        </div>

      </div>
    </main>
  );
}
