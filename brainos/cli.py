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

import sys
from decision_engine import DecisionEngine
from recommendation_engine import RecommendationEngine
from tradeoff_engine import TradeoffEngine
from architecture_generator import ArchitectureGenerator
from project_scaffold_generator import ProjectScaffoldGenerator
from cost_engine import CostEngine
from architecture_review import ArchitectureReview

def run_brainos(intent: str):
    print(f"🧠 [BrainOS] Analyzing Intent: '{intent}'")
    
    # 1. Decision
    de = DecisionEngine()
    constraints = de.analyze(intent)
    print(f"   ↳ Constraints Detected: {', '.join(constraints)}")
    
    # 2. Recommendation
    re = RecommendationEngine()
    architecture = re.recommend(constraints)
    print(f"   ↳ Architecture Selected: {architecture}")
    
    # 3. Tradeoffs & Cost
    te = TradeoffEngine()
    scores = te.evaluate(architecture)
    ce = CostEngine()
    cost_data = ce.estimate(architecture, constraints)
    print(f"   ↳ Tradeoff Scores: {scores}")
    print(f"   ↳ Est. Monthly Cost: ${cost_data['estimated_monthly_usd']}")
    
    # 4. Logical Architecture
    ag = ArchitectureGenerator()
    entities = ag.generate_entities(intent, architecture)
    print(f"   ↳ Entities Generated: {', '.join(entities.keys())}")
    
    # 5. Architecture Review
    ar = ArchitectureReview()
    review = ar.generate_report(constraints, architecture, entities)
    print(f"   ↳ Architecture Score: {review['overall_score']}/100")
    if review["security"]["findings"]:
        print(f"      - Security Flags: {', '.join(review['security']['findings'])}")
    if review["performance"]["findings"]:
        print(f"      - Performance Flags: {', '.join(review['performance']['findings'])}")
    
    # 6. Scaffolding
    psg = ProjectScaffoldGenerator()
    out_dir = psg.generate(intent, entities, output_dir="../out")
    print(f"✅ [BrainOS] Scaffolded AAYU project to: {out_dir}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print('Usage: python cli.py "Build a scalable hospital management system"')
        sys.exit(1)
        
    intent = sys.argv[1]
    run_brainos(intent)
