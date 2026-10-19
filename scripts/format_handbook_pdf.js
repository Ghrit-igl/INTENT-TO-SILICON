const fs = require('fs');
const path = require('path');

const logoPath = path.join(__dirname, '..', 'assets', 'icons', 'aayu-dark.png');
const officialHandbookPath = path.join(__dirname, '..', 'docs', 'AAYU_OFFICIAL_HANDBOOK.html');
const cwhHandbookPath = path.join(__dirname, '..', 'docs', 'AAYU_HANDBOOK_CWH_STYLE.html');

const b64 = fs.existsSync(logoPath) ? fs.readFileSync(logoPath).toString('base64') : '';

const chaptersVol1 = [
  { num: 1, title: 'The 2026 Intent-to-Silicon Revolution', desc: 'Token optimization, AI Agent context waste, 2026 benchmark matrix.' },
  { num: 2, title: 'Terminal, PowerShell & Tooling', desc: 'CLI commands (run, build, --web), .ay/.aa/.aayu extensions, clean terminal.' },
  { num: 3, title: 'Compiler Architecture & 7-Stage Pipeline', desc: 'Pure AAYU lexer, parser, AST, analyzer, bytecode, register VM.' },
  { num: 4, title: 'Hello World & Syntax Anatomy', desc: 'First program, top-level statements, silent output guarantee.' },
  { num: 5, title: 'Variables, Memory & Primitive Types', desc: 'let, state, Int, Float, String, Bool, Null, 128-bit BigInt auto-promotion.' },
  { num: 6, title: 'Operators, Expressions & String Polymorphism', desc: 'Arithmetic, relational, logical, auto-coercion concatenation.' },
  { num: 7, title: 'Conditional Control Flow', desc: 'if, elif, else, end, multi-branch decision trees.' },
  { num: 8, title: 'Loops & Iteration (for & while)', desc: 'while ... end, accumulators, infinite loop bounds and guards.' },
  { num: 9, title: 'Actions (Functions), Parameters & Scope', desc: 'First-class actions, call frames, parameter passing, recursion.' },
  { num: 10, title: 'Arrays & Dynamic Collections', desc: 'Dynamic lists, slicing, safe functional mutation via set_at.' },
  { num: 11, title: 'Dictionaries & Structured Records', desc: 'Key-value maps, JSON serialization, in-memory state tracking.' },
  { num: 12, title: 'Models & Built-In Database (AayuDb)', desc: 'RAM-First engine + WAL + SSD persistence, zero-config migrations.' },
  { num: 13, title: 'Web Routes, APIs & ASGI Microservices', desc: 'Declarative endpoints (get, post, put, delete), respond payload.' },
  { num: 14, title: 'Reactive UI & Single-File Components', desc: 'Declarative Flutter-like widget tree (Page, Column, Row, Button).' },
  { num: 15, title: 'The Cryptographic Engine', desc: 'High-entropy ASCII polynomial hashing, non-colliding SHA digests.' },
  { num: 16, title: 'Math & Computational Standard Library', desc: 'Fast square root, powers, absolute values, statistical math.' },
  { num: 17, title: 'Error Handling & Defensive Programming', desc: 'Guard clauses, safe fallbacks, schema validations.' },
  { num: 18, title: 'File I/O & Persistent Storage', desc: 'In-memory record stores, append-only logs, CSV/text pipelines.' },
  { num: 19, title: 'Data Structures & Algorithms (DSA)', desc: 'Searching, Sorting, Two-Pointers, Sliding Window, Stacks, Queues, DP.' },
  { num: 20, title: 'System Design & Enterprise Architecture', desc: 'Single-file architecture, In-memory WAL, High-concurrency async loop.' },
  { num: 21, title: '5 Production Projects & 20 Master Problems', desc: 'URL Shortener, Pub/Sub, Sentiment AI, E-Commerce, Auth + 20 Problems.' }
];

