# Project Instructions

**IMPORTANT: This project uses Decision Graph (`dg`) to track all decisions.**

## Mandatory: Use dg for Decisions

When ANY decision is made during this conversation, you MUST create a record:

```bash
# Technical decisions → ADR
dg new adr "Decision title"

# Business decisions → Decision
dg new decision "Decision title"

# After incidents → Incident
dg new incident "Incident title"
```

## Before Making Changes

Always search for existing decisions first:
```bash
dg search "topic"
dg list --type adr
```

If you find conflicts with existing decisions, **ASK the user** before proceeding.

## Quick Reference

| Situation | Command |
|-----------|---------|
| Tech decision | `dg new adr "Title"` |
| Business decision | `dg new decision "Title"` |
| Process defined | `dg new process "Title"` |
| How-to written | `dg new runbook "Title"` |
| Meeting notes | `dg new meeting "Title"` |
| Link records | `dg link ADR-001 implements DEC-001` |
| Update status | `dg status ADR-001 accepted` |

## Record Types

| Prefix | Type | Use For |
|--------|------|---------|
| DEC | Decision | Business decisions |
| ADR | ADR | Technical/architecture decisions |
| INC | Incident | Post-mortems |
| RUN | Runbook | Operational how-tos |
| MTG | Meeting | Meeting notes |
| STR | Strategy | Strategic direction |
| POL | Policy | Internal policies |
| PRC | Process | Workflows |

## Example

User: "Let's use PostgreSQL for the database"

You should run:
```bash
dg new adr "Use PostgreSQL for database"
```

Then edit the created file to fill in context, alternatives, and consequences.
