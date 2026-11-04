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
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List, Dict, Any
from ..graphs.intent_graph import IntentGraph
from ..llm_router import LLMRouter

class ClarificationEngineV2:
    """
    Analyzes the Intent Graph for ambiguities, missing fields, or broken relationships.
    Uses the LLM Router to formulate intelligent questions for the human.
    """
    def __init__(self, llm_router: LLMRouter):
        self.llm = llm_router

    def _detect_anomalies(self, graph: IntentGraph) -> List[str]:
        anomalies = []
        for entity, data in graph.entities.items():
            if not data["fields"] and not data["relationships"]:
                anomalies.append(f"Entity '{entity}' has no fields or relationships. Is it an empty marker?")
            
            for action in data["actions"]:
                target = action.get("target")
                if target:
                    # Check if there's a relationship path to the target
                    has_rel = any(rel["target"] == target for rel in data["relationships"])
                    if not has_rel:
                        anomalies.append(f"'{entity}' can '{action['action']}' a '{target}', but there is no structural relationship between them.")
        return anomalies

    def evaluate(self, graph: IntentGraph) -> List[str]:
        anomalies = self._detect_anomalies(graph)
        if not anomalies:
            return []

        prompt = (
            "Given the following architectural anomalies detected in an Intent Graph, "
            "formulate clear, concise questions to ask the human developer to clarify their intent.\n\n"
            f"Anomalies:\n{chr(10).join(anomalies)}"
        )
        
        response = self.llm.generate(
            prompt=prompt,
            system_prompt="You are a Technical Clarification Engine. Ask direct questions."
        )
        # We simulate returning a list of questions
        # In a real implementation, we'd use generate_structured to get an array of strings
        return [q.strip() for q in response.split('\n') if q.strip().endswith('?')]
