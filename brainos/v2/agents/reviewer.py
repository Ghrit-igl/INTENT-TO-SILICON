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
FILE: reviewer.py
PURPOSE: ReviewerAgent for BrainOS v2 Pipeline
=============================================================================
"""

from typing import Dict, Any

class ReviewerAgent:
    """
    ReviewerAgent analyzes the generated architecture for security flaws,
    scalability bottlenecks, and maintainability issues before optimization.
    """
    def __init__(self):
        pass
        
    def execute(self, arch_data: Dict[str, Any]) -> Dict[str, Any]:
        print("[ReviewerAgent] Reviewing architecture for security and scalability...")
        
        architecture = arch_data.get("architecture", {})
        modules = architecture.get("modules", {})
        
        reviews = []
        
        # Heuristics checks
        if "database" in modules and "api" in modules:
            reviews.append({
                "type": "security",
                "severity": "high",
                "message": "Ensure database credentials are read from environment variables, not hardcoded."
            })
            
        if "api" in modules:
            reviews.append({
                "type": "scalability",
                "severity": "medium",
                "message": "API layer should be stateless to allow horizontal scaling."
            })
            
        arch_data["reviews"] = reviews
        return arch_data
