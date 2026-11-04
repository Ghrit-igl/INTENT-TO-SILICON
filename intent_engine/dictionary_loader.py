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
FILE: dictionary_loader.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

import os
import json

class DictionaryLoader:
    def __init__(self, kb_path=None):
        if kb_path is None:
            kb_path = os.path.join(os.path.dirname(__file__), "knowledge_base")
        self.kb_path = kb_path
        self.domains = self._load_json("domains.json")
        self.features = self._load_json("features.json")
        
    def _load_json(self, filename):
        filepath = os.path.join(self.kb_path, filename)
        if not os.path.exists(filepath):
            return {}
        with open(filepath, "r", encoding="utf-8") as f:
            return json.load(f)

    def get_all_domains(self):
        return self.domains

    def get_domain(self, domain_name):
        return self.domains.get(domain_name)

    def get_all_features(self):
        return self.features

    def get_feature(self, feature_name):
        return self.features.get(feature_name)
