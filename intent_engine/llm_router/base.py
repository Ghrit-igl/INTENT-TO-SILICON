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
FILE: base.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from abc import ABC, abstractmethod
from typing import Any, Dict, List, Optional

class LLMRouter(ABC):
    """
    Abstract base class for all LLM interactions in the Intent Engine.
    This ensures that the Intent Engine is decoupled from any specific LLM provider.
    """
    
    @abstractmethod
    def generate(self, prompt: str, system_prompt: Optional[str] = None, temperature: float = 0.7, max_tokens: int = 1000) -> str:
        """
        Generate a plain text response from the LLM.
        """
        pass

    @abstractmethod
    def generate_structured(self, prompt: str, schema: Dict[str, Any], system_prompt: Optional[str] = None, temperature: float = 0.0) -> Dict[str, Any]:
        """
        Generate a structured JSON response strictly adhering to the provided schema.
        """
        pass
