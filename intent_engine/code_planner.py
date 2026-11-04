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
FILE: code_planner.py
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
from intent_engine.graphs.architecture_graph import ArchitectureGraph, RecordArchNode, InterfaceArchNode, ExtensionArchNode, ModuleArchNode

class FilePlan:
    def __init__(self, filename: str):
        self.filename = filename
        self.content = ""

class CodePlanner:
    """
    Takes the Architecture Graph and plans out the AAYU physical files.
    """
    def __init__(self, arch_graph: ArchitectureGraph):
        self.arch_graph = arch_graph
        self.files: Dict[str, FilePlan] = {}

    def plan(self) -> List[FilePlan]:
        for module_name, module_node in self.arch_graph.modules.items():
            self._plan_module(module_name, module_node)
        return list(self.files.values())

    def _plan_module(self, module_name: str, module_node: ModuleArchNode):
        records = []
        interfaces = []
        extensions = []
        
        for comp in module_node.components:
            if isinstance(comp, RecordArchNode):
                records.append(comp)
            elif isinstance(comp, InterfaceArchNode):
                interfaces.append(comp)
            elif isinstance(comp, ExtensionArchNode):
                extensions.append(comp)
                
        # Generate AAYU code for records
        if records:
            file_plan = FilePlan(f"{module_name}_models.aayu")
            content = ""
            for rec in records:
                content += f"entity {rec.name}\nhas\n"
                for field_name, field_type in rec.fields.items():
                    content += f"    {field_name} : {field_type}\n"
                content += "end.\n\n"
            file_plan.content = content
            self.files[file_plan.filename] = file_plan

        # Generate AAYU code for interfaces
        if interfaces:
            file_plan = FilePlan(f"{module_name}_interfaces.aayu")
            content = ""
            for interface in interfaces:
                content += f"interface {interface.name}\nhas\n"
                for method in interface.methods:
                    content += f"    fn {method}()\n"
                content += "end.\n\n"
            file_plan.content = content
            self.files[file_plan.filename] = file_plan

        # Generate AAYU code for extensions
        if extensions:
            file_plan = FilePlan(f"{module_name}_extensions.aayu")
            content = ""
            for ext in extensions:
                content += f"extend {ext.target} with {ext.name}\nhas\n"
                for method in ext.methods:
                    content += f"    fn {method}()\n    do\n        # TODO: Implement {method}\n    end.\n"
                content += "end.\n\n"
            file_plan.content = content
            self.files[file_plan.filename] = file_plan
