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

class DependencyGenerator:
    def resolve_dependencies(self, architecture: dict) -> dict:
        tech_stack = architecture.get('technologies', [])
        deps = {}
        for tech in tech_stack:
            if tech == "react":
                deps["react"] = "^18.2.0"
                deps["react-dom"] = "^18.2.0"
            elif tech == "fastapi":
                deps["fastapi"] = "^0.100.0"
                deps["uvicorn"] = "^0.23.0"
        return deps
