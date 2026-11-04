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
FILE: json_memory.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

import json
import os
from typing import Any, Dict, Optional
from .base import MemoryBackend

class JSONMemory(MemoryBackend):
    """
    JSON file-backed memory storage.
    Useful for local prototyping and snapshotting.
    """
    def __init__(self, file_path: str = "intent_memory.json"):
        self.file_path = file_path
        self._load()

    def _load(self):
        if os.path.exists(self.file_path):
            with open(self.file_path, 'r', encoding='utf-8') as f:
                self._data = json.load(f)
        else:
            self._data = {}

    def _save(self):
        with open(self.file_path, 'w', encoding='utf-8') as f:
            json.dump(self._data, f, indent=2)

    def _get_session_key(self, session_id: Optional[str]) -> str:
        return session_id if session_id else "global"

    def store(self, key: str, value: Any, session_id: Optional[str] = None) -> bool:
        session_key = self._get_session_key(session_id)
        if session_key not in self._data:
            self._data[session_key] = {}
        
        self._data[session_key][key] = value
        self._save()
        return True

    def retrieve(self, key: str, session_id: Optional[str] = None) -> Any:
        session_key = self._get_session_key(session_id)
        return self._data.get(session_key, {}).get(key, None)

    def retrieve_all(self, session_id: Optional[str] = None) -> Dict[str, Any]:
        session_key = self._get_session_key(session_id)
        return self._data.get(session_key, {})

    def clear(self, session_id: Optional[str] = None) -> bool:
        if session_id:
            if session_id in self._data:
                del self._data[session_id]
        else:
            self._data = {}
        
        self._save()
        return True