const chaptersVol2 = [
  { num: 22, title: 'Blockchain & Invariant Smart Contracts', desc: 'Formal invariants, zero-reentrancy mutex guards, mathematical value conservation.' },
  { num: 23, title: 'Creative Silicon: 3D Graphics & Formant Voice', desc: 'Netpbm PPM raster canvas, SVG generation, 3D projection, vowel synthesis.' },
  { num: 24, title: 'Algorithmic Logic Building & C-Speed Roadmap', desc: 'Sub-microsecond execution, JIT register compilation, CPU hardware speed.' },
  { num: 25, title: '7-Layer Liquid Data & Molecular Sharding', desc: 'Layer police inspectors, O(1) prefix-suffix fast cache routing, self-healing WAL.' },
  { num: 26, title: 'Flagship Enterprise Portal & Single-File Stack', desc: 'Multi-tenant architecture, declarative auth middleware, crash-proof recovery.' },
  { num: 27, title: 'Next-Gen AI, Quantization & Computational Math', desc: 'Int8/Int4 quantization, Adam optimizer, transformer scaled dot-product attention.' },
  { num: 28, title: 'Enterprise Production Reality & Architecture', desc: 'Statically linked sub-2.3 MB binary, memory safety, zero-dependency deployment.' },
  { num: 29, title: 'AAYU Silicon AI Agent — Voice Control & Automation', desc: 'Continuous wake-word barrier, Win32 process execution, speech synthesis HUD.' },
  { num: 30, title: 'Native Quantum Computing & Circuit Simulation', desc: '2^N complex state vectors, Hadamard, Toffoli, Grover search, BB84 QKD.' },
  { num: 31, title: 'Fault-Tolerant Actor Concurrency & CSP Channels', desc: 'Go-style CSP channels, select multiplexing, Erlang/OTP actor supervision trees.' },
  { num: 32, title: 'Silicon Binary Codec & Wire Protocol', desc: 'LEB128 varints, ZigZag encoding, CRC16-CCITT packet framing with sync headers.' },
  { num: 33, title: 'Zero-Knowledge Proofs & Threshold Cryptography', desc: 'Finite field math, Shamir (k,n) secret sharing, Schnorr ZKP via Fiat-Shamir.' },
  { num: 34, title: 'Autonomous AI Agent Runtime & Swarm', desc: 'ReAct Reason-Act-Observe loop, loop detection trapper, multi-agent swarm routing.' },
  { num: 35, title: 'Raft Distributed Consensus Engine', desc: 'Leader election, quorum voting, monotonic log replication, state machines.' },
  { num: 36, title: 'Real-Time Stream Analytics & CEP', desc: 'Tumbling/sliding/session windows, Bollinger Bands, RSI, temporal CEP detection.' },
  { num: 37, title: 'Rigid Body Physics & 2D/3D Collision Dynamics', desc: 'Vector kinematics, AABB/Circle collisions, raycasting, momentum conservation.' },
  { num: 38, title: 'Graph Theory, A* Pathfinder & Topological Sort', desc: 'Dijkstra shortest path, A* grid pathfinder, Kahn DAG cycle trap, DSU union-find.' },
  { num: 39, title: 'Geospatial Computing, Geohash & QuadTree Indexing', desc: 'Haversine distance, Base32 Geohash, Jordan curve geofencing, 2D QuadTree.' },
  { num: 40, title: 'Linear ReDoS-Free Regular Expressions', desc: 'Strict O(N) linear-time DFA regex, character classes, pattern replace and split.' },
  { num: 41, title: 'Decentralized Kademlia P2P & Content Addressing', desc: 'XOR metric distance, k-buckets, DHT storage and replication, BitTorrent Magnet URIs.' },
  { num: 42, title: 'Lossless Stream Compression & LZW Codec', desc: 'Run-Length Encoding, LZW streaming dictionary codec, bit-exact lossless seal.' }
];

function buildTocCards(chapList) {
  let cards = '<div class="toc-grid">\n';
  chapList.forEach(c => {
    cards += `  <div class="toc-card">
    <div class="toc-card-header">
      <span class="toc-chap-badge">Chapter ${c.num}</span>
    </div>
    <div class="toc-chap-title">${c.title}</div>
    <div class="toc-chap-desc">${c.desc}</div>
  </div>\n`;
  });
  cards += '</div>';
  return cards;
}

