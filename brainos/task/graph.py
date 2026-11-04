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
FILE: graph.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List, Dict, Set
from .task import Task, TaskStatus
import json
import os

class TaskGraph:
    def __init__(self):
        self.tasks: Dict[str, Task] = {}
        
    def add_task(self, task: Task):
        self.tasks[task.id] = task
        
    def get_task(self, task_id: str) -> Task:
        return self.tasks.get(task_id)
        
    def update_task_status(self, task_id: str, status: TaskStatus):
        if task_id in self.tasks:
            self.tasks[task_id].status = status
            self._update_ready_tasks()
            
    def _update_ready_tasks(self):
        for task in self.tasks.values():
            if task.status == TaskStatus.PENDING:
                if self._can_run(task):
                    task.status = TaskStatus.READY
                    
    def _can_run(self, task: Task) -> bool:
        for dep_id in task.dependencies:
            dep_task = self.tasks.get(dep_id)
            if not dep_task or dep_task.status not in (TaskStatus.COMPLETED, TaskStatus.SKIPPED):
                return False
        return True
        
    def get_ready_tasks(self) -> List[Task]:
        return [t for t in self.tasks.values() if t.status == TaskStatus.READY]
        
    def is_complete(self) -> bool:
        if not self.tasks:
            return True
        return all(t.status in (TaskStatus.COMPLETED, TaskStatus.SKIPPED) for t in self.tasks.values())
        
    def has_failed(self) -> bool:
        return any(t.status == TaskStatus.FAILED for t in self.tasks.values())
        
    def save(self, filepath: str):
        data = [t.to_dict() for t in self.tasks.values()]
        os.makedirs(os.path.dirname(filepath), exist_ok=True)
        with open(filepath, 'w') as f:
            json.dump(data, f, indent=2)
            
    @classmethod
    def load(cls, filepath: str) -> 'TaskGraph':
        graph = cls()
        if os.path.exists(filepath):
            with open(filepath, 'r') as f:
                data = json.load(f)
                for t_data in data:
                    graph.add_task(Task.from_dict(t_data))
            graph._update_ready_tasks()
        return graph
