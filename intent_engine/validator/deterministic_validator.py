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
FILE: deterministic_validator.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List, Dict, Any
from ..graphs.architecture_graph import ArchitectureGraph

class DeterministicValidator:
    """
    A rule engine that validates the Architecture Graph deterministically
    before allowing Code Generation or BrainOS execution.
    """
    def validate(self, arch_graph: ArchitectureGraph) -> bool:
        return len(self.get_errors(arch_graph)) == 0

    def get_errors(self, arch_graph: ArchitectureGraph) -> List[str]:
        errors = []
        
        # Rule 1: Interfaces must have at least one method
        for name, data in arch_graph.interfaces.items():
            if not data.get("methods"):
                errors.append(f"Interface '{name}' must declare at least one method.")
                
        # Rule 2: Extensions must target valid records and interfaces
        for ext in arch_graph.extensions:
            record = ext.get("target_record")
            interface = ext.get("target_interface")
            if record not in arch_graph.records:
                errors.append(f"Extension targets undefined record '{record}'.")
            if interface not in arch_graph.interfaces:
                errors.append(f"Extension targets undefined interface '{interface}'.")
                
        return errors
