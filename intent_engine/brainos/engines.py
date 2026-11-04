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
FILE: engines.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""


from typing import List, Dict, Any

class Rule:
    def __init__(self, id: str, priority: int, when: Dict, recommend: Dict, reason: List[str], cost: str, complexity: str, scale: str, confidence: int, unless: Dict = None, avoid: List[str] = None):
        self.id = id
        self.priority = priority
        self.when = when
        self.unless = unless or {}
        self.recommend = recommend
        self.avoid = avoid or []
        self.reason = reason
        self.cost = cost
        self.complexity = complexity
        self.scale = scale
        self.confidence = confidence

class DecisionEngine:
    def __init__(self):
        self.rules = []

    def load_rules(self, rules_data: List[Dict]):
        for r in rules_data:
            self.rules.append(Rule(**r))
        self.rules.sort(key=lambda x: x.priority, reverse=True)

    def evaluate(self, context: Dict[str, Any]) -> List[Rule]:
        triggered_rules = []
        for rule in self.rules:
            # Check 'when' condition
            match = True
            for k, v in rule.when.items():
                if k not in context or context[k] != v:
                    # simplistic evaluation for prototype
                    if isinstance(v, str) and ('>' in v or '<' in v):
                        pass # handle operators in a real engine
                    else:
                        match = False
                        break
            
            # Check 'unless' condition
            if match and rule.unless:
                for k, v in rule.unless.items():
                    if k in context and context[k] == v:
                        match = False
                        break
            
            if match:
                triggered_rules.append(rule)
        
        return triggered_rules

class RecommendationEngine:
    def generate_recommendation(self, rules: List[Rule]) -> Dict:
        recommendation = {
            "technologies": [],
            "avoid": [],
            "reasons": [],
            "confidence_avg": 0,
            "cost_profile": "Medium",
            "complexity_profile": "Medium",
            "scale_profile": "Standard"
        }
        if not rules:
            return recommendation
            
        conf_sum = 0
        for r in rules:
            for cat, techs in r.recommend.items():
                recommendation["technologies"].extend(techs)
            recommendation["avoid"].extend(r.avoid)
            recommendation["reasons"].extend(r.reason)
            conf_sum += r.confidence
            
        recommendation["confidence_avg"] = conf_sum // len(rules)
        # simplistic cost/scale aggregation
        recommendation["cost_profile"] = rules[0].cost
        recommendation["complexity_profile"] = rules[0].complexity
        recommendation["scale_profile"] = rules[0].scale
        
        # Deduplicate
        recommendation["technologies"] = list(set(recommendation["technologies"]))
        recommendation["avoid"] = list(set(recommendation["avoid"]))
        recommendation["reasons"] = list(set(recommendation["reasons"]))
        
        return recommendation

class ArchitectureReviewEngine:
    def review(self, architecture: Dict) -> List[str]:
        problems = []
        techs = architecture.get("technologies", [])
        
        has_auth = any(t in ["JWT", "OAuth2", "Session"] for t in techs)
        has_cache = any(t in ["Redis", "Memcached"] for t in techs)
        has_logs = any(t in ["ELK", "Audit Logs"] for t in techs)
        
        if not has_auth:
            problems.append("Authentication Missing")
        if not has_cache:
            problems.append("Cache Missing (Performance Risk)")
        if not has_logs:
            problems.append("Audit Logs Missing")
            
        return problems

class ProductionReadinessScorer:
    def score(self, problems: List[str], rules: List[Rule]) -> Dict:
        base_security = 100
        base_perf = 100
        base_scale = 100
        base_maint = 100
        base_cost = 100
        
        for p in problems:
            if "Authentication" in p or "Log" in p:
                base_security -= 20
            if "Cache" in p:
                base_perf -= 15
                base_scale -= 10
                
        # Adjust based on scale rules
        if any(r.scale == "1M+ Users" for r in rules):
            base_scale += 5
            base_cost -= 15 # More scale = higher cost/lower cost efficiency
            base_maint -= 5
            
        overall = int((base_security + base_perf + base_scale + base_maint + base_cost) / 5)
        
        return {
            "Security": f"{max(0, min(100, base_security))}%",
            "Performance": f"{max(0, min(100, base_perf))}%",
            "Scalability": f"{max(0, min(100, base_scale))}%",
            "Maintainability": f"{max(0, min(100, base_maint))}%",
            "Cost Efficiency": f"{max(0, min(100, base_cost))}%",
            "Overall": f"{max(0, min(100, overall))}%"
        }
