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

class CostEngine:
    """
    Estimates the infrastructure cost based on the chosen architecture and scale.
    """
    def estimate(self, architecture: str, constraints: list[str]) -> dict:
        monthly_cost = 0
        breakdown = []
        
        if architecture == "Event-Sourced Actor Model":
            monthly_cost += 1500
            breakdown.append("Event Store Cluster: $800/mo")
            breakdown.append("Actor Compute Nodes: $700/mo")
        elif architecture == "Microservices with Distributed Transactions":
            monthly_cost += 2200
            breakdown.append("Kubernetes Cluster: $1200/mo")
            breakdown.append("Distributed Database: $1000/mo")
        elif architecture == "Edge-Cached Serverless Monolith":
            monthly_cost += 300
            breakdown.append("Serverless Functions: $200/mo")
            breakdown.append("Global Edge Cache: $100/mo")
        else:
            monthly_cost += 50
            breakdown.append("Standard VPS: $50/mo")
            
        if "High Security" in constraints:
            monthly_cost += 200
            breakdown.append("WAF & KMS: $200/mo")
            
        return {
            "estimated_monthly_usd": monthly_cost,
            "breakdown": breakdown
        }
