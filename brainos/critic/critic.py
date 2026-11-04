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
FILE: critic.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from ..task.task import Task

class Critic:
    def evaluate(self, task: Task, execution_result: str) -> str:
        """
        Evaluates execution results, tests, diagnostics, and architecture violations.
        Returns 'PASS', 'FAIL', or 'SKIP'.
        """
        if execution_result != "PASS":
            return execution_result
            
        print("\n" + "="*50)
        print(f"CRITIC VERIFICATION: {task.id}")
        print("Running tests... (mock: PASS)")
        print("Checking Architecture Freeze... (mock: PASS)")
        print("Checking Regression... (mock: PASS)")
        print("="*50)
        
        while True:
            doc_sync = input("Has documentation (Snapshot, Roadmap, Decision Log, Changelog) been synced? (PASS / FAIL): ").strip().upper()
            if doc_sync in ("PASS", "FAIL"):
                if doc_sync == "FAIL":
                    return "FAIL"
                break
            print("Invalid input. Please enter PASS or FAIL.")
            
        while True:
            type_cov = input("Is Type Coverage acceptable for this phase? (PASS / FAIL): ").strip().upper()
            if type_cov in ("PASS", "FAIL"):
                return type_cov
            print("Invalid input. Please enter PASS or FAIL.")
