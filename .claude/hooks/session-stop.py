#!/usr/bin/env python3
"""
Session stop hook - prompt to capture decisions made during session.
Runs on Stop to remind about documenting important decisions.
"""

import json
import os
import sys

def main():
    # Read hook input
    hook_input = json.loads(sys.stdin.read())

    project_dir = os.environ.get("CLAUDE_PROJECT_DIR", ".")

    # Check if decision graph is initialized
    decisions_dir = os.path.join(project_dir, "docs", "decisions")
    if not os.path.isdir(decisions_dir):
        print(json.dumps({"continue": True}))
        return

    # Remind to capture decisions
    output = {
        "continue": True,
        "message": (
            "💡 Session ending - consider capturing any decisions made:\n"
            "- Important technical choices → `/adr`\n"
            "- Business decisions → `/decision`\n"
            "- New processes defined → `dg new process`\n"
            "- Incidents discussed → `/incident`"
        )
    }

    print(json.dumps(output))

if __name__ == "__main__":
    main()
