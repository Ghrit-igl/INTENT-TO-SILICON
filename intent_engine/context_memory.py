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

class ContextMemory:
    def __init__(self):
        self.history = []
        self.active_constraints = set()
        
    def add_interaction(self, prompt, ir):
        self.history.append({"prompt": prompt, "ir": ir})
        for c in ir.constraints:
            self.active_constraints.add(c)
            
    def get_contextual_constraints(self):
        return list(self.active_constraints)
