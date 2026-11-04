import Link from "next/link";
import { notFound } from "next/navigation";
import { Metadata } from "next";
import { chapters } from "@/lib/tutorial-data";
import { ChevronLeft, ChevronRight, BookOpen, Lightbulb, Code, ArrowLeft } from "lucide-react";

export async function generateStaticParams() {
  return chapters.map((ch) => ({ slug: ch.slug }));
}

export async function generateMetadata({ params }: { params: Promise<{ slug: string }> }): Promise<Metadata> {
  const { slug } = await params;
  const chapter = chapters.find((ch) => ch.slug === slug);
  if (!chapter) return { title: "Chapter Not Found" };
  return {
    title: `Ch ${chapter.num}: ${chapter.title} | AAYU Tutorial`,
    description: chapter.description,
  };
}

export default async function ChapterPage({ params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;
  const chapter = chapters.find((ch) => ch.slug === slug);
  if (!chapter) notFound();

  const prevChapter = chapters.find((ch) => ch.num === chapter.num - 1);
  const nextChapter = chapters.find((ch) => ch.num === chapter.num + 1);

  // Simple markdown-like renderer
  const renderContent = (content: string) => {
    const lines = content.split("\n");
    const elements: React.ReactNode[] = [];
    let i = 0;
    let inCodeBlock = false;
    let codeLines: string[] = [];
    let codeLang = "";
    let inTable = false;
    let tableRows: string[][] = [];

    while (i < lines.length) {
      const line = lines[i];

      // Code blocks
      if (line.startsWith("```")) {
        if (inCodeBlock) {
          elements.push(
            <div key={`code-${i}`} className="my-6 rounded-xl overflow-hidden border border-zinc-800">
              <div className="flex items-center justify-between px-4 py-2 bg-zinc-900 border-b border-zinc-800">
                <span className="text-xs font-mono text-zinc-500">{codeLang || "code"}</span>
                <Code className="h-3.5 w-3.5 text-zinc-600" />
              </div>
              <pre className="p-4 bg-zinc-950 overflow-x-auto text-sm">
                <code className="text-green-400 font-mono leading-relaxed">
                  {codeLines.join("\n")}
                </code>
              </pre>
            </div>
          );
          codeLines = [];
          codeLang = "";
          inCodeBlock = false;
        } else {
          inCodeBlock = true;
          codeLang = line.slice(3).trim();
        }
        i++;
        continue;
      }

      if (inCodeBlock) {
        codeLines.push(line);
        i++;
        continue;
      }

      // Tables
      if (line.includes("|") && line.trim().startsWith("|")) {
        if (!inTable) {
          inTable = true;
          tableRows = [];
        }
        const cells = line.split("|").filter(c => c.trim()).map(c => c.trim());
        if (!cells.every(c => /^[-:]+$/.test(c))) {
          tableRows.push(cells);
        }
        i++;
        continue;
      } else if (inTable) {
        elements.push(
          <div key={`table-${i}`} className="my-6 overflow-x-auto rounded-xl border border-zinc-800">
            <table className="w-full text-sm">
              <thead>
                <tr className="bg-zinc-900">
                  {tableRows[0]?.map((cell, ci) => (
                    <th key={ci} className="px-4 py-3 text-left font-semibold text-zinc-300 border-b border-zinc-800">{cell}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {tableRows.slice(1).map((row, ri) => (
                  <tr key={ri} className="border-b border-zinc-800/50 hover:bg-zinc-900/50">
                    {row.map((cell, ci) => (
                      <td key={ci} className="px-4 py-3 text-zinc-400 font-mono text-xs">{cell}</td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        );
        inTable = false;
        tableRows = [];
        continue;
      }

      // Headers
      if (line.startsWith("### ")) {
        elements.push(<h3 key={`h3-${i}`} className="text-xl font-bold text-white mt-10 mb-4">{line.slice(4)}</h3>);
      } else if (line.startsWith("## ")) {
        elements.push(<h2 key={`h2-${i}`} className="text-2xl font-bold text-white mt-12 mb-4 pb-2 border-b border-zinc-800">{line.slice(3)}</h2>);
      }
      // Blockquotes
      else if (line.startsWith("> ")) {
        elements.push(
          <div key={`bq-${i}`} className="my-4 p-4 rounded-lg border-l-4 border-purple-500 bg-purple-500/10">
            <p className="text-purple-200 text-sm">{line.slice(2)}</p>
          </div>
        );
      }
      // Bullet points
      else if (line.startsWith("- ")) {
        elements.push(
          <div key={`li-${i}`} className="flex items-start gap-2 my-1 ml-4">
            <span className="text-purple-400 mt-1.5 text-xs">●</span>
            <span className="text-zinc-300 text-base leading-relaxed">{formatInlineCode(line.slice(2))}</span>
          </div>
        );
      }
      // Inline code in paragraphs
      else if (line.trim() === "") {
        elements.push(<div key={`br-${i}`} className="h-2" />);
      }
      // Bold text check
      else {
        elements.push(
          <p key={`p-${i}`} className="text-zinc-300 text-base leading-relaxed my-2">
            {formatInlineCode(line)}
          </p>
        );
      }

      i++;
    }

    // Flush remaining table
    if (inTable && tableRows.length > 0) {
      elements.push(
        <div key="table-end" className="my-6 overflow-x-auto rounded-xl border border-zinc-800">
          <table className="w-full text-sm">
            <thead>
              <tr className="bg-zinc-900">
                {tableRows[0]?.map((cell, ci) => (
                  <th key={ci} className="px-4 py-3 text-left font-semibold text-zinc-300 border-b border-zinc-800">{cell}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {tableRows.slice(1).map((row, ri) => (
                <tr key={ri} className="border-b border-zinc-800/50 hover:bg-zinc-900/50">
                  {row.map((cell, ci) => (
                    <td key={ci} className="px-4 py-3 text-zinc-400 font-mono text-xs">{cell}</td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      );
    }

    return elements;
  };

  return (
    <div className="min-h-screen bg-black text-white">
      <div className="absolute inset-0 bg-[linear-gradient(to_right,#80808008_1px,transparent_1px),linear-gradient(to_bottom,#80808008_1px,transparent_1px)] bg-[size:24px_24px]"></div>

      <main className="container relative mx-auto px-4 pt-28 pb-24 max-w-4xl">
        {/* Back link */}
        <Link href="/tutorial" className="inline-flex items-center gap-2 text-zinc-500 hover:text-purple-400 transition-colors mb-8 text-sm">
          <ArrowLeft className="h-4 w-4" />
          Back to Syllabus
        </Link>

        {/* Chapter header */}
        <div className="mb-12">
          <div className="flex items-center gap-3 mb-4">
            <span className="text-sm font-mono text-purple-400 bg-purple-500/10 px-3 py-1 rounded-full border border-purple-500/30">
              Chapter {chapter.num.toString().padStart(2, "0")}
            </span>
            <span className="text-sm text-zinc-600">{chapter.phase}</span>
          </div>
          <h1 className="text-4xl md:text-5xl font-extrabold tracking-tight mb-4">{chapter.title}</h1>
          <p className="text-lg text-zinc-400">{chapter.description}</p>
        </div>

        <div className="mb-10 rounded-xl border border-emerald-500/30 bg-emerald-500/5 p-5 text-sm text-emerald-100">
          This lesson uses the verified core subset and links to a checked-in regression program. Its output is covered by the repository test harness. This does not imply support for features outside the lesson; see <Link href="https://github.com/Ghrit-igl/INTENT-TO-SILICON/blob/main/KNOWN_LIMITATIONS.md" className="underline">known limitations</Link>.
        </div>

        {/* What you'll learn */}
        <div className="mb-12 p-6 rounded-xl border border-zinc-800 bg-zinc-900/50">
          <div className="flex items-center gap-2 mb-4">
            <Lightbulb className="h-5 w-5 text-yellow-400" />
            <h2 className="text-lg font-bold text-white">What you&apos;ll learn</h2>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
            {chapter.learningPoints.map((point, idx) => (
              <div key={idx} className="flex items-start gap-2">
                <span className="text-green-400 mt-0.5">✓</span>
                <span className="text-sm text-zinc-300">{point}</span>
              </div>
            ))}
          </div>
        </div>

        {/* Main content */}
        <article className="mb-16">
          {renderContent(chapter.content)}
        </article>

        {/* Exercise */}
        <div className="mb-16 p-6 rounded-xl border border-emerald-800/50 bg-emerald-900/10">
          <div className="flex items-center gap-2 mb-4">
            <BookOpen className="h-5 w-5 text-emerald-400" />
            <h2 className="text-lg font-bold text-emerald-300">Try It Yourself! 🚀</h2>
          </div>
          <p className="text-zinc-300 text-base leading-relaxed">{chapter.exercise}</p>
        </div>

        {/* Navigation */}
        <div className="flex items-center justify-between pt-8 border-t border-zinc-800">
          {prevChapter ? (
            <Link href={`/tutorial/${prevChapter.slug}`} className="flex items-center gap-2 text-zinc-400 hover:text-purple-400 transition-colors group">
              <ChevronLeft className="h-5 w-5 group-hover:-translate-x-1 transition-transform" />
              <div>
                <div className="text-xs text-zinc-600">Previous</div>
                <div className="text-sm font-medium">Ch {prevChapter.num}: {prevChapter.title}</div>
              </div>
            </Link>
          ) : <div />}
          {nextChapter ? (
            <Link href={`/tutorial/${nextChapter.slug}`} className="flex items-center gap-2 text-zinc-400 hover:text-purple-400 transition-colors text-right group">
              <div>
                <div className="text-xs text-zinc-600">Next</div>
                <div className="text-sm font-medium">Ch {nextChapter.num}: {nextChapter.title}</div>
              </div>
              <ChevronRight className="h-5 w-5 group-hover:translate-x-1 transition-transform" />
            </Link>
          ) : <div />}
        </div>
      </main>
    </div>
  );
}

// Helper to format inline code (`code`) in text
function formatInlineCode(text: string): React.ReactNode {
  const parts = text.split(/(\[[^\]]+\]\(https?:\/\/[^)]+\)|`[^`]+`|\*\*[^*]+\*\*)/g);
  return parts.map((part, i) => {
    const link = part.match(/^\[([^\]]+)\]\((https?:\/\/[^)]+)\)$/);
    if (link) {
      return <a key={i} href={link[2]} target="_blank" rel="noreferrer" className="text-cyan-300 underline">{link[1]}</a>;
    }
    if (part.startsWith("`") && part.endsWith("`")) {
      return <code key={i} className="px-1.5 py-0.5 rounded bg-zinc-800 text-purple-300 text-sm font-mono">{part.slice(1, -1)}</code>;
    }
    if (part.startsWith("**") && part.endsWith("**")) {
      return <strong key={i} className="text-white font-semibold">{part.slice(2, -2)}</strong>;
    }
    return <span key={i}>{part}</span>;
  });
}
