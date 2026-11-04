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
FILE: model.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from dataclasses import dataclass, field
from typing import List, Dict

@dataclass
class ProjectSnapshot:
    project_name: str = "AAYU Language"
    version: str = "1.0"
    milestone: str = "5A"
    phase: str = "BrainOS Foundation (Non-AI Orchestrator)"
    completed: List[str] = field(default_factory=list)
    frozen: List[str] = field(default_factory=list)
    matrix: Dict[str, str] = field(default_factory=dict)
    debt: List[str] = field(default_factory=list)
    branch: str = "feature/milestone-5"
    next_target: str = ""
    regression_risk: str = ""
