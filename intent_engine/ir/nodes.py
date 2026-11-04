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
FILE: nodes.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import Any, Dict, List, Optional
from pydantic import BaseModel, Field

class IntentNode(BaseModel):
    """
    Base class for all Intent IR nodes.
    Represents an atomic piece of human intent mapped into a structured format.
    """
    node_type: str
    source_text: str = Field(description="The original natural language text that generated this intent.")
    confidence: float = Field(default=1.0, ge=0.0, le=1.0)


class FieldNode(IntentNode):
    """
    Represents a property of an Entity (e.g., 'age', 'name', 'address').
    """
    node_type: str = "field"
    name: str
    field_type: Optional[str] = None


class EntityNode(IntentNode):
    """
    Represents a core business concept (e.g., 'User', 'Library', 'Book').
    """
    node_type: str = "entity"
    name: str
    fields: List[FieldNode] = Field(default_factory=list)


class RelationshipNode(IntentNode):
    """
    Represents a connection between two Entities.
    """
    node_type: str = "relationship"
    source: str
    relation: str
    target: str
    cardinality: Optional[str] = None


class ConstraintNode(IntentNode):
    """
    Represents a business rule or validation (e.g., 'Age must be over 18').
    """
    node_type: str = "constraint"
    target: str
    rule_description: str


class ActionNode(IntentNode):
    """
    Represents a task, capability, or action.
    """
    node_type: str = "action"
    actor: str
    action: str
    target: Optional[str] = None


class FlowNode(IntentNode):
    """
    Represents a sequence of actions or logical workflow.
    """
    node_type: str = "flow"
    name: str
    steps: List[ActionNode] = Field(default_factory=list)


class Intent(BaseModel):
    """
    The root node of the Intent IR.
    Contains all parsed entities, relationships, constraints, and flows.
    """
    original_intent: str
    entities: List[EntityNode] = Field(default_factory=list)
    relationships: List[RelationshipNode] = Field(default_factory=list)
    constraints: List[ConstraintNode] = Field(default_factory=list)
    actions: List[ActionNode] = Field(default_factory=list)
    flows: List[FlowNode] = Field(default_factory=list)
