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
FILE: llm_parser.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List, Optional
from .llm_router import LLMRouter
from .ir import IntentNode, EntityNode, FieldNode, RelationshipNode, ActionNode, ConstraintNode
import json

class LLMIntentParser:
    """
    Translates raw requirements into structured Intent IR nodes using an LLM.
    """
    def __init__(self, llm_router: LLMRouter):
        self.llm = llm_router
        self.schema = {
            "type": "object",
            "properties": {
                "intents": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "node_type": {"type": "string", "enum": ["entity", "field", "relationship", "action", "constraint"]},
                            "name": {"type": "string"},
                            "entity_name": {"type": "string"},
                            "field_name": {"type": "string"},
                            "field_type": {"type": "string"},
                            "source": {"type": "string"},
                            "relation": {"type": "string"},
                            "target": {"type": "string"},
                            "actor": {"type": "string"},
                            "action": {"type": "string"},
                            "rule_description": {"type": "string"}
                        },
                        "required": ["node_type"]
                    }
                }
            },
            "required": ["intents"]
        }
        self.system_prompt = (
            "You are an Intent Parser. Convert the following business requirement into a structured "
            "intent node. The node_type must be one of: entity, field, relationship, action, constraint. "
            "Populate only the fields relevant to the node_type. Maintain standard TitleCase for entities."
        )

    def parse(self, requirement: str) -> Optional[IntentNode]:
        try:
            response = self.llm.generate_structured(
                prompt=f"Parse this requirement: '{requirement}'",
                schema=self.schema,
                system_prompt=self.system_prompt
            )
            
            intents_data = response.get("intents", [])
            if not intents_data:
                return None
                
            data = intents_data[0]
            node_type = data.get("node_type")
            
            if node_type == "entity":
                return EntityNode(source_text=requirement, name=data.get("name", "Unknown"))
            elif node_type == "field":
                return FieldNode(source_text=requirement, entity_name=data.get("entity_name", ""), field_name=data.get("field_name", ""), field_type=data.get("field_type"))
            elif node_type == "relationship":
                return RelationshipNode(source_text=requirement, source=data.get("source", ""), relation=data.get("relation", ""), target=data.get("target", ""))
            elif node_type == "action":
                return ActionNode(source_text=requirement, actor=data.get("actor", ""), action=data.get("action", ""), target=data.get("target"))
            elif node_type == "constraint":
                return ConstraintNode(source_text=requirement, target=data.get("target", ""), rule_description=data.get("rule_description", ""))
                
            return None
            
        except Exception as e:
            print(f"Parsing failed for '{requirement}': {e}")
            return None
