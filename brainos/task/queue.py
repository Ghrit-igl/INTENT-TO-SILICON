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
FILE: queue.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

from typing import List, Optional
from .task import Task, TaskStatus
from .graph import TaskGraph

class TaskQueue:
    def __init__(self, graph: TaskGraph):
        self.graph = graph
        
    def pop_ready_task(self) -> Optional[Task]:
        ready = self.graph.get_ready_tasks()
        if ready:
            # We just take the first ready task
            task = ready[0]
            self.graph.update_task_status(task.id, TaskStatus.IN_PROGRESS)
            return task
        return None
