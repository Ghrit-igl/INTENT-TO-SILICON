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

class ScalingAdvisor:
    def advise(self, load_params: dict) -> dict:
        req_per_sec = load_params.get("requests_per_second", 0)
        data_size_gb = load_params.get("data_size_gb", 0)
        
        strategy = "monolith"
        database = "single"
        cache = "in-memory"
        
        if req_per_sec > 10000 or data_size_gb > 1000:
            strategy = "microservices"
            database = "sharded_cluster"
            cache = "redis_cluster"
        elif req_per_sec > 1000 or data_size_gb > 100:
            strategy = "load_balanced_monolith"
            database = "primary_replica"
            cache = "redis"
            
        return {
            "strategy": strategy,
            "database": database,
            "cache": cache
        }
