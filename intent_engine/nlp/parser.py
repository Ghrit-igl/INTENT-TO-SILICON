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
from typing import List, Dict, Tuple, Optional

class Token:
    def __init__(self, text: str, pos_tag: str = "UNKNOWN"):
        self.text = text
        self.pos_tag = pos_tag
        self.lemma = text.lower()
        
    def __repr__(self):
        return f"Token({self.text}, {self.pos_tag})"

class NLPPipeline:
    """
    A deterministic, offline NLP pipeline for the AAYU Intent Engine.
    Requires ZERO external ML dependencies.
    """
    
    def __init__(self):
        # Basic heuristic dictionaries for POS tagging
        self.articles = {"a", "an", "the"}
        self.prepositions = {"in", "on", "at", "to", "for", "with", "by", "from", "of", "as"}
        self.conjunctions = {"and", "or", "but", "if", "because", "while", "where"}
        self.pronouns = {"i", "you", "he", "she", "it", "we", "they", "me", "him", "her", "us", "them", "my", "your", "his", "their", "our"}
        
        # Action verbs commonly used in software spec
        self.verbs = {
            "create", "build", "make", "add", "update", "delete", "remove", 
            "store", "save", "fetch", "get", "read", "write", "validate", 
            "check", "ensure", "manage", "handle", "process", "choose", "select"
        }
        
        # Software specific nouns
        self.nouns = {
            "system", "app", "application", "database", "api", "user", "admin", 
            "student", "teacher", "course", "subject", "elective", "semester",
            "prerequisite", "field", "property", "attribute", "name", "age", "id"
        }

    def split_sentences(self, text: str) -> List[str]:
        """Splits a paragraph into sentences."""
        # Simple regex split on '.', '!', '?'
        sentences = re.split(r'(?<=[.!?])\s+', text.strip())
        return [s for s in sentences if s]

    def tokenize(self, sentence: str) -> List[Token]:
        """Tokenizes a sentence into words, stripping punctuation."""
        # Find words and standard punctuation
        raw_tokens = re.findall(r'\b\w+\b|[.,!?;]', sentence)
        return [Token(t) for t in raw_tokens]

    def pos_tag(self, tokens: List[Token]) -> List[Token]:
        """Tags tokens with basic Parts of Speech using deterministic heuristics."""
        for token in tokens:
            word = token.lemma
            
            if not word.isalnum():
                token.pos_tag = "PUNCT"
            elif word in self.articles:
                token.pos_tag = "DET"
            elif word in self.prepositions:
                token.pos_tag = "PREP"
            elif word in self.conjunctions:
                token.pos_tag = "CONJ"
            elif word in self.pronouns:
                token.pos_tag = "PRON"
            elif word in self.verbs or word.endswith("s") and word[:-1] in self.verbs:
                token.pos_tag = "VERB"
            elif word in self.nouns or word.endswith("s") and word[:-1] in self.nouns:
                token.pos_tag = "NOUN"
            else:
                # Fallback heuristics
                if word.endswith("ly"):
                    token.pos_tag = "ADV"
                elif word.endswith("able") or word.endswith("ful") or word.endswith("ic"):
                    token.pos_tag = "ADJ"
                elif word.endswith("ing") or word.endswith("ed"):
                    token.pos_tag = "VERB"
                else:
                    # Default unknown words to NOUN in a software spec context
                    token.pos_tag = "NOUN"
                    
        return tokens

    def process(self, text: str) -> List[List[Token]]:
        """Runs the full NLP pipeline: split -> tokenize -> pos_tag"""
        sentences = self.split_sentences(text)
        result = []
        for sentence in sentences:
            tokens = self.tokenize(sentence)
            tagged = self.pos_tag(tokens)
            result.append(tagged)
        return result
