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
FILE: sqlite_memory.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

import sqlite3
import json
from typing import Any, Dict, Optional
from .base import MemoryBackend

class SQLiteMemory(MemoryBackend):
    """
    SQLite database-backed memory storage.
    Provides robust, transactional storage for intent graphs and sessions.
    """
    def __init__(self, db_path: str = "intent_memory.db"):
        self.db_path = db_path
        self._init_db()

    def _init_db(self):
        with sqlite3.connect(self.db_path) as conn:
            conn.execute('''
                CREATE TABLE IF NOT EXISTS memory (
                    session_id TEXT,
                    key TEXT,
                    value TEXT,
                    PRIMARY KEY (session_id, key)
                )
            ''')
            conn.commit()

    def _get_session_key(self, session_id: Optional[str]) -> str:
        return session_id if session_id else "global"

    def store(self, key: str, value: Any, session_id: Optional[str] = None) -> bool:
        session_key = self._get_session_key(session_id)
        val_str = json.dumps(value)
        
        with sqlite3.connect(self.db_path) as conn:
            conn.execute('''
                INSERT INTO memory (session_id, key, value) 
                VALUES (?, ?, ?)
                ON CONFLICT(session_id, key) DO UPDATE SET value = excluded.value
            ''', (session_key, key, val_str))
            conn.commit()
        return True

    def retrieve(self, key: str, session_id: Optional[str] = None) -> Any:
        session_key = self._get_session_key(session_id)
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute('SELECT value FROM memory WHERE session_id = ? AND key = ?', (session_key, key))
            row = cursor.fetchone()
            if row:
                return json.loads(row[0])
        return None

    def retrieve_all(self, session_id: Optional[str] = None) -> Dict[str, Any]:
        session_key = self._get_session_key(session_id)
        result = {}
        with sqlite3.connect(self.db_path) as conn:
            cursor = conn.execute('SELECT key, value FROM memory WHERE session_id = ?', (session_key,))
            for row in cursor.fetchall():
                result[row[0]] = json.loads(row[1])
        return result

    def clear(self, session_id: Optional[str] = None) -> bool:
        with sqlite3.connect(self.db_path) as conn:
            if session_id:
                conn.execute('DELETE FROM memory WHERE session_id = ?', (session_id,))
            else:
                conn.execute('DELETE FROM memory')
            conn.commit()
        return True
