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

class SemanticNode:
    def __init__(self, token, pos):
        self.token = token
        self.pos = pos
        self.dependencies = []

    def add_dependency(self, rel_type, target_node):
        self.dependencies.append({"rel": rel_type, "target": target_node})

class SemanticGraph:
    def __init__(self):
        self.nodes = []
        
    def build_from_tagged(self, tagged_tokens):
        # A simple linear parse tree mapping for offline offline intent reasoning
        nodes = [SemanticNode(t, p) for t, p in tagged_tokens]
        # Basic noun-verb linking
        current_verb = None
        for node in nodes:
            if node.pos == "VERB":
                current_verb = node
            elif node.pos == "NOUN" and current_verb:
                current_verb.add_dependency("DOBJ", node)
        
        self.nodes = nodes
        return self
