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
FILE: planner.py
PURPOSE: PlannerAgent for BrainOS v2 Pipeline
=============================================================================
"""

from typing import Dict, Any, List
import uuid

class PlannerAgent:
    """
    PlannerAgent takes the Intent IR and generates a step-by-step
    execution roadmap. It identifies system components and determines
    the optimal order of implementation.
    """
    def __init__(self):
        pass
        
    def execute(self, intent_data: Dict[str, Any]) -> Dict[str, Any]:
        print("[PlannerAgent] Generating execution plan...")
        
        entities = intent_data.get("entities", [])
        actions = intent_data.get("actions", [])
        
        # Determine components
        has_db = any(e.lower() in ["database", "db", "postgres", "mysql", "mongodb"] for e in entities)
        has_api = any(e.lower() in ["api", "backend", "server", "rest"] for e in entities)
        has_ui = any(e.lower() in ["ui", "frontend", "dashboard", "website", "client"] for e in entities)
        
        # Fallbacks based on actions
        if not has_api and any(a.lower() in ["deploy", "serve", "host"] for a in actions):
            has_api = True
            
        plan_id = str(uuid.uuid4())
        steps = []
        
        # Order of execution: Data Layer -> API Layer -> Presentation Layer
        if has_db:
            steps.append({
                "id": "step-db",
                "phase": "Data Layer",
                "description": "Initialize database schema and connection models.",
                "depends_on": []
            })
            
        if has_api:
            steps.append({
                "id": "step-api",
                "phase": "API Layer",
                "description": "Create API routes, controllers, and business logic.",
                "depends_on": ["step-db"] if has_db else []
            })
            
        if any(e.lower() in ["llm_client", "rag", "chatbot"] for e in entities):
            steps.append({
                "id": "step-ai",
                "phase": "AI Layer",
                "description": "Integrate LLM and vector database",
                "depends_on": ["step-api"] if has_api else []
            })
            
        if any(e.lower() in ["mobile", "ios", "android"] for e in entities):
            steps.append({
                "id": "step-mobile",
                "phase": "Mobile Backend Layer",
                "description": "Setup GraphQL API for mobile clients",
                "depends_on": ["step-api"] if has_api else []
            })
            
        if has_ui:
            steps.append({
                "id": "step-ui",
                "phase": "Presentation Layer",
                "description": "Build user interface and connect to backend APIs.",
                "depends_on": ["step-api"] if has_api else []
            })
            
        if not steps:
            # Generic script plan
            steps.append({
                "id": "step-core",
                "phase": "Core Logic",
                "description": "Implement the core logic and scripts.",
                "depends_on": []
            })
            
        return {
            "plan_id": plan_id,
            "original_intent": intent_data,
            "roadmap": steps,
            "metadata": {
                "total_phases": len(steps),
                "estimated_complexity": "Medium" if len(steps) > 1 else "Low"
            }
        }
