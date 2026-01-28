# Frontend Design

Help with frontend UI/UX design and implementation.

## When to Use

- Designing new UI components
- Improving visual design (colors, spacing, typography)
- Fixing layout/alignment issues
- Adding CSS animations or transitions
- Responsive design adjustments
- Accessibility improvements
- UX review and feedback

## Gemini UX Review

For UX/frontend design questions, **use the consult-llm MCP server** to get a second opinion from Gemini Pro. It's configured as a Principal UX Engineer who prefers Tailwind CSS.

### How to Use

The `consult-llm` MCP server provides the `consult_llm` tool:

```
# Get UX feedback on a design decision
mcp__consult-llm__consult_llm: "Review this CSS for a record link button: padding: 0.1rem 0.35rem, border-radius: 3px, white text on colored background. Is this good for readability?"

# Review layout approach
mcp__consult-llm__consult_llm: "I'm building a timeline visualization with year markers and nodes. Should I use CSS grid or flexbox? What's the best approach?"
```

### When to Consult Gemini

- Before finalizing visual design changes
- When unsure about color contrast or accessibility
- For complex layout decisions
- To validate UX patterns
- When user reports something "looks weird"
- For Tailwind CSS class recommendations

## Design Tokens (from templates.rs)

```css
--bg: #1a1a2e        /* Page background */
--surface: #16213e   /* Card/panel background */
--primary: site-configured
--accent: site-configured
--text: #eee         /* Primary text */
--text-dim: #999     /* Secondary text */
--success: #4CAF50   /* Accepted/positive */
--warning: #FF9800   /* Proposed/warning */
```

## Workflow

1. **Identify the component** in `src/serve/templates.rs`
2. **Check existing styles** - reuse CSS classes where possible
3. **Consult Gemini** for UX feedback on non-trivial changes
4. **Make minimal changes** - don't over-engineer
5. **Test visually** - run `dg serve` and check in browser
6. **Check responsive** - resize browser to verify layout

## Common Tasks

### Fix alignment
- Use `vertical-align: baseline` for inline elements
- Use flexbox for layout: `display: flex; align-items: center;`

### Fix colors
- Text on dark bg: use `#fff` or `var(--text)`
- Text on colored bg: use `#fff` for contrast
- **Ask Gemini** to verify contrast ratios

### Fix spacing
- Use rem units: `0.25rem`, `0.5rem`, `1rem`
- Padding for buttons: `0.25rem 0.5rem`
