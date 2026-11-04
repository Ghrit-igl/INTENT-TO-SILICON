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
FILE: architecture_generator.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List, Dict
from intent_engine.graphs.intent_graph import IntentGraph, IntentGraphNode
from intent_engine.graphs.architecture_graph import ArchitectureGraph, RecordArchNode, InterfaceArchNode, ExtensionArchNode

class ArchitectureGenerator:
    """
    Deterministically transforms the Intent Graph into the AAYU Architecture Graph.
    This enforces the Intent -> Architecture separation rule.
    """
    def __init__(self, intent_graph: IntentGraph):
        self.intent_graph = intent_graph
        self.arch_graph = ArchitectureGraph()

    def generate(self) -> ArchitectureGraph:
        # Process each root node in the intent graph
        for node in self.intent_graph.root_nodes:
            self._process_node(node)
            
        return self.arch_graph

    def _process_node(self, node: IntentGraphNode):
        node_type = node.ir_node.node_type
        
        if node_type == "entity":
            # Map Entity -> Record
            record = RecordArchNode(name=node.ir_node.name)
            
            # Map Fields -> Record properties
            if hasattr(node.ir_node, 'fields'):
                for field in node.ir_node.fields:
                    # Default to Text if type unknown
                    record.fields[field.name] = field.field_type or "Text"
                    
            self.arch_graph.add_node(record)
            
        elif node_type == "action":
            # Map Action -> Extension/Interface
            # Example: "Borrow Book"
            # -> Record Borrow
            # -> Interface Borrowable
            # -> Extension Student
            if node.ir_node.target:
                # E.g. Action: Borrow, Target: Book
                action_name = node.ir_node.action.capitalize()
                
                # 1. Create a Record for the action itself (e.g. Record Borrow)
                action_record = RecordArchNode(name=action_name)
                action_record.fields["id"] = "Number"
                self.arch_graph.add_node(action_record)
                
                # 2. Create an Interface for the capability (e.g. Interface Borrowable)
                interface = InterfaceArchNode(name=f"{action_name}able")
                interface.methods.append(f"do_{action_name.lower()}")
                self.arch_graph.add_node(interface)
                
                # 3. Extend the actor (if known, else generic)
                actor = node.ir_node.actor
                if actor:
                    extension = ExtensionArchNode(name=f"{actor}{action_name}", target=actor)
                    extension.methods.append(f"do_{action_name.lower()}")
                    self.arch_graph.add_node(extension)

        # Recurse
        for child in node.outgoing:
            self._process_node(child)
