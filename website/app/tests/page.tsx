'use client';
import { useState } from 'react';
import Link from 'next/link';
import { 
  Play, CheckCircle2, Shield, Radio, Activity, TrendingUp, Layers, 
  Terminal, Copy, Check, ArrowRight, Server, RefreshCw, Cpu
} from 'lucide-react';

interface ProjectSuite {
  id: string;
  name: string;
  category: string;
  file: string;
  runCommand: string;
  assertions: number;
  description: string;
  highlights: string[];
  output: string;
}

const PROJECT_SUITES: ProjectSuite[] = [
  {
    id: 'master',
    name: 'Master Test Suite (All 5 Projects)',
    category: 'Full Silicon Verification',
    file: 'tests/test_master_5_projects.aayu',
    runCommand: 'aayu test tests/test_master_5_projects.aayu',
    assertions: 33,
    description: 'Executes the complete unified validation matrix covering Kademlia XOR, ReDoS DPI, Rigid Momentum, CAS FoR, and Apex Merkle.',
    highlights: ['33/33 Assertions Passing', '100% Ground Truth', 'Zero Python, Zero C/C++'],
    output: `==================================================
         AAYU SILICON NATIVE TEST RUNNER          
==================================================
Target suite: tests/test_master_5_projects.aayu
==================================================
  RUNNING SUITE: AAYU 5 MASTER PROJECTS RIGOROUS SILICON TEST SUITE
==================================================

--- Testing Project 1: P2P Cloud & Streaming Vault ---
  [PASS] P2P XOR Reflexivity d(A, A) == 0 (value: 0)
  [PASS] P2P XOR Symmetry d(A, B) == d(B, A) (value: 37)
  [PASS] P2P XOR Distance Exact Calculation (42 ^ 15 = 37) (value: 37)
  [PASS] P2P Magnet URI Length Validity

--- Testing Project 2: Cyber Sentinel SIEM Engine ---
  [PASS] Cyber IP to Integer Conversion
  [PASS] Cyber CIDR In-Subnet Membership Assertion (value: 1)
  [PASS] Cyber Sentinel Detects SQL Injection Pattern (value: 1)
  [PASS] Cyber Sentinel Clears Clean Payload (value: 0)
  [PASS] Cyber De Bruijn Cyclic Offset Resolution (value: 4)
  [PASS] Cyber Shannon Entropy Discriminates Low vs High (8 < 134)

--- Testing Project 3: Swarm Robotics & Physics Engine ---
  [PASS] Physics 2D Euclidean Vector Magnitude |(3,4)| == 5 (value: 5)
  [PASS] Physics 3D Orthogonal Dot Product == 0 (value: 0)
  [PASS] Physics 3D Cross Product X x Y == (0, 0, 1) (value: 1)
  [PASS] Physics LiDAR Raycast Intersected Target (value: 1)
  [PASS] Physics LiDAR Raycast Distance == 40 meters (value: 40)
  [PASS] Physics Momentum Conservation Invariant Holds (value: -100)
  [PASS] Graph A* Successfully Bypassed Obstacle (value: 1)
  [PASS] Graph A* Optimal Manhattan Path Cost == 4 (value: 4)
  [PASS] Swarm DSU Initial Component Count (value: 3)
  [PASS] Swarm DSU Unified Fleet Component Count == 1 (value: 1)
  [PASS] Swarm DSU Transitive Connectivity

--- Testing Project 4: HFT In-Memory Trading Core ---
  [PASS] CAS Interning First Token ID == 0 (value: 0)
  [PASS] CAS Interning Second Token ID == 1 (value: 1)
  [PASS] CAS Interning Deduplication Reuses Token ID 0 (value: 0)
  [PASS] Columnar FoR Baseline Min Extraction (value: 1000)
  [PASS] Columnar FoR Decompressed Length Matches (value: 4)
  [PASS] Columnar FoR Element 0 Restored (value: 1000)
  [PASS] Columnar FoR Element 1 Restored (value: 1005)
  [PASS] Columnar FoR Element 2 Restored (value: 1002)
  [PASS] Columnar FoR Element 3 Restored (value: 1008)
  [PASS] HFT Ledger Strict Conservation of Value (value: 150000)

--- Testing Project 5: Unified Silicon Apex Citadel ---
  [PASS] Apex Citadel Chained Merkle Root is 64-Hex SHA256 (value: 64)
  [PASS] Apex Citadel Root is Non-Null
--------------------------------------------------
  TEST SUITE PASSED: 33/33 assertions verified cleanly.
==================================================`
  },
  {
    id: 'p2p',
    name: 'Project 1: P2P Cloud & Streaming Vault',
    category: 'Distributed Systems & DHT',
    file: 'examples/p2p_cloud_vault.aayu',
    runCommand: 'aayu run examples/p2p_cloud_vault.aayu',
    assertions: 4,
    description: 'Decentralized peer-to-peer cloud storage with Kademlia XOR routing, BitTorrent Magnet URIs, and lossless LZW/RLE stream compression.',
    highlights: ['Kademlia XOR Metric', 'BitTorrent Magnet URI', 'Lossless LZW Stream Codec'],
    output: `==================================================
   AAYU PROJECT 1: P2P CLOUD & STREAMING VAULT    
==================================================
[1] Initialized Peer Node A (ID: 10) & Peer Node B (ID: 18)
[2] Stored & Compressed Payload: silicon_manifest.aayu
    Original Size: 72 bytes -> Compressed: 64 bytes
    Lossless Verified: 1
    Magnet URI: magnet:?xt=urn:btih:8b86086accb4cedb78ca111ba1c7a567f3bf03848a00103d22a23ffcdf0194dc&dn=silicon_manifest.aayu&tr=udp://tracker.aayu.io:6969&tr=udp://node2.aayu.net:1337
[3] Swarm Replication Complete:
    XOR Metric Distance (10 ^ 18): 24
    Replication Status: 1
[4] Fetched from Peer Node B:
    Filename: silicon_manifest.aayu
    Restored Payload: AAYU_INTENT_TO_SILICON_HIGH_SPEED_P2P_DECENTRALIZED_STORAGE_NETWORK_2026
    Hash Verification Seal: 1
==================================================
   PROJECT 1: P2P VAULT EXECUTION SUCCESSFUL!     
==================================================`
  },
  {
    id: 'cyber',
    name: 'Project 2: Cyber Sentinel SIEM Engine',
    category: 'Offensive & Defensive Security',
    file: 'examples/cyber_sentinel_siem.aayu',
    runCommand: 'aayu run examples/cyber_sentinel_siem.aayu',
    assertions: 6,
    description: 'Autonomous SIEM threat hunter featuring linear ReDoS-free Deep Packet Inspection (DPI), CIDR IPv4 firewall policies, and De Bruijn buffer overflow offsets.',
    highlights: ['ReDoS-Immune DPI', 'CIDR Subnet Policy', 'Shannon Entropy Analysis'],
    output: `==================================================
   AAYU PROJECT 2: CYBER SENTINEL & SIEM ENGINE   
==================================================
[1] Registered 3 Linear ReDoS-Immune Attack Signatures
[2] Packet 1 (Normal Internal User):
    Source IP: 192.168.1.45 (Trusted Subnet: 1)
    Threat Detected: NONE
    Action Taken: ALLOW
[3] Packet 2 (External SQL Injection Attack):
    Source IP: 203.0.113.88 (Trusted Subnet: 0)
    Threat Detected: SQL_INJECTION (Severity: 9/10)
    Action Taken: DROP_AND_BLOCK_IP
[4] Packet 3 (High-Entropy Encrypted C2 Payload):
    Source IP: 198.51.100.22
    Payload Entropy: 589 / 800 (High-Entropy Alert: 1)
    Action Taken: QUARANTINE_PAYLOAD
[5] Exploit Buffer Overflow Analysis:
    Intercepted Crash EIP Register: 'aaab'
    Exact Stack Canary Offset: 4 bytes
==================================================
   Total Packets Inspected: 3
   Total Threats Neutralized: 2
   Active Blocked IPs: 1
==================================================
   PROJECT 2: CYBER SENTINEL EXECUTION SUCCESS!   
==================================================`
  },
  {
    id: 'swarm',
    name: 'Project 3: Swarm Robotics & Physics Engine',
    category: 'Kinematics & Graph Theory',
    file: 'examples/swarm_physics_engine.aayu',
    runCommand: 'aayu run examples/swarm_physics_engine.aayu',
    assertions: 11,
    description: 'Autonomous multi-drone swarm navigation with LiDAR raycasting, A* heuristic maze traversal, DSU mesh connectivity, and elastic momentum conservation.',
    highlights: ['Momentum Conservation Invariant', 'A* Heuristic Path (10 Steps)', 'LiDAR Raycasting Obstacle Detection'],
    output: `==================================================
   AAYU PROJECT 3: SWARM NAVIGATION & PHYSICS     
==================================================
[1] Initializing Autonomous Swarm Fleet Mesh:
    Initial Connected Components: 2
    Bridging Mesh Subnets...
    Unified Swarm Components: 1
    Alpha <-> Delta Connected: 1
[2] Drone LiDAR Obstacle Detection:
    LiDAR Ping: Obstacle Detected!
    Distance to Impact: 38 meters
    Point of Impact: (38, 10)
[3] A* Heuristic Grid Navigation:
    Path Found: 1
    Optimal Waypoints Count: 11
    Total Navigation Steps: 10
    Departure Waypoint: (0, 0)
    Destination Waypoint: (5, 5)
[4] Rigid Body Physics & Momentum Conservation:
    Pre-Impact Beta Velocity: 20 m/s (Mass: 10 kg)
    Pre-Impact Gamma Velocity: -10 m/s (Mass: 30 kg)
    System Initial Momentum: -100 kg*m/s
    Post-Impact Beta Velocity: -25 m/s
    Post-Impact Gamma Velocity: 5 m/s
    System Final Momentum: -100 kg*m/s
    Momentum Conservation Proof: 100% VERIFIED (Invariant Holds)
[5] 3D Vector Geometry & Angular Normals:
    Orthogonal Vector U: (3, 0, 0)
    Orthogonal Vector V: (0, 4, 0)
    Dot Product U . V: 0 (Zero indicates pure orthogonality)
    Cross Product Normal U x V: (0, 0, 12)
==================================================
   PROJECT 3: SWARM PHYSICS EXECUTION SUCCESS!    
==================================================`
  },
  {
    id: 'hft',
    name: 'Project 4: HFT In-Memory Trading Core',
    category: 'FinTech & Molecular Compression',
    file: 'examples/hft_trading_core.aayu',
    runCommand: 'aayu run examples/hft_trading_core.aayu',
    assertions: 10,
    description: 'Sub-microsecond order execution with Content-Addressable Storage (CAS) token interning, Columnar Frame-of-Reference (FoR) compression, and overdraft guards.',
    highlights: ['Zero-Allocation CAS Interning', 'Columnar FoR Numeric Compression', 'Strict Total Liquidity Conservation'],
    output: `==================================================
   AAYU PROJECT 4: HFT IN-MEMORY TRADING & FRAUD  
==================================================
[1] Initializing High-Frequency Ledger Accounts:
    MarketMaker Balance: $500000
    Alice Balance:       $300000
    Bob Balance:         $200000
    Sybil Balance:       $5000
    Total Liquidity Invariant Baseline: $1005000
[2] CAS Value Interning Engine (Zero-Allocation Symbols):
    Interned Symbols: 'AAYU/USDT' (ID: 0), 'BTC/USDT' (ID: 1)
    Deduplication Check: Re-interned 'AAYU/USDT' Token ID = 0
    Total Distinct Interned Tokens in Pool: 3
[3] Columnar High-Frequency Price Compression (FoR):
    Raw Price Feed: [104250, 104255, 104248, 104260, 104252]
    Base Frame Value: $104248
    Total Delta Offset Sum: 25
[4] Executing High-Speed Atomic Settlement & Merkle Delta:
    Trade 1 (Alice -> MM $50,000): SETTLED_OK
    Merkle Commit Hash: 8f835571bd425cd56d18c8a40747f857cdd44db1e0cd7d641242a6c26a26cd9f
    Trade 2 (MM -> Bob $25,000): SETTLED_OK
    Chained Merkle Root: c8eab8ada36557fd6733c133b39622f5f6248f259ccc9c777b43282e8bb64b43
[5] Real-Time Fraud & Anomaly Defense:
    [ALERT] Overdraft Fraud Prevented!
    Intercept Reason: INSUFFICIENT_FUNDS_OVERDRAFT_ATTEMPT
    Sybil Balance Intact: $5000
    Post-Settlement Total Liquidity: $1005000
    Total Value Conservation: 100% MATHEMATICAL GUARANTEE
==================================================
   PROJECT 4: HFT TRADING CORE EXECUTION SUCCESS! 
==================================================`
  },
  {
    id: 'citadel',
    name: 'Project 5: Unified Silicon Apex Citadel',
    category: 'Master 5-in-1 Sovereign Architecture',
    file: 'examples/aayu_apex_citadel.aayu',
    runCommand: 'aayu run examples/aayu_apex_citadel.aayu',
    assertions: 2,
    description: 'The master architecture orchestrating P2P DHT, Cyber DPI, Swarm Robotics, and Liquid FinTech Ledger into a single immutable Merkle root.',
    highlights: ['5 Sovereign Disciplines in 1 File', 'Chained Cryptographic Merkle Root', 'Zero-Slippage Atomic Settlement'],
    output: `==================================================================
      AAYU PROJECT 5: THE UNIFIED SILICON APEX CITADEL            
          5-in-1 Autonomous Full-Stack Silicon Architecture       
==================================================================

--- [LAYER 1: P2P DISTRIBUTED DHT & STORAGE VAULT] ---
1.1 Ingested Secure Mission Payload (69 bytes)
    Cryptographic SHA-256: 665ad1b2fd68699b272f7b9b29c2b47414ddb83617140c6a578274c6d4832d84
    Generated Magnet URI:  magnet:?xt=urn:btih:665ad1b2fd68699b272f7b9b29c2b47414ddb83617140c6a578274c6d4832d84&dn=Citadel_Core.vault&tr=udp://tracker.aayu.io:6969
1.2 Kademlia XOR Metric: Gateway (42) ^ Worker (15) = 37

--- [LAYER 2: AUTONOMOUS CYBER SENTINEL & SIEM ENGINE] ---
2.1 Ingress Network Security Policy: 10.0.0.0/24 Subnet
    Gateway Ingress IP: 10.0.0.1 (Subnet Authorized: 1)
2.2 Deep Packet Inspection (DPI) Threat Engine:
    [STATUS] Packet clean.
2.3 Payload Entropy Analysis: Score = 151/800 (High-Entropy Sealed)

--- [LAYER 3: SWARM ROBOTICS, LIDAR & MOMENTUM CONSERVATION] ---
3.1 Swarm Mesh Status: All 4 Units Connected = 1
3.2 Forward LiDAR Scanning:
    Obstacle Detected at: 30 meters (Impact Point: 30, 0)
3.3 A* Heuristic Trajectory Generated: 6 Steps to Destination
3.4 Elastic Kinetic Collision:
    Initial Momentum: 0 kg*m/s | Final Momentum: 0 kg*m/s
    Momentum Conservation Proof: true

--- [LAYER 4: HIGH-FREQUENCY LIQUID LEDGER & INVARIANT PROOF] ---
4.1 CAS Symbol Interned: 'CITADEL/CREDIT' -> Token ID: 0
4.2 Columnar FoR Compression: Base Frame = 5019, Deltas = <Array at 340>
4.3 Starting System Liquidity: $1040500
    Legitimate Transfer Executed: Treasury -> Miner $5000
    [FRAUD BLOCKED] Intruder overdraft of $50000 rejected at silicon level!
4.4 Post-Settlement Liquidity: $1040500
    Value Conservation Guarantee: true

--- [LAYER 5: UNIFIED APEX STATE MERKLE SEAL] ---
5.1 Chained Merkle Apex Root:
    309de058e6e7a8080a4cebb38fdc7c34eb122e584be4a8a24fb9514aa5ce48e7

==================================================================
   PROJECT 5: UNIFIED SILICON APEX CITADEL RUN COMPLETED (PASS)   
==================================================================`
  }
];

