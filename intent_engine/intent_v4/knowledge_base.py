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
FILE: knowledge_base.py
PURPOSE: Part of the AAYU Intent-to-Silicon project
=============================================================================
This file is part of the AAYU (Aayu) Intent-to-Silicon Programming Language.
The AAYU language enables developers to write code using natural language
intentions, which are compiled to optimized backend code.

For beginners: This file handles part of the aayu intent-to-silicon project.
To understand the project architecture, see the ARCHITECTURE_FREEZE.md file.
=============================================================================
"""

KNOWLEDGE_BASE = {
    "Hospital": {
        "keywords": ["hospital", "clinic", "health", "patient", "doctor", "medical"],
        "roles": ["Admin", "Doctor", "Receptionist", "Patient"],
        "entities": ["Patient", "Doctor", "Appointment", "Prescription", "Invoice", "Payment"],
        "relations": [
            {"from": "Patient", "to": "Appointment", "type": "one_to_many"},
            {"from": "Doctor", "to": "Appointment", "type": "one_to_many"},
            {"from": "Appointment", "to": "Prescription", "type": "one_to_one"},
            {"from": "Appointment", "to": "Invoice", "type": "one_to_one"},
            {"from": "Invoice", "to": "Payment", "type": "one_to_one"}
        ],
        "workflow": {
            "name": "AppointmentWorkflow",
            "entity": "Appointment",
            "steps": ["BookAppointment", "DoctorReview", "Prescription", "Billing", "Payment"]
        }
    },
    "CRM": {
        "keywords": ["crm", "customer", "lead", "sales", "relationship"],
        "roles": ["Admin", "SalesManager", "SalesAgent", "Customer"],
        "entities": ["Customer", "Lead", "Opportunity", "Contact", "Note", "Activity"],
        "relations": [
            {"from": "Customer", "to": "Lead", "type": "one_to_many"},
            {"from": "Customer", "to": "Opportunity", "type": "one_to_many"},
            {"from": "Customer", "to": "Contact", "type": "one_to_many"},
            {"from": "Lead", "to": "Note", "type": "one_to_many"},
            {"from": "Opportunity", "to": "Activity", "type": "one_to_many"}
        ],
        "workflow": {
            "name": "LeadPipeline",
            "entity": "Lead",
            "steps": ["New", "Contacted", "Qualified", "Proposal", "Negotiation", "ClosedWon", "ClosedLost"]
        }
    },
    "LMS": {
        "keywords": ["lms", "learning", "course", "student", "teacher", "school", "education"],
        "roles": ["Admin", "Instructor", "Student"],
        "entities": ["Student", "Instructor", "Course", "Enrollment", "Lesson", "Quiz", "Submission"],
        "relations": [
            {"from": "Instructor", "to": "Course", "type": "one_to_many"},
            {"from": "Course", "to": "Lesson", "type": "one_to_many"},
            {"from": "Course", "to": "Quiz", "type": "one_to_many"},
            {"from": "Student", "to": "Course", "type": "many_to_many", "junction": "Enrollment"},
            {"from": "Student", "to": "Quiz", "type": "many_to_many", "junction": "Submission"}
        ],
        "workflow": {
            "name": "CourseWorkflow",
            "entity": "Course",
            "steps": ["Draft", "Review", "Published", "Archived"]
        }
    },
    "Police": {
        "keywords": ["police", "complaint", "fir", "crime", "investigation", "cop"],
        "roles": ["Admin", "Officer", "Citizen"],
        "entities": ["Complaint", "Officer", "Citizen", "Evidence", "Investigation", "Report"],
        "relations": [
            {"from": "Citizen", "to": "Complaint", "type": "one_to_many"},
            {"from": "Officer", "to": "Investigation", "type": "one_to_many"},
            {"from": "Complaint", "to": "Investigation", "type": "one_to_one"},
            {"from": "Complaint", "to": "Evidence", "type": "one_to_many"},
            {"from": "Investigation", "to": "Report", "type": "one_to_one"}
        ],
        "workflow": {
            "name": "ComplaintWorkflow",
            "entity": "Complaint",
            "steps": ["Filed", "Verified", "Investigated", "Action", "Closed"]
        }
    },
    "E-Commerce": {
        "keywords": ["ecommerce", "e-commerce", "shop", "store", "cart", "product", "order", "buy"],
        "roles": ["Admin", "Vendor", "Customer", "Support"],
        "entities": ["Customer", "Product", "Category", "Order", "OrderItem", "Payment", "Review"],
        "relations": [
            {"from": "Category", "to": "Product", "type": "one_to_many"},
            {"from": "Customer", "to": "Order", "type": "one_to_many"},
            {"from": "Order", "to": "Product", "type": "many_to_many", "junction": "OrderItem"},
            {"from": "Order", "to": "Payment", "type": "one_to_one"},
            {"from": "Customer", "to": "Product", "type": "many_to_many", "junction": "Review"}
        ],
        "workflow": {
            "name": "OrderWorkflow",
            "entity": "Order",
            "steps": ["Cart", "Placed", "Paid", "Processing", "Shipped", "Delivered", "Refunded"]
        }
    },
    "Inventory": {
        "keywords": ["inventory", "warehouse", "stock", "supply", "logistics"],
        "roles": ["Admin", "WarehouseManager", "Staff"],
        "entities": ["Product", "Supplier", "Warehouse", "StockItem", "PurchaseOrder", "Movement"],
        "relations": [
            {"from": "Supplier", "to": "Product", "type": "one_to_many"},
            {"from": "Product", "to": "StockItem", "type": "one_to_many"},
            {"from": "Warehouse", "to": "StockItem", "type": "one_to_many"},
            {"from": "Supplier", "to": "PurchaseOrder", "type": "one_to_many"},
            {"from": "StockItem", "to": "Movement", "type": "one_to_many"}
        ],
        "workflow": {
            "name": "PurchaseOrderWorkflow",
            "entity": "PurchaseOrder",
            "steps": ["Draft", "Requested", "Approved", "Received", "Stocked"]
        }
    },
    "ERP": {
        "keywords": ["erp", "enterprise", "resource", "planning", "company", "business"],
        "roles": ["Admin", "Manager", "Employee", "Accountant"],
        "entities": ["Department", "Employee", "Project", "Asset", "Expense", "Invoice"],
        "relations": [
            {"from": "Department", "to": "Employee", "type": "one_to_many"},
            {"from": "Employee", "to": "Project", "type": "many_to_many"},
            {"from": "Department", "to": "Asset", "type": "one_to_many"},
            {"from": "Employee", "to": "Expense", "type": "one_to_many"},
            {"from": "Project", "to": "Invoice", "type": "one_to_many"}
        ],
        "workflow": {
            "name": "ExpenseWorkflow",
            "entity": "Expense",
            "steps": ["Draft", "Submitted", "ManagerApproved", "FinanceApproved", "Paid"]
        }
    },
    "HRMS": {
        "keywords": ["hrms", "hr", "human", "resources", "leave", "payroll", "attendance"],
        "roles": ["Admin", "HRManager", "Manager", "Employee"],
        "entities": ["Employee", "Department", "Attendance", "LeaveRequest", "Payroll", "Review"],
        "relations": [
            {"from": "Department", "to": "Employee", "type": "one_to_many"},
            {"from": "Employee", "to": "Attendance", "type": "one_to_many"},
            {"from": "Employee", "to": "LeaveRequest", "type": "one_to_many"},
            {"from": "Employee", "to": "Payroll", "type": "one_to_many"},
            {"from": "Employee", "to": "Review", "type": "one_to_many"}
        ],
        "workflow": {
            "name": "LeaveWorkflow",
            "entity": "LeaveRequest",
            "steps": ["Draft", "Requested", "ManagerReview", "HRReview", "Approved", "Rejected"]
        }
    }
}

class KnowledgeBase:
    @classmethod
    def get_domains(cls):
        return KNOWLEDGE_BASE

    @classmethod
    def find_domain(cls, intent_text: str):
        intent_lower = intent_text.lower()
        
        for domain, data in KNOWLEDGE_BASE.items():
            for kw in data["keywords"]:
                if kw in intent_lower:
                    return domain
        return None
