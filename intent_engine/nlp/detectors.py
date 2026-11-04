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
FILE: detectors.py
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
from typing import List, Dict, Any, Optional

from intent_engine.nlp.parser import NLPPipeline, Token
from intent_engine.ir.nodes import Intent, EntityNode, FieldNode, RelationshipNode, ConstraintNode, ActionNode, FlowNode

class KnowledgeBase:
    """Loads all knowledge dictionaries for the offline Intent Engine."""
    def __init__(self, kb_path: str):
        self.kb_path = kb_path
        self.entities = self._load("entities.json")
        self.actions = self._load("actions.json")
        self.constraints = self._load("constraints.json")
        self.relationships = self._load("relationships.json")
        self.fields = self._load("fields.json")
        
        # Build inverted index for fast synonym lookup: word -> Canonical Entity
        self.inverted_entities = self._build_inverted(self.entities)
        self.inverted_actions = self._build_inverted(self.actions)
        self.inverted_constraints = self._build_inverted(self.constraints)
        self.inverted_relationships = self._build_inverted(self.relationships)
        self.inverted_fields = self._build_inverted(self.fields)

    def _load(self, filename: str) -> Dict[str, Any]:
        filepath = os.path.join(self.kb_path, filename)
        if os.path.exists(filepath):
            with open(filepath, 'r') as f:
                return json.load(f)
        return {}

    def _build_inverted(self, kb_dict: Dict[str, List[str]]) -> Dict[str, str]:
        inverted = {}
        for canonical, synonyms in kb_dict.items():
            inverted[canonical.lower()] = canonical
            for syn in synonyms:
                inverted[syn.lower()] = canonical
        return inverted


class RuleBasedIntentParser:
    """
    Constructs the structured Intent IR from tagged Tokens.
    Uses the Knowledge Base to resolve synonyms.
    """
    def __init__(self, kb: KnowledgeBase):
        self.kb = kb
        self.nlp = NLPPipeline()

    def parse(self, text: str) -> Intent:
        sentences = self.nlp.process(text)
        
        intent = Intent(original_intent=text)
        
        # We process each sentence to find entities, actions, fields, etc.
        for tokens in sentences:
            self._detect_entities_and_fields(tokens, intent)
            self._detect_actions(tokens, intent)
            self._detect_relationships(tokens, intent)
            self._detect_constraints(tokens, intent)
            
        return intent

    def _detect_entities_and_fields(self, tokens: List[Token], intent: Intent):
        current_entity: Optional[EntityNode] = None
        
        for token in tokens:
            word = token.lemma
            
            # Check for Entities
            if word in self.kb.inverted_entities:
                canonical = self.kb.inverted_entities[word]
                # Avoid duplicates
                if not any(e.name == canonical for e in intent.entities):
                    entity = EntityNode(node_type="entity", name=canonical, source_text=token.text)
                    intent.entities.append(entity)
                    current_entity = entity
                else:
                    current_entity = next(e for e in intent.entities if e.name == canonical)
            
            # Check for Fields attached to the most recent entity
            elif word in self.kb.inverted_fields and current_entity:
                canonical_field = self.kb.inverted_fields[word]
                if not any(f.name == canonical_field for f in current_entity.fields):
                    field = FieldNode(node_type="field", name=canonical_field, source_text=token.text)
                    current_entity.fields.append(field)

    def _detect_actions(self, tokens: List[Token], intent: Intent):
        for token in tokens:
            word = token.lemma
            if word in self.kb.inverted_actions:
                canonical = self.kb.inverted_actions[word]
                # Default actor is 'System' if not specified
                action = ActionNode(node_type="action", actor="System", action=canonical, source_text=token.text)
                intent.actions.append(action)

    def _detect_relationships(self, tokens: List[Token], intent: Intent):
        # A simple sliding window or state machine can detect X relation Y
        pass # To be fully fleshed out as the Intent Graph evolves

    def _detect_constraints(self, tokens: List[Token], intent: Intent):
        # Simple constraint detection
        pass # To be fully fleshed out as the Intent Graph evolves
