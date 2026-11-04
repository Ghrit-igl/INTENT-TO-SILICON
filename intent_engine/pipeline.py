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
FILE: pipeline.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from .llm_router import OpenAIConnector
from .extractor import RequirementExtractor
from .llm_parser import LLMIntentParser
from .graphs.intent_graph import IntentGraph
from .architecture_generator import ArchitectureGenerator
from .code_planner import CodePlanner
from .validator.deterministic_validator import DeterministicValidator

class IntentPipeline:
    """
    The main integration pipeline linking Human Intent to BrainOS tasks.
    """
    def __init__(self):
        self.llm = OpenAIConnector()
        self.extractor = RequirementExtractor(self.llm)
        self.parser = LLMIntentParser(self.llm)
        self.arch_generator = ArchitectureGenerator(self.llm)
        self.planner = CodePlanner()
        self.validator = DeterministicValidator()

    def process(self, raw_prompt: str) -> bool:
        print(f"--- Processing Intent ---\nPrompt: {raw_prompt}")
        
        # 1. Extract Requirements
        requirements = self.extractor.extract(raw_prompt)
        print(f"Extracted Requirements: {len(requirements)}")
        
        # 2. Parse into Intent Graph
        intent_graph = IntentGraph()
        for req in requirements:
            node = self.parser.parse(req)
            if node:
                intent_graph.ingest(node)
        print("Intent Graph Built.")
        
        # 3. Generate Architecture
        arch_graph = self.arch_generator.generate(intent_graph)
        print(f"Architecture Graph Built: {len(arch_graph.records)} records, {len(arch_graph.interfaces)} interfaces.")
        
        # 4. Validate Architecture
        errors = self.validator.get_errors(arch_graph)
        if errors:
            print(f"Validation Failed: {errors}")
            return False
            
        # 5. Plan Code Generation Tasks
        tasks = self.planner.plan(arch_graph)
        print(f"Generated {len(tasks)} tasks for BrainOS execution.")
        
        # 6. Pass to BrainOS (Mocked)
        self._dispatch_to_brainos(tasks)
        return True

    def _dispatch_to_brainos(self, tasks: list):
        print("Dispatching to BrainOS Task Graph...")
        for i, task in enumerate(tasks):
            print(f"  [{i+1}] {task['type']}: {task['target']}")
        print("Pipeline Complete.")

if __name__ == "__main__":
    pipeline = IntentPipeline()
    pipeline.process("Make a Library system where a User can borrow Books.")
