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
FILE: knowledge_graph.py
PURPOSE: KnowledgeGraph for Intent Engine v2
=============================================================================
"""

from typing import Dict, Any

class KnowledgeGraph:
    def __init__(self):
        self.memory = {}
        
    def update(self, s_graph: Dict[str, Any]):
        # Store context for future prompts
        for entity in s_graph.get("entities", []):
            self.memory[entity] = self.memory.get(entity, 0) + 1
            
    def enrich(self, s_graph: Dict[str, Any]) -> Dict[str, Any]:
        enriched = s_graph.copy()
        
        # Add inferred entities based on domain knowledge
        entities_lower = [e.lower() for e in s_graph.get("entities", [])]
        
        # Web / SaaS Apps
        if any(app in entities_lower for app in ["crm", "blog", "ecommerce", "saas", "dashboard", "portal"]):
            for comp in ["database", "api", "ui", "auth"]:
                if comp not in enriched["entities"]:
                    enriched["entities"].append(comp)
                    
        # API / Microservices
        if "api" in entities_lower or "microservice" in entities_lower:
            for comp in ["router", "controller", "service", "database"]:
                if comp not in enriched["entities"]:
                    enriched["entities"].append(comp)
                    
        # Mobile Apps
        if any(app in entities_lower for app in ["mobile", "ios", "android", "app"]):
            for comp in ["api", "client", "auth"]:
                if comp not in enriched["entities"]:
                    enriched["entities"].append(comp)
                    
        # AI / ML Apps
        if any(app in entities_lower for app in ["ai", "ml", "chatbot", "rag"]):
            for comp in ["vector_db", "llm_client", "api", "ui"]:
                if comp not in enriched["entities"]:
                    enriched["entities"].append(comp)
                
        return enriched
