# Architecting the Causal Graph: A Monorepo-Native Framework for Decision Lineage in Enterprise and Personal Contexts

## 1. Introduction: The Epistemological Crisis in Management and Life

In the contemporary landscape of high-velocity business and increasingly complex personal life management, the fundamental challenge is no longer the acquisition of information, but the preservation of context. We operate in environments characterized by "wicked problems"—challenges that are ill-defined, possess no stopping rule, and where solutions are not true-or-false, but good-or-bad. Whether deciding to migrate a monolithic software architecture to microservices or determining the optimal geographic location for a family home, the decision is rarely an isolated event. It is a node in a sprawling, interconnected directed acyclic graph (DAG) of causality.

However, standard documentation practices fail to capture this topology. Decisions are typically recorded as static snapshots—meeting minutes, email threads, or isolated Markdown files—divorced from the arguments that shaped them and the outcomes they produced. This leads to the phenomenon of "Context Collapse," where future stakeholders (or one’s future self) cannot determine the rationale behind a legacy constraint. This opacity births "Chesterton’s Fence" dilemmas: the inability to remove a seemingly redundant fence (or process, or code module) because one does not know why it was erected in the first place.

The user’s query posits a sophisticated solution: a documentation tool designed to track these chains of decisions and causalities, housed within a **Git monorepo**. This specific architectural choice—the monorepo—is profound. By situating the decision record within the same version-control substrate as the execution artifacts (source code, financial data, legal documents), we enable "Atomic Causality." A single Git commit can encapsulate the intent (the decision), the implementation (the code), and the validation (the test).

This report provides an exhaustive blueprint for constructing such a tool. It synthesizes ontologies from systems engineering (Issue-Based Information Systems, Goal Structuring Notation) with modern software practices (Architecture Decision Records, Docs-as-Code). It details the technical implementation of a **Decision Causality Engine (DCE)** using the Git monorepo as the database of record, exploring schema design, parsing logic, automation pipelines, and visualization strategies necessary to render the invisible architecture of thought visible.

------

## 2. Ontological Foundations of Decision Modeling

Before a single line of code is written for the documentation tool, we must define the metaphysics of the system. What is a "decision"? How does it differ from a "task" or a "goal"? To track causality, we require a rigid ontology that can handle the ambiguity of human reasoning while remaining machine-parseable. We will integrate three distinct modeling frameworks to cover the full spectrum of "Life and Business" decision-making.

### 2.1 The Argumentation Layer: Issue-Based Information Systems (IBIS)

The foundation of our model addresses the "Why." In both strategic business planning and personal life dilemmas, the path to a decision is often a contentious negotiation between competing values. The **Issue-Based Information System (IBIS)**, developed by Werner Kunz and Horst Rittel in the 1960s, provides the necessary structure for this.

IBIS treats the decision process not as a linear algorithm but as a conversation. It consists of three primary elements:

1. **Issues (Questions):** These are the root nodes of uncertainty. In a software context, an issue might be "How do we handle state management?" In a personal context, "Where should we live?" Issues are open-ended and invite dialogue.
2. **Positions (Ideas):** These are the proposed responses to an Issue. A single Issue will spawn multiple Positions (e.g., "Redux," "Context API," "MobX"). Positions are mutually exclusive in the final selection but coexist during the deliberation phase.
3. **Arguments (Pros/Cons):** These are the logical modifiers attached to Positions. They do not exist independently; they must support or object to a Position.

#### 2.1.1 Integrating IBIS into Git

In our monorepo tool, the IBIS elements map to specific artifacts. While traditional IBIS tools like Compendium use graphical interfaces, a "Docs-as-Code" approach requires a textual representation. We can model Issues as directory structures or "Request for Comment" (RFC) documents.

- **File Structure Implication:** An Issue is a Markdown file (e.g., `ISSUE-001.md`). Positions are either sub-headings within that file or distinct files linked via frontmatter.
- **Causality Tracking:** The IBIS model captures the *rejected* alternatives. This is critical for preventing the "Zombie Decision" phenomenon, where a team re-litigates a previously discarded option because they forgot the fatal flaw (Con argument) identified six months prior.

