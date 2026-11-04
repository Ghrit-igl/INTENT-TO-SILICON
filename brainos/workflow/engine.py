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
FILE: engine.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from ..workflow.states import WorkflowState
from ..planner.planner import Planner
from ..task.graph import TaskGraph
from ..task.queue import TaskQueue
from ..guard.freeze import FreezeGuard
from ..guard.decision_log import DecisionGuard
from ..executor.executor import ExecutorProvider, HumanApprovalExecutor
from ..critic.critic import Critic
from ..impact.analyzer import ImpactAnalyzer, RegressionRisk
from ..snapshot.parser import SnapshotParser
from ..snapshot.renderer import SnapshotRenderer
import os
import json

class WorkflowEngine:
    def __init__(self, state_dir: str = "brainos/state"):
        self.state = WorkflowState.PLANNING
        self.state_dir = state_dir
        os.makedirs(self.state_dir, exist_ok=True)
        
        self.planner = Planner()
        self.freeze_guard = FreezeGuard()
        self.decision_guard = DecisionGuard()
        self.executor = HumanApprovalExecutor()
        self.critic = Critic()
        self.impact_analyzer = ImpactAnalyzer()
        self.snapshot_parser = SnapshotParser()
        self.snapshot_renderer = SnapshotRenderer()
        
        self.graph = None
        self.queue = None
        self.current_task = None
        
    def run(self, goal: str):
        print(f"Starting BrainOS loop for goal: {goal}")
        
        while self.state != WorkflowState.DONE and self.state != WorkflowState.FAILED:
            print(f"\n--- STATE: {self.state} ---")
            
            if self.state == WorkflowState.PLANNING:
                self.graph = self.planner.plan(goal)
                self.queue = TaskQueue(self.graph)
                self.graph.save(os.path.join(self.state_dir, "graph.json"))
                self.state = WorkflowState.GUARD
                
            elif self.state == WorkflowState.GUARD:
                # In MVP, we check a stub to simulate guard pass
                passed = True
                if not self.freeze_guard.check("Type System"): # Dummy check
                    passed = False
                
                if passed:
                    self.state = WorkflowState.READY
                else:
                    print("Guard violation detected.")
                    self.state = WorkflowState.FAILED
                    
            elif self.state == WorkflowState.READY:
                self.current_task = self.queue.pop_ready_task()
                if self.current_task:
                    self.state = WorkflowState.EXECUTING
                else:
                    if self.graph.is_complete():
                        self.state = WorkflowState.SNAPSHOT
                    else:
                        print("Graph is deadlocked or failed.")
                        self.state = WorkflowState.FAILED
                        
            elif self.state == WorkflowState.EXECUTING:
                result = self.executor.execute(self.current_task)
                self.current_task.metadata["execute_result"] = result
                self.state = WorkflowState.CRITIQUE
                
            elif self.state == WorkflowState.CRITIQUE:
                eval_res = self.critic.evaluate(self.current_task, self.current_task.metadata["execute_result"])
                
                if eval_res == "PASS":
                    from ..task.task import TaskStatus
                    self.graph.update_task_status(self.current_task.id, TaskStatus.COMPLETED)
                    self.state = WorkflowState.IMPACT
                elif eval_res == "SKIP":
                    from ..task.task import TaskStatus
                    self.graph.update_task_status(self.current_task.id, TaskStatus.SKIPPED)
                    self.state = WorkflowState.READY
                else:
                    from ..task.task import TaskStatus
                    self.graph.update_task_status(self.current_task.id, TaskStatus.FAILED)
                    self.state = WorkflowState.FAILED
                    
                self.graph.save(os.path.join(self.state_dir, "graph.json"))
                
            elif self.state == WorkflowState.IMPACT:
                risk = self.impact_analyzer.analyze(self.current_task)
                self.current_task.metadata["regression_risk"] = risk.value
                self.graph.save(os.path.join(self.state_dir, "graph.json"))
                print(f"Impact Analyzed: Risk = {risk.value}")
                
                # After impact, go back to ready for next task
                self.state = WorkflowState.READY
                
            elif self.state == WorkflowState.SNAPSHOT:
                snap = self.snapshot_parser.parse("PROJECT_SNAPSHOT.md")
                snap.completed.append("BrainOS Task Execution (MVP)")
                self.snapshot_renderer.render(snap, "PROJECT_SNAPSHOT.md")
                print("PROJECT_SNAPSHOT.md updated.")
                self.state = WorkflowState.DONE
                
        print(f"\nWorkflow Finished with state: {self.state}")
