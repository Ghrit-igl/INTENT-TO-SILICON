import Link from 'next/link';

const priorities = [
  'Align the language specification with parser behavior and tested examples.',
  'Add reproducible compiler bootstrap comparison and make it a CI gate.',
  'Expand end-to-end coverage for features currently represented in compiler code.',
  'Harden JSON persistence or adopt a clearly specified database backend.',
  'Add CI coverage for Python tools, the VS Code extension, and release archives.',
];

export default function RoadmapPage() {
  return (
    <main className="min-h-screen pt-32 pb-20">
      <div className="container mx-auto px-4 max-w-4xl">
        <h1 className="text-5xl font-bold mb-6">AAYU Roadmap</h1>
        <p className="text-xl text-zinc-400 mb-12">The current priorities focus on making implementation, tests, documentation, and release packages agree.</p>
        <ol className="space-y-5 list-decimal pl-6 text-lg text-zinc-200">
          {priorities.map((item) => <li key={item}>{item}</li>)}
        </ol>
        <p className="mt-10 text-zinc-400">
          See the <Link className="text-cyan-400 underline" href="https://github.com/Ghrit-igl/INTENT-TO-SILICON/blob/main/ROADMAP.md">repository roadmap</Link> for the full status and later work.
        </p>
      </div>
    </main>
  );
}