The tool must support **Dialogue Mapping**, visualizing the tree of arguments. If `Argument A` (Pro) supports `Position X`, and `Argument B` (Con) attacks `Argument A`, the tool must render this adversarial relationship. This depth is what separates a "decision log" from a "causality engine."

### 2.2 The Assurance Layer: Goal Structuring Notation (GSN)

While IBIS captures the debate, it does not inherently capture the *validity* or the *verification* of the decision. For this, we turn to safety-critical systems engineering and **Goal Structuring Notation (GSN)**. GSN is typically used to build "Safety Cases" (e.g., proving a car's brakes will not fail), but we adapt it here for "Business Assurance Cases" and "Life Assurance Cases."

GSN introduces specific node types:

1. **Goal:** A claim about the system (e.g., "The new database reduces latency by 50%").
2. **Strategy:** The reasoning used to achieve the goal (e.g., "Use in-memory caching").
3. **Evidence:** The tangible artifact that proves the Goal has been met (e.g., `benchmarks/2024-latency-report.pdf`).

#### 2.2.1 GSN for Business and Life

In a business context, linking a decision (Strategy) to a Goal ensures alignment with OKRs (Objectives and Key Results). If a decision record claims to "Improve Developer Velocity," the GSN model requires a link to the *Evidence* node.

- **Traceability Mechanism:** The tool parses these links. If a Goal node exists without linked Evidence, the system flags it as an "Unverified Assumption."
- **Life Application:** In personal finance decisions, a Goal might be "Retire at 60." The Strategy might be "Maximize 401k." The Evidence is the quarterly account statement committed to the encrypted `data/` folder of the monorepo.

This layer adds accountability. A decision is not just a choice; it is a claim that must be substantiated.

### 2.3 The Record Layer: Architecture Decision Records (ADR)

The final layer is the immutable log of the outcome. **Architecture Decision Records (ADR)** provide the industry-standard format for software engineering decisions. They are lightweight, text-based, and version-controlled.

Key attributes of an ADR include:

- **Context:** The constraints and forces at play (e.g., budget, timeline, skill set).
- **Decision:** The specific choice made (often the "Winning Position" from the IBIS phase).
- **Status:** The lifecycle state (`Proposed`, `Accepted`, `Deprecated`, `Superseded`).
- **Consequences:** The resulting context, including new liabilities (technical debt) or capabilities.

#### 2.3.1 The "Superseded" Lifecycle

Crucial to "tracking chains" is the `Superseded` status. When `DEC-002` replaces `DEC-001`, a causal link is formed. The tool must enforce referential integrity here: one cannot mark a decision as superseded without specifying the successor. This creates a linked list of history, allowing a new employee (or family member) to traverse backwards from the current state to the origin of the system.

### 2.4 Synthesis: The Unified Causal Model (UCM)

To satisfy the user’s request for a single tool, we must synthesize these three models into a unified schema. We cannot ask users to write three separate documents. Instead, we define a "Super-Node" in our graph.

| **Conceptual Layer** | **Data Element**               | **Git Implementation**               |
| -------------------- | ------------------------------ | ------------------------------------ |
| **Why?** (IBIS)      | Issue, Position, Argument      | Markdown sections or RFC branches    |
| **What?** (ADR)      | Decision, Context, Consequence | Markdown Frontmatter & Body          |
| **Proven By?** (GSN) | Goal, Evidence                 | Links to `/data` or `/metrics` files |
| **Logic?** (DMN)     | Business Rules                 | DMN Tables or Code References        |

This UCM allows the tool to answer complex queries: "Show me all accepted decisions (ADR) that support the goal of 'High Availability' (GSN) but had significant opposition regarding 'Cost' (IBIS).

------

## 3. Schema Design and Data Serialization

To operationalize the Unified Causal Model, we need a rigorous data schema. In a git monorepo environment, the most robust storage format is **Markdown with YAML Frontmatter**. This format is human-readable, diff-friendly (essential for Git), and parseable by virtually every modern static site generator and documentation tool.

### 3.1 The Extended YAML Frontmatter Schema

We propose a comprehensive schema that captures the causality fields required by our UCM. The tool's parser will define a TypeScript interface to validate this frontmatter during the CI/CD process.

YAML

```
---
id: DEC-2025-042
title: "Transition to Edge Computing for User Profiles"
date: 2025-10-27
status: Accepted # Enum: Proposed, Accepted, Rejected, Deprecated, Superseded
authors: ["@arch-team", "@jdoe"]

# Causality: The Backward Chain (Provenance)
derived_from:
  - type: issue
    id: "ISSUE-109"
    desc: "Latency in APAC region exceeds SLA"
  - type: goal
    id: "GOAL-2025-Q4"
    desc: "Reduce global TTFB to <100ms"

# Causality: The Forward Chain (Lifecycle)
supersedes:
enables:
blocked_by:

# IBIS Argumentation Summary
positions_considered:
  - name: "AWS Lambda @ Edge"
    outcome: accepted
    rationale: "Lowest operational overhead"
  - name: "Multi-region Kubernetes"
    outcome: rejected
    rationale: "High maintenance cost (See ARG-004)"

# Traceability: The Monorepo Link
impacts:
  - path: "apps/user-profile-service"
    type: code_module
  - path: "infra/terraform/cdn"
    type: infrastructure
  - metric: "metrics/latency-p99"
    type: evidence

# Semantic Web / Knowledge Graph
tags: ["infrastructure", "performance", "edge"]
---
```

### 3.2 Semantic Linking with JSON-LD

To ensure the tool is future-proof and interoperable, we can embed **JSON-LD** (JavaScript Object Notation for Linked Data) contexts. By mapping the YAML keys to standard vocabularies (like `schema.org`), we transform the monorepo into a semantic database.

- **Mapping:**
  - `derived_from` $\rightarrow$ `http://schema.org/isBasedOn`
  - `supersedes` $\rightarrow$ `http://schema.org/replacedBy`
  - `author` $\rightarrow$ `http://schema.org/author`

This allows the use of enterprise knowledge graph tools (like Neo4j or generic RDF stores) to ingest the repository and perform complex reasoning, such as transitive closure queries (e.g., "Find all decisions that eventually led to the current database choice, even through 10 steps of supersession").

### 3.3 Handling Soft vs. Hard Links

A critical design decision in the schema is the linking mechanism.

- **Hard Links (UUIDs/Paths):** Used for critical causality (`supersedes: DEC-001`). These must be validated by the linter. If `DEC-001` is deleted, the build fails. This ensures structural integrity.
- **Soft Links (Wikilinks):** Used for contextual exploration (`[[Concept: Latency]]`). These borrow from the "Zettelkasten" method popular in tools like Obsidian and Logseq. The parser should treat these as "related edges" in the graph but not enforce strict existence, allowing for "red links" (concepts that haven't been written yet), which encourages future documentation.

### 3.4 Table: Schema Field Definitions and Validation Rules

| **Field**            | **Type** | **Required?** | **Validation Rule**                              | **Description**                                        |
| -------------------- | -------- | ------------- | ------------------------------------------------ | ------------------------------------------------------ |
| `id`                 | String   | Yes           | Unique, Regex `^DEC-\d{4}-\d{3}$`                | The canonical identifier.                              |
| `status`             | Enum     | Yes           | ``                                               | Lifecycle state. Determines graph visualization color. |
| `supersedes`         | Array    | No            | Must exist in repo. No self-reference.           | The ID of the decision this record invalidates.        |
| `derived_from`       | Array    | No            | Must exist. Cycle detection required.            | The problem or goal that triggered this decision.      |
| `impacts.path`       | String   | No            | Must resolve to a real file/dir in the monorepo. | Links the decision to the implementation artifacts.    |
| `assurance.evidence` | String   | No            | Must be a valid path to a file in `/data`.       | The GSN evidence link.                                 |



------

## 4. Git Monorepo Architecture

The choice of a **monorepo** is the defining constraint of this system. It is not merely a storage location but an active participant in the decision lifecycle. The architecture must handle potentially thousands of decision records alongside gigabytes of source code or business data.

### 4.1 Directory Structure: Fractal vs. Centralized

There are two competing philosophies for structuring decision records in a monorepo: Centralized (all decisions in one folder) and Fractal (decisions co-located with the code they affect).

#### 4.1.1 The Hybrid Approach

We recommend a **Hybrid Architecture** to satisfy both global visibility and local context.

/monorepo-root

├──.github/workflows/          # CI Automation for the tool

├── tools/decision-cli/         # The custom source code for our tool

├── decisions/                  # GLOBAL DECISIONS (Corporate/Life Strategy)

│   ├── strategic/              # "Expand to Asia" / "Buy a House"

│   ├── standards/              # "Use ISO 8601 Dates"

│   └── index.md                # The Root Graph Node

├── apps/

│   ├── finance-backend/

│   │   ├── src/

│   │   └── decisions/          # LOCAL DECISIONS (ADRs specific to this app)

│   │       ├── ADR-001-db-schema.md

│   │       └── ADR-002-api-retry.md

│   └── web-dashboard/

├── data/                       # EVIDENCE (GSN)

│   ├── 2024-financials/

│   └── benchmarks/

└── nx.json                     # Monorepo Build Tool Config

- **Global Decisions:** Housed in the root `/decisions` folder. These set the constraints for the sub-projects (e.g., "All apps must use TypeScript").
- **Local Decisions:** Co-located with the code. This ensures that when a developer (or family member) is working on a specific module (or life project), the context is immediately adjacent.

### 4.2 Leveraging Monorepo Build Tools (Nx / Turborepo)

To manage the scale of a monorepo, standard Git commands are insufficient. We integrate **Nx** or **Turborepo** to treat "documentation" as a first-class build artifact.

#### 4.2.1 The Dependency Graph Extension

Nx and Turborepo build a "Project Graph" to understand code dependencies. We can extend this graph to include decision files.

- **Nx Plugin:** We write a custom Nx plugin that parses the `impacts` field in our decision YAML.
- **Graph Injection:** If `DEC-001` lists `impacts: apps/finance-backend`, the plugin injects a dependency edge: ` -> [Node: apps/finance-backend]`.
- **Intelligent Rebuilds:** Now, if we modify `DEC-001`, Nx knows that `apps/finance-backend` is *affected*. We can trigger specific CI jobs (e.g., "Run Integration Tests" or "Notify Team Lead") solely based on a documentation change. This is the implementation of "Causality" in the build pipeline.

### 4.3 Git Internals: Hooks and Metadata

To tightly couple the decision record to the Git history, we utilize Git's internal mechanisms.

#### 4.3.1 Commit Trailers and Metadata

Git allows for structured metadata in commit messages via "Trailers" (e.g., `Signed-off-by`). We introduce a custom trailer: `Decision-ID`.

- **Policy:** Any commit that alters logic must reference the Decision ID that authorized it.

  feat: implement exponential backoff for API retries

  This prevents thundering herd problems during outages.

  Decision-ID: DEC-2025-042

  Impacts: apps/finance-backend

- **Traceability:** This creates an unbreakable link. Years later, `git log --grep=DEC-2025-042` will reveal every single line of code changed in service of that decision.

#### 4.3.2 Git Hooks for Enforcement

We implement **Git Hooks** (using `husky` or server-side pre-receive hooks) to enforce this policy.

- **Commit-Msg Hook:** Regex validates that if the user is committing to a protected path (e.g., `/src/core`), the message *must* contain a valid `Decision-ID`.
- **Pre-Push Hook:** Runs the linter to ensure that any new Markdown files in `decisions/` have valid Frontmatter and that `supersedes` links point to existing IDs. This prevents "Graph Corruption" at the source.

### 4.4 Table: Git Hook Implementation Strategy

| **Hook Type** | **Execution Context**      | **Trigger**   | **Action Performed**                                         | **Enforced By**  |
| ------------- | -------------------------- | ------------- | ------------------------------------------------------------ | ---------------- |
| `commit-msg`  | Local Developer Machine    | `git commit`  | Validate message format contains `Decision-ID` trailer if core files touched. | Husky / Lefthook |
| `pre-commit`  | Local Developer Machine    | `git commit`  | Run `dec-cli lint` on staged Markdown files. Check YAML syntax. | Husky / Lefthook |
| `pre-push`    | Local Developer Machine    | `git push`    | Run cycle detection on the decision graph. Prevent circular causality push. | Husky            |
| `pre-receive` | Git Server (GitHub/GitLab) | Incoming Push | Verify GPG signatures and decision authorization (e.g., Author is in `approvers` list). | Server Policy    |



------

## 5. The Tooling Ecosystem: Implementation Details

We must now build the actual software agent—the **Decision Causality Engine (DCE)**—that operates within this environment. While off-the-shelf tools like Logseq or Obsidian handle the *editing*, they lack the strict schema validation and CI/CD integration required for a "System of Record."

### 5.1 Building the `causality-cli`

We develop a CLI tool, preferably in **Rust** (for speed in large monorepos) or **TypeScript** (for ecosystem compatibility), to act as the parser and graph builder.

#### 5.1.1 Parsing Logic and AST Transformation

The CLI does not just read text; it constructs an Abstract Syntax Tree (AST) of the decision graph.

1. **Ingest:** Glob all `**/*.md` files.
2. **Parse:** Use `gray-matter` to extract YAML and `remark` to parse the Markdown body.
3. **Graph Construction:** Nodes are instantiated for each decision. Edges are created based on `supersedes`, `derived_from`, and `enables` fields.
4. **Cycle Detection:** The tool runs a generic topological sort (Kahn's algorithm). If a cycle is detected (A supersedes B, B supersedes A), the build **fails**. This is a critical feature: logical paradoxes in decision-making are not permitted.

#### 5.1.2 The "Dead Decision" Detector

Similar to "Dead Code" analysis, the CLI analyzes the graph for isolated subgraphs.

- **Orphaned Decisions:** A decision with no incoming links (no one cites it) and no outgoing links (it cites nothing) and no code impact. These are candidates for deprecation or archiving.
- **Zombie Logic:** A decision marked `Accepted` but which depends on a `Deprecated` parent. The CLI flags this inconsistent state, prompting a review.

### 5.2 Integration with Static Site Generators (SSG)

For consumption, the graph must be rendered into a human-readable format. We leverage **Docusaurus** or **Hugo** for this, as they support "Docs-as-Code" natively.

- **Plugin Architecture:** We write a local plugin for the SSG.
  - **Input:** The JSON graph generated by `causality-cli`.
  - **Transformation:** Injects "Backlinks" sections into every page. If `DEC-002` supersedes `DEC-001`, the page for `DEC-001` gets an automatic banner: "⚠️ This decision has been superseded by DEC-002."
  - **Visualization:** Embeds interactive Mermaid diagrams generated on-the-fly from the graph data.

### 5.3 Local Editing Experience: Obsidian & Logseq

To ensure low friction, the monorepo is configured to work directly with **Obsidian** or **Logseq**.

- **Obsidian Vault:** The repo root is the Vault root. We include a `.obsidian/config` file in the repo to standardize plugins (e.g., Dataview, Templater) for all users.
- **Logseq Compatibility:** Since Logseq uses a slightly different Markdown flavor for block referencing, the `causality-cli` acts as a bridge, normalizing syntax during the build process so that Logseq's "journals" can be linked to formal ADRs without breaking the CI pipeline.

### 5.4 Table: Tooling Compatibility Matrix

| **Feature**           | **Obsidian**            | **Logseq**        | **VS Code (Dendron/Foam)** | **Custom causality-cli**    |
| --------------------- | ----------------------- | ----------------- | -------------------------- | --------------------------- |
| **Graph View**        | Excellent (Native)      | Good (Native)     | Plugin Required            | Generates Static Graph      |
| **Schema Validation** | Weak (Linter Plugin)    | Weak              | Moderate (JSON Schema)     | **Strict (CI/CD Enforced)** |
| **Refactoring**       | Auto-update links       | Auto-update links | Regex-based                | N/A                         |
| **Git Integration**   | Plugin (`obsidian-git`) | Plugin (Beta)     | Native                     | Native                      |
| **Mobile Access**     | Yes                     | Yes               | No                         | Via Generated Site          |



------

## 6. Traceability and Impact Analysis

Traceability is the capability to follow the life of a requirement (or decision) in both a forwards and backwards direction.

### 6.1 Algorithmic Change Impact Analysis (CIA)

When a user proposes a change to a decision, the tool must predict the blast radius.

1. **Inputs:** The diff of the proposed change (e.g., modifying `DEC-010`).
2. **Graph Traversal:** The tool queries the graph for all nodes where `derived_from` includes `DEC-010`.
3. **Recursive Discovery:** It continues down the chain to finding leaf nodes (Code Modules, Business Processes).
4. **Reporting:** The PR is annotated: "Modifying DEC-010 will conceptually impact 3 downstream decisions and 12 code modules. Please verify.".

### 6.2 The Requirements Traceability Matrix (RTM)

Classically, RTMs are massive spreadsheets. In our monorepo, the RTM is a generated view.

- The `causality-cli` generates a matrix mapping **Business Goals** (GSN) $\rightarrow$ **Decisions** (ADR) $\rightarrow$ **Code Implementation** (Impacts).
- **Gap Analysis:** The tool highlights rows where the chain is broken.
  - *Unjustified Code:* Code modules that exist but trace back to no active Decision.
  - *Unfulfilled Goals:* Business goals that trace to no Decisions or Code. This automated audit capability is invaluable for compliance (e.g., SOC2, ISO 27001) where proving traceability is mandatory.

------

## 7. Visualization and User Experience

To make the causal graph comprehensible, we must visualize it. A text list of 1,000 decisions is useless; a map is essential.

### 7.1 The Global Causality Map

We utilize **Graphviz (`dot`)** to generate a static SVG of the entire decision universe.

- **Clustering:** Decisions are clustered by `context` tags (e.g., subgraph "Finance", subgraph "Health").
- **Styling:**
  - *Solid Lines:* Hard causality (`supersedes`, `blocked_by`).
  - *Dotted Lines:* Soft causality (`derived_from`, `enables`).
  - *Color:* Status (Green=Accepted, Grey=Deprecated, Red=Rejected).
- This map allows users to zoom out and see the "tectonic plates" of their decision history moving over time.

### 7.2 Interactive Web Graphs

For the static site, we implement **Cytoscape.js** or **D3 Force Directed Graphs**. Unlike the static Graphviz image, these allow the user to:

- **Filter:** "Show me only decisions related to 'Database' made in 2024."
- **Pathfind:** "Highlight the shortest path between 'Goal: IPO' and 'Task: Refactor Login'."
- **Timeline View:** A slider allows the user to scrub through time, seeing nodes appear and disappear as they traverse the history of the repo.

### 7.3 Embedded Mermaid Diagrams

Within each individual decision file, the tool automatically injects a **Mermaid** flowchart showing the "Local Neighborhood" of that decision. This gives immediate context to the reader without requiring them to leave the page.

Code snippet

```
graph LR
    G -->|Drives| D
    D -->|Impacts| C[Code: User Profile]
    D -->|Supersedes| OLD
    style D fill:#bbf,stroke:#333,stroke-width:2px
```

------

## 8. Case Study Simulations: Life and Business

To demonstrate the universality of this tool, we simulate two scenarios using the exact same infrastructure.

### 8.1 Scenario A: The Microservices Pivot (Business)

- **The Issue:** The engineering team is facing slow deployment cycles (IBIS Issue).
- **The Debate:** A new RFC branch `rfc/microservices` is created.
  - `DEC-100.md` is drafted proposing "Split Monolith."
  - Arguments (Pros/Cons) are added by the team via Pull Request comments.
  - The "Con: Operational Complexity" argument triggers a sub-decision `DEC-101: Adopt Kubernetes` to mitigate the risk.
- **The Decision:** `DEC-100` is Accepted. `DEC-101` is Accepted.
- **The Linkage:** The frontmatter of `DEC-100` lists `enables:`.
- **The Evidence:** A GSN Goal "Maintain 99.9% Uptime" is linked. The evidence path is set to `data/uptime-reports/`.
- **Outcome:** Six months later, uptime drops. The team traces the graph from "Uptime Goal" -> `DEC-101` -> `DEC-100`. They realize the "Operational Complexity" Con was accurate and under-mitigated. They deprecate `DEC-101` and replace it with `DEC-200: Managed PaaS`, updating the `supersedes` link. The history remains intact.

### 8.2 Scenario B: The Relocation (Life)

- **The Issue:** "Where should the family live?" (IBIS Issue).
- **The Debate:**
  - Position A: "Stay in City" (Pro: Short Commute, Con: High Cost).
  - Position B: "Move to Suburb" (Pro: More Space, Con: Long Commute).
- **The Decision:** `DEC-LIFE-050: Move to Suburb` is Accepted.
- **The Causality:** The decision frontmatter includes `derived_from: GOAL-LIFE-001 (Maximize Savings)`.
- **The Impact:** The decision links to `data/finance/mortgage-documents/`.
- **The Future:** Two years later, the user feels unhappy. They check the graph. They see the "Long Commute" Con was noted but dismissed. They realize `GOAL-LIFE-001` (Savings) was prioritized over `GOAL-LIFE-002` (Mental Health). They now have a structured basis to make a *new* decision (e.g., `DEC-LIFE-060: Buy Car with Autopilot`) to mitigate the specific constraint they accepted, rather than blindly regretting the move.

------

## 9. Operational Culture and Governance

A tool is only as effective as the culture that uses it.

### 9.1 The "Decision Review" Ritual

Just as code has Code Review, decisions need Decision Review. The monorepo workflow enforces this.

- **The PR is the Meeting:** Instead of a synchronous meeting to decide, the debate happens in the PR. This forces written articulation of arguments (IBIS), which is clearer than verbal debate.
- **Immutable History:** Once merged, the decision is law. To change it, one must open a new PR. This prevents "Gaslighting" in organizations where leaders claim "we never decided that."

### 9.2 Managing "Decision Debt"

The tool's analytics allow for the management of Decision Debt.

- **Staleness Metrics:** The CLI reports on decisions that haven't been reviewed in >2 years.
- **Density Metrics:** The tool identifies areas of the code with *no* associated decisions (High Risk) or areas with *too many* conflicting decisions (High Complexity).

------

## 10. Conclusion

The construction of a **Decision Causality Engine** within a Git monorepo is a transformative act. It moves an organization or an individual from a state of **Implicit Oral History** to **Explicit Causal Engineering**.

By integrating the argumentation of **IBIS**, the verification of **GSN**, and the structure of **ADRs** into a unified markdown schema, we capture the full fidelity of human choice. By leveraging the **Git Monorepo**, we ensure that this history is atomically coupled with the reality it governs. By building a **Tooling Ecosystem** (`causality-cli`, CI/CD pipelines, Visualization), we make this data actionable, viewable, and alive.

This report has provided the theoretical layout, the technical schema, the architectural diagrams, and the operational workflows required to build this system. It is now up to the user to initialize the repository and make the first commit: `DEC-001: Adopt the Causal Graph`.