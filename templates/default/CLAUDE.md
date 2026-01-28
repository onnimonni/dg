# Project Instructions

This project uses Decision Graph (`dg`) to track decisions, architecture, and operational knowledge.

## Your Responsibilities

1. **Check context before acting**: `dg search "<topic>"`
2. **Capture decisions**: Use `/decision`, `/adr`, `/incident`, `/runbook`, `/meeting`
3. **Ask when conflicts found**: If new work contradicts existing records, ask first
4. **Link records**: `dg link ADR-XXX implements DEC-YYY`

## Quick Reference

```bash
dg new decision "Title"     # Business decision
dg new adr "Title"          # Technical decision
dg new incident "Title"     # Post-mortem
dg new runbook "Title"      # How-to guide
dg new meeting "Title"      # Meeting notes

dg list                     # List all
dg search "query"           # Search
dg show DEC-001             # View record
dg graph                    # View relationships
```

## Record Types

| Type | Prefix | Use For |
|------|--------|---------|
| Decision | DEC | Business decisions |
| ADR | ADR | Technical decisions |
| Incident | INC | Post-mortems |
| Runbook | RUN | Operational how-tos |
| Meeting | MTG | Meeting notes |
| Strategy | STR | Strategic direction |
| Policy | POL | Internal policies |
| Process | PRC | Workflows |
