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
FILE: intent_graph.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List, Dict, Set, Optional
from ..ir.nodes import Intent, EntityNode, FieldNode, RelationshipNode, ConstraintNode, ActionNode

class IntentGraphNode:
    def __init__(self, ir_node):
        self.ir_node = ir_node
        self.incoming: List['IntentGraphNode'] = []
        self.outgoing: List['IntentGraphNode'] = []

    def add_edge_to(self, other: 'IntentGraphNode'):
        if other not in self.outgoing:
            self.outgoing.append(other)
        if self not in other.incoming:
            other.incoming.append(self)


class IntentGraph:
    """
    Converts the flat Intent IR into a navigatable dependency graph.
    This graph feeds directly into BrainOS.
    """
    def __init__(self, intent_ir: Intent):
        self.ir = intent_ir
        self.nodes: Dict[str, IntentGraphNode] = {}
        self.root_nodes: List[IntentGraphNode] = []
        self._build_graph()

    def _build_graph(self):
        # 1. Create nodes for all entities
        for entity in self.ir.entities:
            node = IntentGraphNode(entity)
            self.nodes[f"entity_{entity.name}"] = node
            self.root_nodes.append(node)
            
            # Fields are structurally attached inside EntityNode in IR,
            # but we can also treat them as dependent nodes in the graph
            for field in entity.fields:
                field_node = IntentGraphNode(field)
                self.nodes[f"field_{entity.name}_{field.name}"] = field_node
                node.add_edge_to(field_node)

        # 2. Add Relationships
        for rel in self.ir.relationships:
            rel_node = IntentGraphNode(rel)
            self.nodes[f"rel_{rel.source}_{rel.relation}_{rel.target}"] = rel_node
            
            # Source entity -> Relationship -> Target entity
            source_key = f"entity_{rel.source}"
            target_key = f"entity_{rel.target}"
            
            if source_key in self.nodes:
                self.nodes[source_key].add_edge_to(rel_node)
            if target_key in self.nodes:
                rel_node.add_edge_to(self.nodes[target_key])

        # 3. Add Actions (attached to target entities if applicable)
        for act in self.ir.actions:
            act_node = IntentGraphNode(act)
            action_id = f"action_{act.action}_{act.target or 'system'}"
            self.nodes[action_id] = act_node
            
            if act.target:
                target_key = f"entity_{act.target}"
                if target_key in self.nodes:
                    # The entity is a dependency for the action to exist
                    self.nodes[target_key].add_edge_to(act_node)
            else:
                self.root_nodes.append(act_node)
                
        # 4. Add Constraints
        for constraint in self.ir.constraints:
            const_node = IntentGraphNode(constraint)
            self.nodes[f"constraint_{constraint.target}_{hash(constraint.rule_description)}"] = const_node
            
            # Connect constraint to its target (entity or action)
            target_entity_key = f"entity_{constraint.target}"
            if target_entity_key in self.nodes:
                self.nodes[target_entity_key].add_edge_to(const_node)

    def print_graph(self, node: IntentGraphNode, depth: int = 0, visited: Set[IntentGraphNode] = None):
        if visited is None:
            visited = set()
            
        if node in visited:
            return
            
        visited.add(node)
        
        indent = "  " * depth
        node_type = node.ir_node.node_type
        
        if node_type == "entity":
            print(f"{indent}[Entity] {node.ir_node.name}")
        elif node_type == "field":
            print(f"{indent} - [Field] {node.ir_node.name}")
        elif node_type == "action":
            print(f"{indent}⚡ [Action] {node.ir_node.action} on {node.ir_node.target}")
        elif node_type == "relationship":
            print(f"{indent}↔ [Relation] {node.ir_node.relation} to {node.ir_node.target}")
        elif node_type == "constraint":
            print(f"{indent}⚠ [Constraint] {node.ir_node.rule_description}")
            
        for child in node.outgoing:
            self.print_graph(child, depth + 1, visited)
