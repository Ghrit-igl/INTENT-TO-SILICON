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

from .tokenizer import Tokenizer
from .pos_tagger import POSTagger
from .semantic_graph import SemanticGraph
from .knowledge_graph import KnowledgeGraph
from .constraint_resolver import ConstraintResolver
from .intent_ir import IntentIR
from .context_memory import ContextMemory

class OfflineNLPEngine:
    def __init__(self):
        self.tokenizer = Tokenizer()
        self.tagger = POSTagger()
        self.knowledge = KnowledgeGraph()
        self.resolver = ConstraintResolver()
        self.memory = ContextMemory()

    def process(self, prompt: str) -> IntentIR:
        # Pipeline: Tokenizer -> Parser (Tagger) -> Semantic Graph -> Knowledge Graph -> Context Memory -> Constraint Resolver -> Intent IR
        tokens = self.tokenizer.tokenize(prompt)
        tagged = self.tagger.tag(tokens)
        
        sgraph = SemanticGraph().build_from_tagged(tagged)
        knodes = self.knowledge.resolve(sgraph)
        
        raw_constraints = []
        entities = []
        for k in knodes:
            entities.append(k.entity)
            raw_constraints.extend(k.security_rules)
            raw_constraints.extend(k.performance_rules)
            
        # Merge active constraints from memory
        raw_constraints.extend(self.memory.get_contextual_constraints())
        
        resolved_nfrs = self.resolver.resolve(raw_constraints)
        
        ir = IntentIR()
        ir.goal = prompt
        ir.entities = entities
        ir.constraints = raw_constraints
        ir.non_functional = resolved_nfrs
        ir.confidence = 0.95
        
        self.memory.add_interaction(prompt, ir)
        
        return ir
