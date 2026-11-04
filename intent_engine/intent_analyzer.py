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
FILE: intent_analyzer.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from intent_engine.dictionary_loader import DictionaryLoader

class IntentAnalyzer:
    def __init__(self):
        self.loader = DictionaryLoader()
        self.domains = self.loader.get_all_domains()
        self.features = self.loader.get_all_features()
        
    def analyze(self, user_prompt: str) -> dict:
        prompt_lower = user_prompt.lower()
        
        detected_domain = None
        domain_data = None
        
        # Detect primary domain
        for d_name, d_info in self.domains.items():
            for alias in d_info.get("aliases", []):
                if alias in prompt_lower:
                    detected_domain = d_name
                    domain_data = d_info
                    break
            if detected_domain:
                break
                
        if not detected_domain:
            return {
                "status": "UNKNOWN_DOMAIN",
                "message": "Sorry, I couldn't recognize a supported domain in your prompt."
            }
            
        # Extract features mentioned in the prompt
        detected_features = []
        for f_name, f_info in self.features.items():
            for alias in f_info.get("aliases", []):
                if alias in prompt_lower:
                    if f_name not in detected_features:
                        detected_features.append(f_name)
                        
        # Identify missing features (cross questions needed)
        suggested = domain_data.get("suggested_features", [])
        missing_features = [f for f in suggested if f not in detected_features]
        
        questions = []
        for mf in missing_features:
            f_info = self.features.get(mf)
            if f_info and "cross_question" in f_info:
                questions.append(f_info["cross_question"])
                
        return {
            "status": "SUCCESS",
            "domain": detected_domain,
            "entities": domain_data.get("entities", []),
            "detected_features": detected_features,
            "missing_features": missing_features,
            "questions": questions
        }

if __name__ == "__main__":
    analyzer = IntentAnalyzer()
    
    test_prompts = [
        "Build a College LMS",
        "I need a CRM for my travel agency",
        "E-commerce site with stripe payments"
    ]
    
    for p in test_prompts:
        print(f"\\n--- Analyzing: '{p}' ---")
        result = analyzer.analyze(p)
        if result["status"] == "SUCCESS":
            print(f"Detected Domain: {result['domain']}")
            print("Entities:")
            for e in result["entities"]:
                print(f"- {e}")
            print("\\nQuestions:")
            for i, q in enumerate(result["questions"], 1):
                print(f"{i}. {q}")
        else:
            print(result["message"])
