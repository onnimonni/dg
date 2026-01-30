# DG Tool Feedback

Based on creating the Pied Piper sample project (20 records, 30 links).

## What Worked Well

- `dg init` - fast, sensible defaults
- `dg new <type> "title"` - intuitive
- `dg why`, `dg impact`, `dg principles` - excellent graph traversal
- `dg build` + `.site.yaml` branding - works great
- `type:adr status:accepted` query syntax - powerful
- Stats/visualization - good health overview

## Pain Points

### 1. YAML Frontmatter is Error-Prone

YAML's indentation sensitivity causes frequent errors:

```yaml
# Easy to break:
links:
  depends_on: [DEC-001]
   enables: [ADR-002]  # Oops, wrong indent = parse error
```

**Suggestion**: Switch to TOML frontmatter.

```toml
+++
type = "decision"
id = "DEC-001"
title = "Anti-Hooli Principle"
status = "accepted"
core = true
authors = ["Richard"]
tags = ["culture", "ethics"]

[links]
depends_on = []
enables = ["DEC-002", "POL-001"]
+++
```

TOML advantages:
- No indentation sensitivity
- Explicit syntax (quotes required for strings)
- Native Rust support via `toml` crate
- No "Norway problem" (`no` ≠ `false`)
- Clearer array syntax

### 2. Manual File Editing After Every `dg new`

Creating a record is step 1. Then I had to:
- Open file
- Edit frontmatter (authors, tags, core, dates, status)
- Replace template placeholders
- Add links manually

**Impact**: 80% file editing, 20% CLI.

### 3. No CLI Options for Metadata

```bash
# Wanted:
dg new decision "Title" --core --status accepted --author "Richard"

# Reality:
dg new decision "Title"
vim docs/decisions/DEC-001-*.md  # Manual edit
```

### 4. Links Require File Editing

```bash
# dg link adds ONE link
dg link ADR-001 depends_on DEC-001

# Multiple link types = manual edit
```

### 5. No `dg edit` Command

```bash
dg edit ADR-001  # Want: opens in $EDITOR
```

### 6. Verbose Templates

SPADE template is overkill for simple records. Need minimal option.

### 7. No Batch Operations

```bash
# Wanted:
dg import records.toml
```

### 8. Date Handling

`created`/`updated` auto-set to today. Historical records need manual edit.

## Suggested Improvements

### High Priority

**1. Switch frontmatter from YAML to TOML**

```toml
+++
type = "adr"
id = "ADR-001"
title = "Middle-Out Compression"
status = "accepted"
created = 2014-04-15
updated = 2014-05-01
authors = ["Richard Hendricks"]
tags = ["compression", "algorithm"]
core = true

[links]
depends_on = ["DEC-001"]
enables = ["ADR-002", "ADR-003"]
relates_to = ["INC-001"]
+++

# Middle-Out Compression Algorithm

Content here...
```

**2. CLI metadata flags**

```bash
dg new decision "Title" \
  --status accepted \
  --core \
  --author "Name" \
  --tag security \
  --depends-on DEC-001
```

**3. `dg edit` command**

```bash
dg edit ADR-001              # Open in $EDITOR
dg edit ADR-001 --status     # Interactive picker
```

**4. `dg set` for quick changes**

```bash
dg set ADR-001 status accepted
dg set ADR-001 core true
dg set ADR-001 +tag security
```

### Medium Priority

**5. Site config as TOML**

```toml
# .site.toml (instead of .site.yaml)
title = "Pied Piper Decisions"
description = "Decision history"
logo = "assets/logo.svg"
primary_color = "#2E7D32"
accent_color = "#4CAF50"
footer = "Pied Piper Inc."

[custom_css]
inline = """
.core { border-color: gold; }
"""
```

**6. Simpler templates**

```bash
dg new decision "Title" --minimal  # Skip SPADE boilerplate
```

**7. Import/export**

```bash
dg export > backup.toml
dg import records.toml
```

**8. Clone records**

```bash
dg clone ADR-001 ADR-006
dg derive ADR-001 --supersedes
```

### Nice to Have

**9. `dg lint --fix`** - Auto-fix simple issues

**10. Project templates**

```bash
dg new decision "Title" --template quick
```

## Workflow Comparison

### Current

```bash
dg new decision "Anti-Hooli Principle"
# Open file
# Fix YAML indentation
# Change status
# Add core: true
# Add authors list
# Add tags list
# Add links (careful with indent!)
# Save
dg reindex
```

### Ideal (with TOML + CLI flags)

```bash
dg new decision "Anti-Hooli Principle" \
  --status accepted \
  --core \
  --author "Richard" \
  --tag culture,ethics \
  --enables DEC-002,POL-001 \
  --edit
```

## Summary

**Core strengths**: Graph model, traversal commands (`why`, `impact`, `principles`).

**Main pain**: Record creation requires too much manual editing.

**Priority fixes**:

| # | Change | Impact |
|---|--------|--------|
| 1 | YAML → TOML frontmatter | Eliminate indent errors |
| 2 | `--status`, `--author`, `--tag` flags | Skip post-creation editing |
| 3 | `dg edit <id>` | Quick access to records |
| 4 | `dg set <id> <field> <value>` | Single-field changes |

These changes would cut record creation time by ~70%.
