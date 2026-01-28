#!/usr/bin/env python3
"""
Session start hook - remind Claude about decision graph context.
Runs on SessionStart to inject decision graph awareness.
"""

import json
import os
import subprocess
import sys

def get_dg_stats():
    """Get decision graph statistics"""
    try:
        result = subprocess.run(
            ["dg", "stats", "--quiet"],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except Exception:
        pass
    return None

def get_recent_records():
    """Get recent records from the graph"""
    try:
        result = subprocess.run(
            ["dg", "list", "--limit", "5"],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except Exception:
        pass
    return None

def main():
    # Read hook input
    hook_input = json.loads(sys.stdin.read())

    project_dir = os.environ.get("CLAUDE_PROJECT_DIR", ".")

    # Check if decision graph is initialized
    decisions_dir = os.path.join(project_dir, "docs", ".decisions")
    if not os.path.isdir(decisions_dir):
        # No decision graph - nothing to remind about
        print(json.dumps({"continue": True}))
        return

    # Get context
    stats = get_dg_stats()
    recent = get_recent_records()

    # Build reminder message
    reminder_parts = [
        "📊 Decision Graph Active",
        "",
        "This project uses `dg` to track decisions, strategies, ADRs, incidents, and more.",
        "When you make decisions or discover important context, consider capturing it:",
        "- `/decision` - business decisions",
        "- `/adr` - technical/architecture decisions",
        "- `/incident` - post-mortems",
        "- `/runbook` - operational how-tos",
        "",
        "Before making changes, check if related records exist: `dg search <topic>`",
        "If something conflicts with existing decisions, ask the user for clarification.",
    ]

    if stats:
        reminder_parts.extend(["", "Current stats:", stats])

    if recent:
        reminder_parts.extend(["", "Recent records:", recent])

    # Output reminder
    output = {
        "continue": True,
        "message": "\n".join(reminder_parts)
    }

    print(json.dumps(output))

if __name__ == "__main__":
    main()
