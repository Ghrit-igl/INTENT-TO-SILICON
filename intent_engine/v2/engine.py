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
FILE: engine.py
PURPOSE: Core IntentEngine orchestrator for v2
=============================================================================
"""

from typing import Dict, Any, List
from .tokenizer.tokenizer import Tokenizer
from .semantic.semantic import SemanticAnalyzer
from .graph.semantic_graph import SemanticGraph
from .graph.knowledge_graph import KnowledgeGraph
from .resolver.constraint_resolver import ConstraintResolver

class IntentEngine:
    """
    IntentEngine parses natural language prompts and generates an Intent IR.
    """
    def __init__(self):
        self.tokenizer = Tokenizer()
        self.semantic = SemanticAnalyzer()
        self.semantic_graph = SemanticGraph()
        self.knowledge_graph = KnowledgeGraph()
        self.constraint_resolver = ConstraintResolver()

    def process_prompt(self, prompt: str) -> Dict[str, Any]:
        print("[IntentEngine] Processing user prompt...")
        
        # 1. Tokenization (Multi-intent splitting)
        tokens = self.tokenizer.tokenize(prompt)
        
        # 2. Semantic Analysis
        entities, actions, requirements = self.semantic.analyze(tokens)
        
        # 3. Build Semantic Graph
        s_graph = self.semantic_graph.build(entities, actions, requirements)
        
        # 4. Integrate with Knowledge Graph (memory)
        self.knowledge_graph.update(s_graph)
        enriched_graph = self.knowledge_graph.enrich(s_graph)
        
        # 5. Constraint Resolution (e.g., check compatibility)
        resolved_graph = self.constraint_resolver.resolve(enriched_graph)
        
        # 6. Generate Intent IR
        ir = {
            "prompt": prompt,
            "entities": resolved_graph.get("entities", []),
            "actions": resolved_graph.get("actions", []),
            "non_functional": resolved_graph.get("requirements", {}),
            "intents": resolved_graph.get("intents", [])
        }
        
        print("[IntentEngine] Generated Intent IR.")
        return ir
