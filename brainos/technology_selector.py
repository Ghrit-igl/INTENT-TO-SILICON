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

class TechnologySelector:
    def select(self, requirements: list) -> dict:
        tech = []
        if "frontend" in requirements:
            tech.append("react")
        if "backend" in requirements:
            tech.append("fastapi")
        if "database" in requirements:
            tech.append("postgresql")
        return {"selected": tech}
