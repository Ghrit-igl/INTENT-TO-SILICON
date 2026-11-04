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
FILE: architecture_graph.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List, Dict, Optional
from pydantic import BaseModel

class ArchNode(BaseModel):
    node_type: str
    name: str

class RecordArchNode(ArchNode):
    node_type: str = "record"
    fields: Dict[str, str] = {}

class InterfaceArchNode(ArchNode):
    node_type: str = "interface"
    methods: List[str] = []

class ExtensionArchNode(ArchNode):
    node_type: str = "extension"
    target: str
    methods: List[str] = []

class ModuleArchNode(ArchNode):
    node_type: str = "module"
    components: List[ArchNode] = []

class ArchitectureGraph:
    def __init__(self):
        self.modules: Dict[str, ModuleArchNode] = {}
        self.root = ModuleArchNode(name="main")
        self.modules["main"] = self.root

    def add_node(self, node: ArchNode, module: str = "main"):
        if module not in self.modules:
            self.modules[module] = ModuleArchNode(name=module)
        self.modules[module].components.append(node)
