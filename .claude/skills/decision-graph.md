# Decision Graph Skill

Manages company knowledge graph - decisions, strategies, policies, clients, opportunities, processes, and hiring records.

## Triggers

- User makes a business decision worth documenting
- User asks "why did we...?" or "what's our policy on...?"
- User discusses clients, opportunities, or processes
- User mentions strategic planning or direction
- `/decision`, `/strategy`, `/client`, `/opportunity` commands

## Actions

### Search First
Before creating, always search for existing records:
```bash
dg search "relevant keywords"
dg search -c "search content too"
```

### Create Records
```bash
dg new decision "Title of decision"
dg new strategy "Strategic initiative name"
dg new client "Client name"
dg new policy "Policy name"
dg new opportunity "Opportunity description"
dg new process "Process name"
dg new hiring "Role title"
```

### Link Related Records
```bash
dg link FROM_ID link_type TO_ID
```

Link types: `supersedes`, `depends_on`, `enables`, `relates_to`, `conflicts_with`, `refines`, `implements`

### Update Status
```bash
dg status ID new_status
```

Statuses: `draft`, `proposed`, `accepted`, `deprecated`, `superseded`

### View and Export
```bash
dg list                    # List all
dg show ID                 # Show details
dg show ID -l              # Show with links
dg graph                   # Visualize relationships
dg stats                   # Statistics
```

## Record Types

| Type | Prefix | When to Use |
|------|--------|-------------|
| Decision | DEC | Business decisions (SPADE framework) |
| Strategy | STR | Strategic direction (Six-Pager format) |
| Policy | POL | Internal policies, compliance |
| Client | CLI | Client info, customizations |
| Opportunity | OPP | Market opportunities (OST format) |
| Process | PRC | Internal processes (DACI framework) |
| Hiring | HIR | Role definitions, scorecards |

## Best Practices

1. **Search before creating** - Avoid duplicates
2. **Link records** - Build the knowledge graph
3. **Use tags** - Enable discovery
4. **Keep titles concise** - Make them searchable
5. **Update status** - Keep records current

## Example

User: "Let's use PostgreSQL for our new service instead of MongoDB"

```bash
# Search for related decisions
dg search "database"

# Create decision
dg new decision "Use PostgreSQL for new service"

# Edit the file to add context, alternatives, rationale

# Link to related records if found
dg link DEC-042 relates_to DEC-015

# Mark as accepted
dg status DEC-042 accepted
```
