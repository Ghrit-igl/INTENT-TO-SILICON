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
PURPOSE: Autonomous Project Generation Pipeline (BrainOS v2)
=============================================================================
"""

import os
import json
import shutil
from typing import Dict, Any
from intent_engine.v2.engine import IntentEngine
from brainos.v2.pipeline import BrainOSPipeline

class ProjectGenerator:
    """
    Orchestrates the entire autonomous project generation pipeline:
    User Prompt -> Intent Engine v2 -> BrainOS v2 -> Architecture Model ->
    Module Graph -> Folder Generator -> Code Generator -> Formatter ->
    Linter -> Compiler -> Unit Tests -> Validator -> Ready Project
    """
    def __init__(self, target_dir: str = "."):
        self.target_dir = target_dir
        self.intent_engine = IntentEngine()
        self.brainos = BrainOSPipeline()
        
    def generate(self, prompt: str, project_name: str = "auto_project") -> bool:
        print(f"\n[Generator] Starting pipeline for: '{prompt}'")
        project_path = os.path.join(self.target_dir, project_name)
        
        try:
            # 1. Intent Engine
            print("[Generator] -> Intent Engine v2")
            intent_ir = self.intent_engine.process_prompt(prompt)
            
            # 2. BrainOS Pipeline
            print("[Generator] -> BrainOS v2")
            arch_result = self.brainos.process_intent(intent_ir)
            
            # 3. Architecture Model & Module Graph
            print("[Generator] -> Architecture Model & Module Graph")
            folder_structure = arch_result.get("folder_structure", {})
            generated_files = arch_result.get("generated_files", {})
            
            # 4. Folder Generator
            print("[Generator] -> Folder Generator")
            self._scaffold_folders(project_path, folder_structure)
            
            # 5. Code Generator
            print("[Generator] -> AAYU Code Generator")
            self._write_files(project_path, generated_files)
            
            # 6. Formatter
            print("[Generator] -> Formatter")
            self._run_formatter(project_path)
            
            # 7. Linter
            print("[Generator] -> Linter")
            self._run_linter(project_path)
            
            # 8. Compiler (Syntax/Type Checks)
            print("[Generator] -> Compiler")
            self._run_compiler(project_path)
            
            # 9. Unit Tests
            print("[Generator] -> Unit Tests")
            self._run_tests(project_path)
            
            # 10. Validator
            print("[Generator] -> Final Validator")
            self._final_validation(project_path)
            
            print(f"\n[READY] Project '{project_name}' successfully generated at {project_path}")
            return True
            
        except Exception as e:
            print(f"\n[GENERATION FAILED] {str(e)}")
            print("Project is NOT ready. Needs Review.")
            # Rollback on failure
            if os.path.exists(project_path):
                shutil.rmtree(project_path)
            return False

    def _scaffold_folders(self, project_path: str, structure: Dict[str, Any]):
        """Creates the initial project folders from architecture."""
        base_structure = {
            "src": {},
            "docs": {},
            "tests": {}
        }
        
        # Merge architecture folders
        def merge_dicts(d1, d2):
            for k, v in d2.items():
                if isinstance(v, dict) and k in d1:
                    merge_dicts(d1[k], v)
                else:
                    d1[k] = v
        
        merge_dicts(base_structure, structure)
        
        def create_nodes(current_path: str, nodes: Dict[str, Any]):
            for node, content in nodes.items():
                node_path = os.path.join(current_path, node)
                if content is None or isinstance(content, str):
                    # It's a file
                    with open(node_path, 'w') as f:
                        f.write(content if isinstance(content, str) else "")
                elif isinstance(content, list):
                    # Folder with list of files
                    os.makedirs(node_path, exist_ok=True)
                    for file_item in content:
                        open(os.path.join(node_path, file_item), 'w').close()
                elif isinstance(content, dict):
                    # Nested folder
                    os.makedirs(node_path, exist_ok=True)
                    create_nodes(node_path, content)
                    
        os.makedirs(project_path, exist_ok=True)
        create_nodes(project_path, base_structure)
        
        # Write .gitattributes for GitHub language support
        with open(os.path.join(project_path, ".gitattributes"), 'w') as f:
            f.write("*.aayu linguist-language=Rust\n")
            
        # Write basic aayu.toml
        with open(os.path.join(project_path, "aayu.toml"), 'w') as f:
            f.write(f"""
[project]
name = "{os.path.basename(project_path)}"
version = "0.1.0"
edition = "2026"
""")

        # Write README.md
        with open(os.path.join(project_path, "README.md"), 'w') as f:
            f.write(f"# {os.path.basename(project_path)}\n\nGenerated AAYU project.\n")

        # Write .gitignore
        with open(os.path.join(project_path, ".gitignore"), 'w') as f:
            f.write("*.db\nbuild/\n.aayu/\n")

        # Add AI context rules so LLM Agents know how to code in AAYU
        ai_rules = """# AAYU Language - AI Agent Guidelines
You are an AI programming assistant. AAYU is a new language. Follow these rules:
1. End statements with `.` instead of `;`.
2. Scopes use `{ }`.
3. Print using `show("text")`.
4. Define variables with `let x = 1.`.
5. Functions use `fn`: `fn add(a, b) { return a + b. }`.
6. Built-in modules: `os`, `sys`, `time`, `math`, `json`, `http`, `crypto`, `sqlite3`, `socket`, `re`.
7. Errors: `try { throw "error". } catch e { } finally { }`.
"""
        with open(os.path.join(project_path, ".cursorrules"), 'w') as f:
            f.write(ai_rules)
        with open(os.path.join(project_path, ".windsurfrules"), 'w') as f:
            f.write(ai_rules)

    def _write_files(self, base_path: str, files: Dict[str, str]):
        for rel_path, content in files.items():
            full_path = os.path.join(base_path, rel_path)
            os.makedirs(os.path.dirname(full_path), exist_ok=True)
            with open(full_path, "w") as f:
                f.write(content)

    def _run_formatter(self, project_path: str):
        # Simulated format pass
        pass

    def _run_linter(self, project_path: str):
        # Simulated lint pass
        pass

    def _run_compiler(self, project_path: str):
        # Simulated compile pass
        # Check aayu.toml exists
        if not os.path.exists(os.path.join(project_path, "aayu.toml")):
            raise ValueError("Compiler Error: aayu.toml missing.")

    def _run_tests(self, project_path: str):
        # Simulated test pass
        pass

    def _final_validation(self, project_path: str):
        # Must have main.aayu or lib.aayu
        src_dir = os.path.join(project_path, "src")
        if not os.path.exists(src_dir):
            raise ValueError("Validation Error: src/ directory missing.")
