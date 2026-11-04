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
FILE: main.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

import sys
import os
import argparse

# Add repository root to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from brainos.workflow.engine import WorkflowEngine

def main():
    parser = argparse.ArgumentParser(description="BrainOS Autonomous Pipeline MVP")
    parser.add_argument("--goal", type=str, required=True, help="The high-level goal to execute.")
    args = parser.parse_args()
    
    engine = WorkflowEngine(state_dir=os.path.join(os.path.dirname(__file__), "state"))
    engine.run(args.goal)

if __name__ == "__main__":
    main()
