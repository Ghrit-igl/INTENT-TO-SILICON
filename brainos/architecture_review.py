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

from security_review import SecurityReview
from performance_review import PerformanceReview

class ArchitectureReview:
    """
    Aggregates all reviews to provide a final Architecture Score.
    """
    def generate_report(self, constraints: list[str], architecture: str, entities: dict) -> dict:
        sr = SecurityReview().review(entities)
        pr = PerformanceReview().review(constraints, architecture)
        
        overall_score = (sr["score"] + pr["score"]) // 2
        
        return {
            "overall_score": overall_score,
            "security": sr,
            "performance": pr
        }
