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

class POSTagger:
    def __init__(self):
        # A small offline lexicon for AAYU specific semantics
        self.lexicon = {
            "create": "VERB",
            "build": "VERB",
            "make": "VERB",
            "database": "NOUN",
            "api": "NOUN",
            "fast": "ADJ",
            "secure": "ADJ",
            "a": "DET",
            "the": "DET",
            "with": "PREP"
        }

    def tag(self, tokens: list) -> list:
        tagged = []
        for token in tokens:
            lower = token.lower()
            if lower in self.lexicon:
                tagged.append((token, self.lexicon[lower]))
            elif lower.endswith('ly'):
                tagged.append((token, "ADV"))
            elif lower.endswith('ing') or lower.endswith('ed'):
                tagged.append((token, "VERB"))
            elif lower.isalnum():
                tagged.append((token, "NOUN"))
            else:
                tagged.append((token, "PUNCT"))
        return tagged
