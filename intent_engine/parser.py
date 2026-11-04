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
FILE: parser.py
PURPOSE: Parsing - Converts tokens to Abstract Syntax Tree (AST)
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles parsing - converts tokens to abstract syntax tree (ast).
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

import re
from .intents import DefineEntityIntent, DefineFieldIntent, DefineRelationshipIntent, DefineTaskIntent, Intent

class IntentParser:
    def parse(self, text: str) -> Intent:
        text_lower = text.lower().strip()
        
        # Define Entity Rule
        # E.g. "Make a student record", "Create a library entity", "make a student"
        entity_match = re.search(r'(?:create|make)\s+(?:a\s+|an\s+)?([a-zA-Z_]+)\s*(?:record|entity)?', text_lower)
        if entity_match:
            name = entity_match.group(1).title()
            return DefineEntityIntent(
                confidence=0.85, # Rule-based baseline confidence
                source_text=text,
                name=name
            )
            
        # Define Relationship Rule
        # E.g. "Student belongs to School"
        rel_match = re.search(r'([a-z_]+)\s+(belongs to|has a|owns)\s+([a-z_]+)', text_lower)
        if rel_match:
            source = rel_match.group(1).title()
            relation_raw = rel_match.group(2).lower()
            target = rel_match.group(3).title()
            
            relation_map = {
                "belongs to": "belongs_to",
                "has a": "has_a",
                "owns": "owns"
            }
            return DefineRelationshipIntent(
                confidence=0.90,
                source_text=text,
                source=source,
                relation=relation_map.get(relation_raw, relation_raw),
                target=target
            )

        # Define Field Rule
        # E.g. "Add age to student", "Add name to student"
        field_match = re.search(r'add\s+([a-zA-Z_]+)\s+to\s+([a-zA-Z_]+)', text_lower)
        if field_match:
            field = field_match.group(1).lower()
            entity = field_match.group(2).title()
            return DefineFieldIntent(
                confidence=0.90,
                source_text=text,
                entity_name=entity,
                field_name=field
            )
        # Define Task Rule
        # E.g. "Student can borrow books"
        task_match = re.search(r'([a-z_]+)\s+can\s+([a-z_]+)\s+([a-z_]+)s?', text_lower)
        if task_match:
            actor = task_match.group(1).title()
            action = task_match.group(2).lower()
            target = task_match.group(3).title()
            
            # Singularize target if it ends with 's' and isn't already singular
            if target.endswith('s') and text_lower.endswith('s'):
                 target = target[:-1]

            return DefineTaskIntent(
                confidence=0.90,
                source_text=text,
                actor=actor,
                action=action,
                target=target
            )
            
        return None
