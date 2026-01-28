# Feedback on Decision Graph HTML & Objective Alignment

Based on `OBJECTIVE.md` and an analysis of the generated `_site` HTML, here is a report on what can be improved.

## 1. Alignment with Objectives

### "Answering Why"
*   **Objective**: "If Claude can't find the reason it should ask from the user... work towards the first principles."
*   **Current State**: The graph visualizes *dependencies* (`depends_on`), but the *rationale* is buried in the text of the markdown files.
*   **Improvement**: The visual graph could distinctly highlight "Rationale" or "Refines" links to show the chain of reasoning back to first principles (Foundational nodes). Foundational nodes are currently highlighted with a gold border, which is a good start.

### "Both Humans and Claude"
*   **Objective**: Tools for both.
*   **Current State**: The generated site is human-centric.
*   **Improvement**: The site should expose a clear `graph.json` or `knowledge.json` link in the footer or header, making it trivial for an AI agent (browsing the site) to ingest the whole graph structure in one go without scraping HTML.

### "Generate websites... to find items and their relations easily"
*   **Objective**: Easy visualization and relation finding.
*   **Current State**: The `graph.html` uses a basic D3 force-directed layout.
*   **Improvement**:
    *   **Filtering**: For larger graphs (like a "whole company"), the single hairball graph will be unusable. Add filters to the Graph view (e.g., "Show only Strategy", "Show only neighbors of X").
    *   **Hierarchical View**: A force-directed graph doesn't show *flow* well. A Sugiyama (layered) layout or a timeline-based layout would better show the *evolution* of decisions over time.

## 2. HTML & UX Improvements

### Critical Bugs
*   **Missing Title**: The `<title>` tag and the main `<h1>` header are empty (`<title>Records - </title>`). This confirms the configuration (`dg.toml`) was not loaded correctly during the build (which relates to the CLI issue I was fixing).

### Visual & Usability
*   **Graph View**:
    *   **Hardcoded Height**: The graph container has `height = 500`. It should ideally take up the full available screen height for better immersion.
    *   **Edge Labels**: Edge labels (relationship types) are not visible on the D3 graph, or are just lines. Differentiating distinct relationships (e.g., `conflicts_with` vs `enables`) via distinct line styles (dashed, colored) is implemented in logic but needs to be visually distinct enough.
    *   **Interactivity**: Clicking a node goes to the page. It would be nice to have a side-panel summary first.
*   **Index Page**:
    *   **Empty State**: The filters are functional, but having a "Quick Start" or "Foundational Principles" section at the top would help new users orient themselves before diving into the list of records.

## 3. Recommended Next Steps

1.  **Fix Config Loading**: Priority #1 is fixing the bug where the site title is missing. (This validates my current code task).
2.  **Enhance Graph Visualization**:
    *   Add a toggle for "Layered View" (DAG) to see dependency chains clearly.
    *   Make the graph fullscreen.
3.  **AI Integration**: Ensure a `data.json` is generated and linked in the footer.
