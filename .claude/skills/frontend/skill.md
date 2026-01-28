# Frontend Design

Use this skill when working on HTML templates, CSS, or UI components in `src/serve/templates.rs`.

## Design Philosophy

1. **Data Density, Not Clutter:** Display complex technical data (IDs, timestamps, dependencies) without overwhelming. Use whitespace and grouping effectively.

2. **"Unfurling" References:** Never display raw IDs (e.g., "DEC-005") as plain text. Unfurl them into rich, interactive components (mini-cards) showing status, title, and context.

3. **Visual Hierarchy:**
   - **Primary:** Titles, Status Badges (Critical info)
   - **Secondary:** Metadata, Authors, Dates (Context)
   - **Tertiary:** Borders, backgrounds, decorative elements

4. **Micro-Interactions:** Always include `:hover` states, focus rings, and subtle transitions (e.g., `transition 0.15s`, `transform: translateY(-2px)`). UI must feel "alive."

## Technical Stack

- **Templates:** `src/serve/templates.rs` - Rust string constants with MiniJinja
- **CSS:** Inline in BASE_TEMPLATE, no external framework
- **Icons:** Inline SVG or Unicode symbols (★, →, ›)
- **Fonts:** System fonts + monospace for IDs/code
- **Mode:** Dark mode default

## Design Tokens

```css
--bg: #1a1a2e        /* Page background */
--surface: #16213e   /* Card/panel background */
--primary: site-configured
--accent: site-configured
--text: #e2e8f0      /* Primary text (slate-200) */
--text-dim: #94a3b8  /* Secondary text (slate-400) */
--success: #4CAF50   /* Accepted/positive */
--warning: #FF9800   /* Proposed/warning */
```

## Type Colors

```css
DEC: #4CAF50  /* Decision - green */
ADR: #607D8B  /* Architecture - blue-gray */
INC: #F44336  /* Incident - red */
POL: #FF9800  /* Policy - orange */
RUN: #8BC34A  /* Runbook - light green */
STR: #2196F3  /* Strategy - blue */
PRC: #00BCD4  /* Process - cyan */
```

## Gemini UX Review

For complex design decisions, use `mcp__consult-llm__consult_llm` to get feedback from Gemini (configured as Principal UX Engineer).

### When to Consult

- Before finalizing visual design changes
- When unsure about color contrast or accessibility
- For complex layout decisions
- To validate UX patterns

## Workflow

1. **Identify the component** in `src/serve/templates.rs`
2. **Check existing styles** - reuse CSS classes where possible
3. **Consult Gemini** for UX feedback on non-trivial changes
4. **Make minimal changes** - don't over-engineer
5. **Test visually** - run `dg serve` and check in browser
6. **Take screenshots** - use Playwright MCP for visual verification

## Common Patterns

### Clickable Cards
```css
.card-link {
    transition: transform 0.15s, box-shadow 0.15s;
}
.card-link:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.3);
}
```

### Status Badges
```css
.badge.accepted { background: var(--success); }
.badge.proposed { background: var(--warning); color: #000; }
.badge.open { background: #e74c3c; }
.badge.resolved { background: #3498db; }
```

### Muted Pills/Tags
```css
.tag {
    background: rgba(148,163,184,0.1);
    color: var(--text-dim);
    font-family: monospace;
    font-size: 0.75rem;
    padding: 0.15rem 0.5rem;
    border-radius: 3px;
}
```
