# ==============================================================================
# COPYRIGHT (C) 2026 AYUSH GHRIT KAUSHIK. ALL RIGHTS RESERVED.
# 
# This source code is the proprietary intellectual property of Ayush Ghrit Kaushik.
# GitHub: https://github.com/Ghrit-igl
# 
# UNAUTHORIZED COPYING, REPRODUCTION, OR DISTRIBUTION IS STRICTLY PROHIBITED.
# ANY ATTEMPT TO CLONE OR CREATE DERIVATIVE WORKS FROM AAYU WILL BE SUBJECT
# TO LEGAL ACTION.
# ==============================================================================

"""
=============================================================================
FILE: builder.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

import os
import sys
import json
import subprocess
from intent_engine.cross_question_engine import CrossQuestionEngine
from intent_engine.architecture_generator import ArchitectureGenerator
from intent_engine.code_generator import CodeGenerator

def build_app(intent: str):
    print("--- AAYU Intent-to-Silicon ---")
    
    # Phase 1 & 2: Understand and Ask
    cq_engine = CrossQuestionEngine()
    intent_graph = cq_engine.run_interactive(intent)
    
    if intent_graph["status"] != "SUCCESS":
        return
        
    # Phase 3: Architecture Generation
    print("\n[4/5] Generating Architecture Rules...")
    arch_gen = ArchitectureGenerator()
    arch = arch_gen.generate(intent_graph)
    
    with open("architecture.json", "w") as f:
        json.dump(arch, f, indent=2)
        
    # Phase 4: Code Generation
    print("[5/5] Generating AAYU Code...")
    code_gen = CodeGenerator()
    code_gen.generate(arch)
    
    # Phase 5: Compile and Run
    print("\n[SUCCESS] Project generated!")
    print("Compiling main.aayu...")
    
    cli_path = os.path.join(os.path.dirname(os.path.dirname(__file__)), "cli.py")
    subprocess.run([sys.executable, cli_path, "compile", "main.aayu"])
    
    print("\nServer running: http://localhost:8080")
    print("Press Ctrl+C to stop.\n")
    
    try:
        subprocess.run([sys.executable, cli_path, "vm", "main.ayc"])
    except KeyboardInterrupt:
        print("\nServer stopped.")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python -m prototype.intent_engine.builder \"Build a CRM\"")
        sys.exit(1)
    build_app(sys.argv[1])
