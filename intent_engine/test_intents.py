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
FILE: test_intents.py
PURPOSE: Test suite for AAYU components
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles test suite for aayu components.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

import sys
from intent_engine.parser import IntentParser
from intent_engine.graph import IntentGraph
from intent_engine.verifier import VerificationEngine
from intent_engine.generator import AayuGenerator

def run_pipeline(thoughts):
    print("="*40)
    parser = IntentParser()
    graph = IntentGraph()

    print("HUMAN THOUGHTS:")
    for t in thoughts:
        print(f"- {t}")
        intent = parser.parse(t)
        if intent:
            graph.ingest(intent)

    print("\n[Layer 3] ENTITY GRAPH SNAPSHOT:")
    for entity, data in graph.entities.items():
        print(f"\n{entity}")
        for field in data["fields"]:
            print(f"  - {field}")
        for rel in data.get("relations", []):
            print(f"  -> {rel['relation']} -> {rel['target']}")
        for task in data.get("tasks", []):
            print(f"  [Task] can {task['action']} {task['target']}")

    verifier = VerificationEngine()
    report = verifier.verify(graph)

    print("\n[Layer 4] VERIFICATION REPORT:")
    print(f"Status: {report.status}")
    print(f"Score: {report.score}")
    print(f"Confidence: {report.confidence}%")
    for msg in report.passes + report.warnings + report.failures:
        print(f"  {msg}")

    print("\n[Layer 5] AAYU GENERATOR:")
    generator = AayuGenerator()
    try:
        result = generator.generate(graph, report)
        print("\n=== GENERATED CODE ===")
        print(result.code)
        print("=== GENERATION METADATA ===")
        import json
        print(json.dumps(result.metadata, indent=2))
    except Exception as e:
        print(f"\nGeneration Blocked: {str(e)}")

if __name__ == "__main__":
    # Test 1: Missing Target
    thoughts_broken = [
        "Make a Student entity",
        "Add name to student",
        "Student belongs to Library"
    ]
    print("TEST 1: BROKEN RELATIONSHIP")
    run_pipeline(thoughts_broken)
    
    # Test 2: Valid Graph
    thoughts_valid = [
        "Make a Library entity",
        "Add owner to library",
        "Make a Student entity",
        "Add name to student",
        "Student belongs to Library"
    ]
    print("\n\nTEST 2: VALID RELATIONSHIP")
    run_pipeline(thoughts_valid)
    
    # Test 3: Task Behavior
    thoughts_tasks = [
        "Make a Student entity",
        "Add name to student",
        "Make a Book entity",
        "Add title to book",
        "Make a Library entity",
        "Add address to library",
        "Student can borrow books",
        "Student can return books",
        "Library can issue books"
    ]
    print("\n\nTEST 3: TASK BEHAVIOR")
    run_pipeline(thoughts_tasks)
