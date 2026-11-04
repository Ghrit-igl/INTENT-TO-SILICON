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
FILE: tokenizer.py
PURPOSE: Tokenizer for Intent Engine v2
=============================================================================
"""

import re
from typing import List

class Tokenizer:
    def __init__(self):
        pass
        
    def tokenize(self, prompt: str) -> List[str]:
        # Split on " and " to handle multi-intent prompts
        intents = re.split(r'\s+and\s+', prompt, flags=re.IGNORECASE)
        
        tokens = []
        for intent in intents:
            # simple word tokenization for heuristics
            words = re.findall(r'\b\w+\b', intent.lower())
            tokens.append(words)
            
        return tokens
