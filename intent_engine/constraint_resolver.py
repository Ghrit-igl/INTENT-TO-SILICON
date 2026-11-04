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

class ConstraintResolver:
    def resolve(self, constraints_list):
        resolved = {
            "security": "standard",
            "performance": "standard",
            "availability": "99.9",
            "budget": "medium",
            "latency": "medium"
        }
        
        for constraint in constraints_list:
            c = constraint.lower()
            if "fast" in c or "high throughput" in c or "performance" in c:
                resolved["performance"] = "high"
                resolved["latency"] = "low"
            if "secure" in c or "encrypted" in c:
                resolved["security"] = "high"
            if "cheap" in c or "low cost" in c:
                resolved["budget"] = "low"
                
        return resolved
