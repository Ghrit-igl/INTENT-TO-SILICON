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
FILE: multi_step.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import Dict, Any, List
from ..graphs.intent_graph import IntentGraph
from ..llm_router import LLMRouter

class MultiStepReasoner:
    """
    Evaluates the Intent Graph for domain completeness and software engineering best practices.
    Acts as a critique loop before architecture generation.
    """
    def __init__(self, llm_router: LLMRouter):
        self.llm = llm_router

    def evaluate(self, graph: IntentGraph) -> List[str]:
        prompt = (
            "Review the following Intent Graph representing a software domain.\n"
            "Identify any violations of SOLID principles or major structural flaws.\n\n"
            f"Graph:\n{graph.to_dict()}"
        )
        
        response = self.llm.generate(
            prompt=prompt,
            system_prompt="You are a Software Architect Reviewer. Provide a list of structural critiques."
        )
        
        return [line.strip() for line in response.split('\n') if line.strip().startswith('-')]
