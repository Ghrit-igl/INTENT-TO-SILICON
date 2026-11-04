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
FILE: verifier.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from abc import ABC, abstractmethod
from typing import List
from .graph import IntentGraph

class VerificationRule(ABC):
    @abstractmethod
    def evaluate(self, graph: IntentGraph) -> dict:
        """Returns a dict with 'passes', 'warnings', 'failures', and 'score_penalty'"""
        pass

class EmptyEntityRule(VerificationRule):
    def evaluate(self, graph: IntentGraph) -> dict:
        passes = []
        failures = []
        penalty = 0
        
        for entity_name, data in graph.entities.items():
            if not data["fields"]:
                failures.append(f"[FAIL] {entity_name} entity has no fields")
                penalty += 20
            else:
                passes.append(f"[PASS] {entity_name} entity valid ({len(data['fields'])} fields)")
                
        return {"passes": passes, "warnings": [], "failures": failures, "score_penalty": penalty}

class RelationshipRule(VerificationRule):
    def evaluate(self, graph: IntentGraph) -> dict:
        warnings = []
        penalty = 0
        
        if len(graph.entities) > 1:
            has_relations = False
            for data in graph.entities.values():
                if data.get("relations"):
                    has_relations = True
                    break
            if not has_relations:
                warnings.append("[WARN] No explicit relationships defined between entities")
                penalty += 10
            
        return {"passes": [], "warnings": warnings, "failures": [], "score_penalty": penalty}

class RelationshipIntegrityRule(VerificationRule):
    def evaluate(self, graph: IntentGraph) -> dict:
        passes = []
        failures = []
        penalty = 0
        
        for entity_name, data in graph.entities.items():
            for rel in data.get("relations", []):
                target = rel["target"]
                if target not in graph.entities:
                    failures.append(f"[FAIL] {entity_name} {rel['relation']} '{target}', but '{target}' does not exist.")
                    penalty += 20
                else:
                    passes.append(f"[PASS] {entity_name} -> {target} valid")
                    
        return {"passes": passes, "warnings": [], "failures": failures, "score_penalty": penalty}

class TaskIntegrityRule(VerificationRule):
    def evaluate(self, graph: IntentGraph) -> dict:
        passes = []
        failures = []
        penalty = 0
        
        for entity_name, data in graph.entities.items():
            for task in data.get("tasks", []):
                target = task["target"]
                if target not in graph.entities:
                    failures.append(f"[FAIL] {entity_name} can {task['action']} '{target}', but '{target}' does not exist.")
                    penalty += 20
                else:
                    passes.append(f"[PASS] {entity_name} task {task['action']}_{target.lower()} valid")
                    
        return {"passes": passes, "warnings": [], "failures": failures, "score_penalty": penalty}

class VerificationReport:
    def __init__(self, score: int, confidence: int, passes: List[str], warnings: List[str], failures: List[str]):
        self.score = score
        self.confidence = confidence
        self.passes = passes
        self.warnings = warnings
        self.failures = failures
        
    @property
    def status(self) -> str:
        if self.score >= 80:
            return "READY"
        elif self.score >= 60:
            return "NEEDS REVIEW"
        else:
            return "BLOCKED"

class VerificationEngine:
    def __init__(self):
        # A modular plugin architecture for verification rules
        self.rules: List[VerificationRule] = [
            EmptyEntityRule(),
            RelationshipRule(),
            RelationshipIntegrityRule(),
            TaskIntegrityRule()
        ]
        
    def verify(self, graph: IntentGraph) -> VerificationReport:
        score = 100
        passes = []
        warnings = []
        failures = []
        
        # Calculate baseline confidence from the graph's intent sources
        all_confidences = []
        for data in graph.entities.values():
            for intent in data["source_intents"]:
                all_confidences.append(intent.confidence)
                
        confidence = int((sum(all_confidences) / len(all_confidences)) * 100) if all_confidences else 100
        
        for rule in self.rules:
            result = rule.evaluate(graph)
            passes.extend(result["passes"])
            warnings.extend(result["warnings"])
            failures.extend(result["failures"])
            score -= result["score_penalty"]
            
        score = max(0, score)
        return VerificationReport(score, confidence, passes, warnings, failures)
