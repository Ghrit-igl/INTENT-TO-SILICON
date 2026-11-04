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

class MemoryBackend(ABC):
    """
    Abstract base class for all Intent Engine memory backends.
    Stores and retrieves sessions, past intent graphs, and developer preferences.
    """
    
    @abstractmethod
    def store(self, key: str, value: Any, session_id: Optional[str] = None) -> bool:
        """
        Store a value in memory.
        """
        pass

    @abstractmethod
    def retrieve(self, key: str, session_id: Optional[str] = None) -> Any:
        """
        Retrieve a value from memory.
        """
        pass

    @abstractmethod
    def retrieve_all(self, session_id: Optional[str] = None) -> Dict[str, Any]:
        """
        Retrieve all keys and values for a specific session.
        """
        pass
    
    @abstractmethod
    def clear(self, session_id: Optional[str] = None) -> bool:
        """
        Clear memory for a session (or globally if session_id is None).
        """
        pass
