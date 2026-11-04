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
FILE: generator.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from .graph import IntentGraph
from .verifier import VerificationReport

class GenerationResult:
    def __init__(self, code: str, metadata: dict):
        self.code = code
        self.metadata = metadata

class AayuGenerator:
    def generate(self, graph: IntentGraph, report: VerificationReport) -> GenerationResult:
        # B-4 Architecture Lock
        if report.status != "READY":
            raise Exception(f"Architecture Lock: Cannot generate code when status is '{report.status}'")
            
        lines = []
        metadata = {
            "entities_generated": len(graph.entities),
            "entity_details": {},
            "verification_score": report.score,
            "confidence": report.confidence
        }
        
        # B-5 Code Generation
        for entity_name, data in graph.entities.items():
            fields = list(data["fields"])
            
            metadata["entity_details"][entity_name] = {
                "fields_count": len(fields),
                "relations": [],
                "tasks": []
            }
            
            for rel in data.get("relations", []):
                # E.g., target="Library", field_name="library"
                field_name = rel["target"].lower()
                if field_name not in fields:
                    fields.append(field_name)
                    
                metadata["entity_details"][entity_name]["relations"].append({
                    "relation": rel["relation"],
                    "target": rel["target"],
                    "mapped_field": field_name
                })
            
            lines.append(f"record {entity_name}.")
            for field in fields:
                lines.append(f"    {field}")
            lines.append("end.")
            lines.append("")
            
            for task in data.get("tasks", []):
                task_name = f"{task['action']}_{task['target'].lower()}"
                actor_lower = entity_name.lower()
                target_lower = task['target'].lower()
                
                metadata["entity_details"][entity_name]["tasks"].append(task_name)
                
                lines.append(f"task {task_name} with {actor_lower} and {target_lower}.")
                lines.append(f"    show \"Executing {task_name}\".")
                lines.append("end.")
                lines.append("")
            
        return GenerationResult("\n".join(lines), metadata)
