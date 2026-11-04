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
FILE: extractor.py
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
from .llm_router import LLMRouter
import json

class RequirementExtractor:
    """
    Extracts atomic, logical requirements from unstructured human thoughts/prompts.
    """
    def __init__(self, llm_router: LLMRouter):
        self.llm = llm_router
        self.schema = {
            "type": "object",
            "properties": {
                "requirements": {
                    "type": "array",
                    "items": {
                        "type": "string",
                        "description": "An atomic, distinct business requirement extracted from the prompt."
                    }
                }
            },
            "required": ["requirements"]
        }
        self.system_prompt = (
            "You are an expert systems analyst. Your job is to extract raw, unstructured human "
            "prompts into a list of atomic business requirements. Break down compound sentences "
            "into individual logical facts (e.g. 'Student has a name and belongs to a Library' -> "
            "['Student has a name', 'Student belongs to a Library']). Do not add any new information."
        )

    def extract(self, prompt: str) -> List[str]:
        try:
            response = self.llm.generate_structured(
                prompt=f"Extract requirements from this prompt:\n\n{prompt}",
                schema=self.schema,
                system_prompt=self.system_prompt
            )
            return response.get("requirements", [])
        except Exception as e:
            # Fallback for stubbed/mocked LLM
            return [prompt]
