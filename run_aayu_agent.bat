@echo off
title AAYU AI Agent - Intent to Silicon
echo ================================================================
echo AAYU AI AGENT - NATIVE LAPTOP AUTOMATION & VOICE SYSTEM
echo 100%% Pure AAYU Language (.aayu) ^| ZERO Python
echo ================================================================
echo Starting AAYU Silicon VM Server on port 8080...
echo Opening Dashboard at http://localhost:8080
start http://localhost:8080
"%~dp0bin\aayu.exe" run "%~dp0examples\aayu_ai_agent.aayu" --web --port 8080
pause
