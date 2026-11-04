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
FILE: cross_question_engine.py
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
from intent_engine.intent_analyzer import IntentAnalyzer

class CrossQuestionEngine:
    def __init__(self):
        self.analyzer = IntentAnalyzer()
        
    def run_interactive(self, initial_prompt):
        print("[1/3] Understanding Intent...")
        analysis = self.analyzer.analyze(initial_prompt)
        
        if analysis["status"] != "SUCCESS":
            print(f"Error: {analysis['message']}")
            sys.exit(1)
            
        print(f"\nDetected Domain: {analysis['domain']}")
        
        print("Entities:")
        for e in analysis["entities"]:
            print(f"- {e}")
            
        missing = analysis["missing_features"]
        questions = analysis["questions"]
        
        if not questions:
            print("\n[SUCCESS] No further clarification needed. Intent Locked!")
            return analysis
            
        print("\n[2/3] Asking Clarification Questions...")
        
        collected_answers = {}
        for feature_key, question in zip(missing, questions):
            print(f"\nSystem: {question}")
            ans = input("You: ")
            collected_answers[feature_key] = ans
            
        print("\n[3/3] Finalizing Intent Graph...")
        analysis["user_answers"] = collected_answers
        print("[SUCCESS] Intent Locked!")
        
        return analysis

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print('Usage: python -m intent_engine.cross_question_engine "Build an LMS"')
        sys.exit(1)
        
    engine = CrossQuestionEngine()
    final_intent = engine.run_interactive(sys.argv[1])
