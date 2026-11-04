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
FILE: clarification_engine.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List
from intent_engine.graphs.intent_graph import IntentGraph, IntentGraphNode

class ClarificationQuestion:
    def __init__(self, node_ref: str, question: str):
        self.node_ref = node_ref
        self.question = question

    def __repr__(self):
        return f"Clarification({self.node_ref}): {self.question}"

class ClarificationEngine:
    """
    Analyzes the Intent Graph deterministically to find ambiguities, 
    missing fields, or dangling relationships.
    """
    def __init__(self, graph: IntentGraph):
        self.graph = graph
        self.questions: List[ClarificationQuestion] = []

    def run(self) -> List[ClarificationQuestion]:
        self.questions = []
        for node in self.graph.root_nodes:
            self._analyze_node(node)
        return self.questions

    def _analyze_node(self, node: IntentGraphNode):
        node_type = node.ir_node.node_type
        
        if node_type == "entity":
            if not getattr(node.ir_node, 'fields', []):
                self.questions.append(
                    ClarificationQuestion(
                        node.ir_node.name,
                        f"The entity '{node.ir_node.name}' was detected but it has no fields. What fields should it contain?"
                    )
                )
                
        elif node_type == "relationship":
            if not node.ir_node.target:
                self.questions.append(
                    ClarificationQuestion(
                        f"rel_{node.ir_node.source}",
                        f"The entity '{node.ir_node.source}' has a relationship '{node.ir_node.relation}', but the target entity is missing."
                    )
                )
                
        elif node_type == "action":
            if not node.ir_node.target:
                self.questions.append(
                    ClarificationQuestion(
                        f"action_{node.ir_node.action}",
                        f"The action '{node.ir_node.action}' was detected, but what does it operate on?"
                    )
                )

        # Recurse through dependents
        for child in node.outgoing:
            self._analyze_node(child)
