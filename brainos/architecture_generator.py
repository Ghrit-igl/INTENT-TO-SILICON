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

﻿class ArchitectureGenerator:
    """
    Generates the logical AAYU entities based on the architecture.
    """
    def generate_entities(self, intent: str, architecture: str) -> dict:
        intent_lower = intent.lower()
        entities = {}
        if "hospital" in intent_lower:
            entities["Patient"] = ["id: Number", "name: Text", "dob: Text", "medical_history: Text"]
            entities["Doctor"] = ["id: Number", "name: Text", "specialty: Text"]
            entities["Appointment"] = ["id: Number", "patient_id: Number", "doctor_id: Number", "date: Text", "status: Text"]
        elif "banking" in intent_lower:
            entities["Account"] = ["id: Number", "owner: Text", "balance: Number"]
            entities["Transaction"] = ["id: Number", "from_account: Number", "to_account: Number", "amount: Number", "timestamp: Text"]
        elif "blog" in intent_lower:
            entities["Post"] = ["id: Number", "title: Text", "content: Text", "author_id: Number"]
            entities["User"] = ["id: Number", "username: Text", "email: Text"]
        else:
            entities["App"] = ["id: Number", "name: Text"]
        return entities
