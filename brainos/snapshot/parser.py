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

from .model import ProjectSnapshot
import os

class SnapshotParser:
    def parse(self, filepath: str) -> ProjectSnapshot:
        """
        Parses PROJECT_SNAPSHOT.md into the Snapshot Model.
        """
        model = ProjectSnapshot()
        
        if not os.path.exists(filepath):
            return model
            
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
            
        # Very basic parsing for MVP (just reading line by line based on headers)
        # This is a stub that should be replaced with a robust Markdown parser.
        current_section = None
        
        for line in lines:
            line = line.strip()
            if not line:
                continue
                
            if line.startswith('## '):
                current_section = line[3:].strip()
                continue
                
            if current_section == "Current State":
                if line.startswith("**Milestone** :"):
                    model.milestone = line.split(":", 1)[1].strip()
                elif line.startswith("**Phase** :"):
                    model.phase = line.split(":", 1)[1].strip()
            elif current_section == "Completed":
                if line.startswith("✓"):
                    model.completed.append(line[1:].strip())
            elif current_section == "Frozen":
                if line.startswith("✓"):
                    model.frozen.append(line[1:].strip())
            elif current_section == "Architecture Freeze Matrix":
                if line.startswith("|") and "Status" not in line and "---" not in line:
                    parts = [p.strip() for p in line.split("|") if p.strip()]
                    if len(parts) >= 2:
                        model.matrix[parts[0]] = parts[1]
            elif current_section == "Technical Debt":
                if line.startswith("•"):
                    model.debt.append(line[1:].strip())
            elif current_section == "Current Branch":
                if not line.startswith("─"):
                    model.branch = line
            elif current_section == "Next Target":
                if not line.startswith("─"):
                    model.next_target = line
            elif current_section == "Regression Risk":
                if not line.startswith("─"):
                    model.regression_risk = line
                    
        return model
