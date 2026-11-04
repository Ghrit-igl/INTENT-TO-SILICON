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
FILE: semantic.py
PURPOSE: SemanticAnalyzer for Intent Engine v2
=============================================================================
"""

from typing import List, Tuple, Dict, Any

class SemanticAnalyzer:
    def __init__(self):
        # Expanded dictionaries for heuristics
        self.action_keywords = {"build", "create", "generate", "deploy", "host", "add", "make", "setup", "initialize", "design"}
        self.entity_keywords = {
            "crm", "blog", "app", "api", "database", "ui", "frontend", "backend",
            "ecommerce", "saas", "dashboard", "portal", "mobile", "ios", "android",
            "microservice", "ai", "ml", "chatbot", "rag"
        }
        self.req_keywords = {"authentication", "auth", "kubernetes", "k8s", "secure", "fast", "scalable", "docker", "serverless"}

    def analyze(self, token_groups: List[List[str]]) -> Tuple[List[str], List[str], Dict[str, str]]:
        entities = set()
        actions = set()
        requirements = {}

        for group in token_groups:
            for word in group:
                if word in self.action_keywords:
                    actions.add(word)
                elif word in self.entity_keywords:
                    entities.add(word)
                elif "blog" in word:
                    entities.add("blog")
                elif word in self.req_keywords:
                    if word in ["authentication", "auth", "secure"]:
                        requirements["security"] = "high"
                    elif word in ["kubernetes", "k8s"]:
                        requirements["deployment"] = "kubernetes"
                        requirements["scalability"] = "high"

        return list(entities), list(actions), requirements
