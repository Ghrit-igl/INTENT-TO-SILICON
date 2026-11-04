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
FILE: intents.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from dataclasses import dataclass

@dataclass
class Intent:
    confidence: float
    source_text: str

@dataclass
class DefineEntityIntent(Intent):
    name: str

@dataclass
class DefineFieldIntent(Intent):
    entity_name: str
    field_name: str

@dataclass
class DefineRelationshipIntent(Intent):
    source: str
    relation: str
    target: str

@dataclass
class DefineTaskIntent(Intent):
    actor: str
    action: str
    target: str