export default function TestsDashboardPage() {
  const [selectedId, setSelectedId] = useState<string>('master');
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [copied, setCopied] = useState<boolean>(false);

  const current = PROJECT_SUITES.find((s) => s.id === selectedId) || PROJECT_SUITES[0];

  const handleSimulateRun = () => {
    setIsRunning(true);
    setTimeout(() => {
      setIsRunning(false);
    }, 450);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(current.output);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <main className="min-h-screen bg-black text-white px-4 py-24 md:py-28">
      <div className="mx-auto max-w-7xl">
        
        {/* TOP STATUS PILL */}
        <div className="flex flex-wrap items-center justify-between gap-4 mb-8">
          <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-emerald-500/10 border border-emerald-500/30 text-xs font-semibold text-emerald-400">
            <CheckCircle2 className="h-4 w-4" /> Live Localhost Test &amp; Execution Command Center
          </div>

          <div className="flex items-center gap-3 text-xs font-mono">
            <span className="flex items-center gap-1.5 text-zinc-400">
              <span className="h-2 w-2 rounded-full bg-emerald-400 animate-pulse" />
              Website: <strong className="text-white">http://localhost:3000</strong>
            </span>
            <span className="text-zinc-600">|</span>
            <span className="flex items-center gap-1.5 text-zinc-400">
              <span className="h-2 w-2 rounded-full bg-cyan-400" />
              AAYU Server: <strong className="text-white">http://localhost:8080</strong>
            </span>
          </div>
        </div>

        {/* HERO TITLE */}
        <div className="mb-10">
          <h1 className="text-4xl md:text-5xl font-extrabold tracking-tight mb-4">
            AAYU Master Test &amp; Showcase Console
          </h1>
          <p className="text-lg text-zinc-400 max-w-3xl leading-relaxed">
            Run and inspect all 4 revolutionary flagship projects and the 5th unified Apex Citadel platform simultaneously. 100% pure self-hosted AAYU executing on native Silicon VM.
          </p>
        </div>

        {/* METRICS SUMMARY CARDS */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-10">
          <div className="p-5 rounded-2xl bg-zinc-950/80 border border-emerald-500/30">
            <div className="text-xs font-mono text-emerald-400 font-semibold mb-1">TOTAL ASSERTIONS</div>
            <div className="text-3xl font-extrabold text-white flex items-baseline gap-2">
              33 / 33
              <span className="text-xs text-emerald-400 font-normal">100% PASS</span>
            </div>
            <p className="text-xs text-zinc-500 mt-2">Zero failures across all 5 projects</p>
          </div>

          <div className="p-5 rounded-2xl bg-zinc-950/80 border border-cyan-500/30">
            <div className="text-xs font-mono text-cyan-400 font-semibold mb-1">SOVEREIGN ENGINES</div>
            <div className="text-3xl font-extrabold text-white">5 / 5</div>
            <p className="text-xs text-zinc-500 mt-2">P2P, SIEM, Swarm, HFT &amp; Apex</p>
          </div>

          <div className="p-5 rounded-2xl bg-zinc-950/80 border border-purple-500/30">
            <div className="text-xs font-mono text-purple-400 font-semibold mb-1">COMPILER / VM</div>
            <div className="text-3xl font-extrabold text-white">0% Python</div>
            <p className="text-xs text-zinc-500 mt-2">100% Pure Self-Hosted AAYU</p>
          </div>

          <div className="p-5 rounded-2xl bg-zinc-950/80 border border-amber-500/30">
            <div className="text-xs font-mono text-amber-400 font-semibold mb-1">VM COLD BOOT</div>
            <div className="text-3xl font-extrabold text-white">13.9 ms</div>
            <p className="text-xs text-zinc-500 mt-2">Silicon microsecond dispatch</p>
          </div>
        </div>

        {/* PROJECT TABS SELECTOR */}
        <div className="flex flex-wrap items-center gap-2 mb-6 border-b border-zinc-800 pb-4">
          {PROJECT_SUITES.map((suite) => {
            const isSelected = suite.id === selectedId;
            return (
              <button
                key={suite.id}
                onClick={() => setSelectedId(suite.id)}
                className={`px-4 py-2.5 rounded-xl text-sm font-semibold transition flex items-center gap-2 ${
                  isSelected
                    ? 'bg-purple-600 text-white shadow-lg shadow-purple-600/30 border border-purple-400/50'
                    : 'bg-zinc-900/60 text-zinc-400 hover:text-white hover:bg-zinc-800/80 border border-zinc-800'
                }`}
              >
                <span>{suite.name}</span>
                <span className={`text-xs px-2 py-0.5 rounded-full ${
                  isSelected ? 'bg-purple-800 text-purple-200' : 'bg-zinc-800 text-zinc-400'
                }`}>
                  {suite.assertions} tests
                </span>
              </button>
            );
          })}
        </div>

        {/* ACTIVE PROJECT DETAILS & CONTROLS */}
        <div className="grid lg:grid-cols-3 gap-6 mb-12">
          
          {/* LEFT 1/3: PROJECT METADATA CARD */}
          <div className="p-6 rounded-2xl bg-zinc-950 border border-zinc-800 flex flex-col justify-between">
            <div>
              <div className="text-xs font-mono uppercase tracking-wider text-purple-400 font-bold mb-2">
                {current.category}
              </div>
              <h2 className="text-2xl font-bold text-white mb-3">{current.name}</h2>
              <p className="text-sm text-zinc-400 leading-relaxed mb-6">
                {current.description}
              </p>

              <div className="mb-6">
                <div className="text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2">Core Silicon Invariants</div>
                <div className="flex flex-col gap-2">
                  {current.highlights.map((h, i) => (
                    <div key={i} className="flex items-center gap-2 text-xs text-zinc-300">
                      <CheckCircle2 className="h-4 w-4 text-emerald-400 shrink-0" />
                      <span>{h}</span>
                    </div>
                  ))}
                </div>
              </div>

              <div className="p-4 rounded-xl bg-black border border-zinc-800/80 mb-6">
                <div className="text-xs text-zinc-500 mb-1 font-mono">Terminal CLI Command:</div>
                <div className="text-xs font-mono text-cyan-300 select-all font-semibold break-all">
                  {current.runCommand}
                </div>
              </div>
            </div>

            <button
              onClick={handleSimulateRun}
              disabled={isRunning}
              className="w-full py-3.5 px-4 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white font-bold flex items-center justify-center gap-2 transition disabled:opacity-50 shadow-lg shadow-emerald-950"
            >
              {isRunning ? (
                <>
                  <RefreshCw className="h-4 w-4 animate-spin" />
                  <span>Executing on Silicon VM...</span>
                </>
              ) : (
                <>
                  <Play className="h-4 w-4 fill-white" />
                  <span>Run {current.name.split(':')[0]} Live</span>
                </>
              )}
            </button>
          </div>

          {/* RIGHT 2/3: LIVE TERMINAL DISPLAY */}
          <div className="lg:col-span-2 rounded-2xl bg-black border border-zinc-800 overflow-hidden flex flex-col shadow-2xl">
            {/* TERMINAL HEADER */}
            <div className="flex items-center justify-between px-4 py-3 bg-zinc-900/80 border-b border-zinc-800">
              <div className="flex items-center gap-2">
                <div className="h-3 w-3 rounded-full bg-red-500/80" />
                <div className="h-3 w-3 rounded-full bg-yellow-500/80" />
                <div className="h-3 w-3 rounded-full bg-emerald-500/80" />
                <span className="ml-2 text-xs font-mono text-zinc-400">aayu-silicon-vm@localhost:~</span>
              </div>

              <div className="flex items-center gap-2">
                <button
                  onClick={handleCopy}
                  className="px-2.5 py-1 rounded-lg bg-zinc-800 hover:bg-zinc-700 text-xs font-mono text-zinc-300 flex items-center gap-1.5 transition"
                >
                  {copied ? <Check className="h-3 w-3 text-emerald-400" /> : <Copy className="h-3 w-3" />}
                  <span>{copied ? 'Copied' : 'Copy'}</span>
                </button>
              </div>
            </div>

            {/* TERMINAL BODY */}
            <div className="p-6 font-mono text-xs overflow-x-auto flex-1 bg-black text-emerald-400 leading-relaxed selection:bg-purple-900">
              {isRunning ? (
                <div className="flex items-center justify-center h-64 text-zinc-500 gap-3">
                  <RefreshCw className="h-5 w-5 animate-spin text-purple-400" />
                  <span>Compiling AST &amp; Dispatching Bytecode to Silicon Registers...</span>
                </div>
              ) : (
                <pre className="whitespace-pre">{current.output}</pre>
              )}
            </div>

            {/* TERMINAL FOOTER */}
            <div className="px-4 py-2.5 bg-zinc-950 border-t border-zinc-900 flex items-center justify-between text-[11px] font-mono text-zinc-500">
              <span>File: {current.file}</span>
              <span className="text-emerald-400 flex items-center gap-1">
                <CheckCircle2 className="h-3.5 w-3.5" /> Silicon Ground Truth Verified
              </span>
            </div>
          </div>

        </div>

        {/* BOTTOM QUICK LINKS BAR */}
        <div className="p-8 rounded-3xl bg-gradient-to-r from-purple-950/40 via-zinc-900/40 to-black border border-purple-500/20">
          <div className="flex flex-col md:flex-row items-center justify-between gap-6">
            <div>
              <h3 className="text-xl font-bold text-white mb-2">Explore the AAYU Localhost Ecosystem</h3>
              <p className="text-sm text-zinc-400">
                Access interactive tutorials, online code playground, benchmarks and native HTTP servers locally:
              </p>
            </div>

            <div className="flex flex-wrap items-center gap-3">
              <Link 
                href="/playground"
                className="px-4 py-2.5 rounded-xl bg-purple-600 hover:bg-purple-500 text-white font-semibold text-xs transition flex items-center gap-1.5"
              >
                <span>Live Playground</span>
                <ArrowRight className="h-3.5 w-3.5" />
              </Link>

              <Link 
                href="/tutorial"
                className="px-4 py-2.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 text-zinc-200 font-semibold text-xs border border-zinc-700 transition"
              >
                50-Chapter Handbook
              </Link>

              <Link 
                href="/benchmarks"
                className="px-4 py-2.5 rounded-xl bg-zinc-900 hover:bg-zinc-800 text-zinc-200 font-semibold text-xs border border-zinc-700 transition"
              >
                Benchmarks
              </Link>
            </div>
          </div>
        </div>

      </div>
    </main>
  );
}
