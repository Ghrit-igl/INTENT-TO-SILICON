 
import type { Metadata } from "next";
import "./globals.css";
import { Navbar } from "@/components/navbar";
import { Footer } from "@/components/footer";
import { GlobalSearch } from "@/components/global-search";

export const metadata: Metadata = {
  title: "AAYU Programming Language | By Ayush Ghrit Kaushik",
  description: "AAYU is a modern, simple, fast, offline-first, and AI-native programming language designed by Ayush Ghrit Kaushik. Build production apps by defining human intent instead of boilerplate.",
  keywords: ["AAYU", "AAYU Programming Language", "Ayush Ghrit Kaushik", "Intent to Silicon", "AI Programming", "AGI", "Compiler", "Ghrit-igl", "Web Development", "Database CRUD", "No-code alternative"],
  authors: [{ name: "Ayush Ghrit Kaushik", url: "https://github.com/Ghrit-igl" }],
  creator: "Ayush Ghrit Kaushik",
  publisher: "Ayush Ghrit Kaushik",
  robots: "index, follow",
  openGraph: {
    title: "AAYU Programming Language | Intent to Silicon",
    description: "Write less. Build more. AAYU is an intent-first programming language with a built-in BrainOS for autonomous software engineering. Created by Ayush Ghrit Kaushik.",
    url: "https://intent-to-silicon.vercel.app/",
    siteName: "AAYU",
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    title: "AAYU Programming Language",
    description: "The AI-native programming language created by Ayush Ghrit Kaushik.",
    creator: "@Ghrit-igl",
  }
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className="dark">
      <body
        className="antialiased bg-black text-white min-h-screen"
      >
        <div className="relative flex min-h-screen flex-col">
          <Navbar />
          <GlobalSearch />
          
          <script
            type="application/ld+json"
            dangerouslySetInnerHTML={{
              __html: JSON.stringify({
                "@context": "https://schema.org",
                "@type": "SoftwareApplication",
                "name": "AAYU",
                "operatingSystem": "Windows, Linux, macOS",
                "applicationCategory": "DeveloperApplication",
                "author": {
                  "@type": "Person",
                  "name": "Ayush Ghrit Kaushik",
                  "url": "https://github.com/Ghrit-igl"
                },
                "description": "A modern, AI-native programming language compiling human intent to scalable architecture.",
                "offers": {
                  "@type": "Offer",
                  "price": "0",
                  "priceCurrency": "USD"
                }
              })
            }}
          />

          {children}
          <Footer />
        </div>
      </body>
    </html>
  );
}
