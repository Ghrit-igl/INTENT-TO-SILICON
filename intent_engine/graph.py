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
FILE: graph.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import Dict
from .intents import Intent, DefineEntityIntent, DefineFieldIntent, DefineRelationshipIntent, DefineTaskIntent

class IntentGraph:
    def __init__(self):
        # Format: {"Student": {"fields": ["name", "age"], "relations": [...]}}
        self.entities: Dict[str, dict] = {}
        self.raw_intents = []
        
    def ingest(self, intent: Intent):
        self.raw_intents.append(intent)
        
        if isinstance(intent, DefineEntityIntent):
            if intent.name not in self.entities:
                self.entities[intent.name] = {"fields": [], "relations": [], "tasks": [], "source_intents": [intent]}
            else:
                self.entities[intent.name]["source_intents"].append(intent)
                
        elif isinstance(intent, DefineFieldIntent):
            if intent.entity_name not in self.entities:
                self.entities[intent.entity_name] = {"fields": [], "relations": [], "tasks": [], "source_intents": []}
            if intent.field_name not in self.entities[intent.entity_name]["fields"]:
                self.entities[intent.entity_name]["fields"].append(intent.field_name)
            self.entities[intent.entity_name]["source_intents"].append(intent)

        elif isinstance(intent, DefineRelationshipIntent):
            if intent.source not in self.entities:
                self.entities[intent.source] = {"fields": [], "relations": [], "tasks": [], "source_intents": []}
            self.entities[intent.source]["relations"].append({
                "relation": intent.relation,
                "target": intent.target
            })
            self.entities[intent.source]["source_intents"].append(intent)
            
        elif isinstance(intent, DefineTaskIntent):
            if intent.actor not in self.entities:
                self.entities[intent.actor] = {"fields": [], "relations": [], "tasks": [], "source_intents": []}
            self.entities[intent.actor]["tasks"].append({
                "action": intent.action,
                "target": intent.target
            })
            self.entities[intent.actor]["source_intents"].append(intent)
            
    def get_snapshot(self):
        return self.entities