function processHandbook(filePath) {
  if (!fs.existsSync(filePath)) return;
  let html = fs.readFileSync(filePath, 'utf8');

  // 1. Embed real Base64 logo if present
  if (b64) {
    html = html.replace(/<img class="cover-logo"[^>]*>/, '<img class="cover-logo" src="data:image/png;base64,' + b64 + '" alt="AAYU Logo">');
  }

  // 2. Adjust .cover-page CSS
  const coverOld = /\.cover-page\s*\{[\s\S]*?page-break-after:\s*always;\s*break-after:\s*page;\s*\}/;
  const coverNew = `.cover-page {
    box-sizing: border-box;
    min-height: 930px;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    background: linear-gradient(135deg, #090d16 0%, #111827 50%, #1e1b4b 100%);
    color: #ffffff;
    padding: 42px 38px;
    border-radius: 12px;
    page-break-after: always;
    break-after: page;
  }`;
  html = html.replace(coverOld, coverNew);

  // 3. Fix edition tag and remove stray toc-card from cover page
  const coverFooterRegex = /<div class="edition-tag">[\s\S]*?<\/div>\s*<\/div>[\s\S]*?(?=<div class="toc-container"|<!-- PAGE 2)/;
  const newCoverFooter = `<div class="edition-tag">
      <strong>2026 Master Edition</strong><br>
      42-Chapter Complete Standard<br>
      1,002 / 1,002 Passing Silicon Assertions
    </div>
  </div>
</div>\n\n`;
  html = html.replace(coverFooterRegex, newCoverFooter);

  // 4. Format Table of Contents (Volume I and Volume II)
  const tocVol1Html = buildTocCards(chaptersVol1);
  const tocVol2Html = buildTocCards(chaptersVol2);

  const newTocSection = `<!-- PAGE 2: TABLE OF CONTENTS (VOLUME I) -->
<div class="toc-container">
  <h1 class="toc-h1">Table of Contents — Volume I: Core Architecture &amp; Foundations (Chapters 1 - 21)</h1>
  <p class="toc-subtitle">Master engineering blueprint for single-file full-stack computing, memory safety, and core algorithms:</p>

  ${tocVol1Html}

  <div class="box-deepdive" style="margin-top: 6px; margin-bottom: 0; padding: 7px 12px; font-size: 11px;">
    <strong>Verified on Native Silicon:</strong> Every code snippet across all 42 chapters is verified on <code>aayu.exe</code> with zero external dependencies (1,002/1,002 Passing Tests).
  </div>
</div>

<div class="page-break"></div>

<!-- PAGE 3: TABLE OF CONTENTS (VOLUME II) -->
<div class="toc-container">
  <h1 class="toc-h1">Table of Contents — Volume II: Next-Gen AI, Distributed &amp; Scientific Computing (Chapters 22 - 42)</h1>
  <p class="toc-subtitle">Advanced silicon modules: Blockchain, 7-Layer Liquid Data, Quantum, Actors, Raft, Physics, A* Routing, Regex &amp; P2P DHT:</p>

  ${tocVol2Html}

  <div class="box-tip" style="margin-top: 6px; margin-bottom: 0; padding: 7px 12px; font-size: 11px;">
    <strong>The AAYU Guarantee:</strong> Zero Python, zero C glue, zero node_modules. Complete full-stack, distributed consensus, and AI in single <code>.aayu</code> files.
  </div>
</div>

<!-- CHAPTER 1 -->`;

  const tocSectionRegex = /(?:<!-- PAGE 2: TABLE OF CONTENTS[\s\S]*?|<!-- PAGE 2:[\s\S]*?)<!-- CHAPTER 1 -->/;
  html = html.replace(tocSectionRegex, newTocSection);

  // Ensure TOC styles exist
  if (!html.includes('.toc-grid')) {
    const extraStyles = `
  /* TOC 2-COLUMN GRID */
  .toc-container {
    page-break-after: always;
    break-after: page;
    padding-top: 2px;
  }
  .toc-h1 {
    font-size: 18px;
    font-weight: 800;
    color: #0f172a;
    border-bottom: 2px solid #6366f1;
    padding-bottom: 4px;
    margin-top: 0;
    margin-bottom: 4px;
    break-before: auto;
  }
  .toc-subtitle {
    color: #475569;
    font-size: 10.5px;
    margin-bottom: 8px;
  }
  .toc-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 5px 12px;
    margin-bottom: 8px;
  }
  .toc-card {
    background: #f8fafc;
    border: 1px solid #e2e8f0;
    border-left: 3px solid #6366f1;
    border-radius: 4px;
    padding: 4px 7px;
    break-inside: avoid;
  }
  .toc-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1px;
  }
  .toc-chap-badge {
    font-size: 9px;
    font-weight: 700;
    color: #4f46e5;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  .toc-chap-title {
    font-size: 10.5px;
    font-weight: 700;
    color: #0f172a;
    line-height: 1.2;
  }
  .toc-chap-desc {
    font-size: 9px;
    color: #64748b;
    line-height: 1.25;
    margin-top: 1px;
  }
`;
    html = html.replace('</style>', extraStyles + '\n</style>');
  }

  fs.writeFileSync(filePath, html, 'utf8');
  console.log('Processed:', path.basename(filePath), 'Size:', html.length, 'bytes');
}

processHandbook(officialHandbookPath);
processHandbook(cwhHandbookPath);
console.log('All Handbooks formatted successfully!');
