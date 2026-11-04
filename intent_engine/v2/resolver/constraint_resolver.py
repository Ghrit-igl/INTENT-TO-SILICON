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
FILE: constraint_resolver.py
PURPOSE: ConstraintResolver for Intent Engine v2
=============================================================================
"""

from typing import Dict, Any

class ConstraintResolver:
    def __init__(self):
        pass
        
    def resolve(self, enriched_graph: Dict[str, Any]) -> Dict[str, Any]:
        resolved = enriched_graph.copy()
        
        # Check constraints (e.g. can't deploy without a backend)
        entities = resolved.get("entities", [])
        actions = resolved.get("actions", [])
        
        if "deploy" in actions and "api" not in entities:
            # Infer an API is needed if they want to deploy something like a CRM
            if "crm" in entities:
                resolved["entities"].append("api")
                
        return resolved
