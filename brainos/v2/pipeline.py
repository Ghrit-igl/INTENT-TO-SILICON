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
FILE: pipeline.py
PURPOSE: Core Pipeline Orchestrator for BrainOS v2
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
"""

from typing import Dict, Any, List
from .agents.planner import PlannerAgent
from .agents.architect import ArchitectAgent
from .agents.reviewer import ReviewerAgent
from .agents.optimizer import OptimizerAgent
from .agents.validator import ValidatorAgent
from .agents.executor import ExecutorAgent

class BrainOSPipeline:
    """
    BrainOSPipeline orchestrates the sequence of agents to convert
    high-level user intent into fully validated code.
    
    Order:
    Human Prompt -> Intent Engine -> Planner -> Architect -> Reviewer -> Optimizer -> Validator -> Executor -> Compiler
    """
    
    def __init__(self):
        self.planner = PlannerAgent()
        self.architect = ArchitectAgent()
        self.reviewer = ReviewerAgent()
        self.optimizer = OptimizerAgent()
        self.validator = ValidatorAgent()
        self.executor = ExecutorAgent()
        
    def process_intent(self, parsed_intent_graph: Dict[str, Any]) -> Dict[str, Any]:
        """
        Executes the full pipeline on a parsed intent graph.
        """
        print("[BrainOS v2] Starting Pipeline execution...")
        
        # 1. Planning Phase
        plan = self.planner.execute(parsed_intent_graph)
        
        # 2. Architecture Phase
        architecture = self.architect.execute(plan)
        
        # 3. Review Phase
        reviewed_architecture = self.reviewer.execute(architecture)
        
        # 4. Optimization Phase
        optimized_architecture = self.optimizer.execute(reviewed_architecture)
        
        # 5. Validation Phase (Dry-Run / Dry-Compile)
        is_valid = self.validator.execute(optimized_architecture)
        
        if not is_valid:
            raise ValueError("[BrainOS v2] Pipeline failed during validation phase. Code will not be written to disk.")
            
        # 6. Execution Phase (Code Generation & Disk Writing)
        result = self.executor.execute(optimized_architecture)
        
        print("[BrainOS v2] Pipeline execution complete.")
        return result
