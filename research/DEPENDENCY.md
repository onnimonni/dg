# Comprehensive Analysis of Dynamic Dependency Modeling: From Static Graphs to Causal Decision Architectures

## 1. Introduction: The Paradigm Shift in Dependency Modeling

The representation of dependencies—whether in software engineering, supply chain logistics, organizational workflows, or autonomous robotic systems—has traditionally been dominated by the static Directed Acyclic Graph (DAG). In this classical view, nodes represent discrete entities, and edges represent binary, immutable relationships of necessity. A library *requires* a package; a task *precedes* a milestone; a supplier *ships to* a manufacturer. This topological approach, exemplified by software manifest files and basic project management charts, excels at calculating transitive closures and identifying structural bottlenecks. However, as systems increase in complexity and autonomy, the static DAG becomes fundamentally insufficient. It fails to capture the dynamic, conditional, and probabilistic nature of real-world interactions. The modern requirement is to model not just connectivity, but *consequence*—to answer complicated chains of reasoning where the existence of a dependency itself is a function of system state, agent decisions, and external stochasticity.

This report explores the necessary evolution from static structural maps to dynamic **Decision Dependency Graphs**. This transition requires moving beyond the "fallacy of the graph," where visual elegance masks the underlying logic often buried in code , toward formalisms that treat decision-making and state transitions as first-class citizens of the network topology. The challenge involves integrating diverse fields: the probabilistic rigor of **Influence Diagrams** and **Structural Causal Models (SCMs)**; the industrial robustness of **BPMN** and **DMN**; the reactive agility of **Statecharts** and **Hierarchical Task Networks (HTN)**; and the inferential power of **GraphRAG** and modern graph database query languages like **Cypher 25** and **Gremlin**.

We posit that a unified architecture for modeling complicated dependency chains must hybridize these approaches. It must provide a mechanism for **conditional traversal**, where the path through the graph is determined at runtime based on accumulated state; it must support **counterfactual reasoning**, allowing the system to simulate outcomes of actions not taken; and it must facilitate **human-in-the-loop** interaction, where the graph serves as a shared interface for visualization and control. This document provides an exhaustive technical analysis of these methodologies, evaluating their theoretical foundations, engineering implementations, and practical applications in high-stakes environments.

------

## 2. Theoretical Foundations of Decision and Causal Modeling

To construct a graph capable of answering "what if" questions and navigating complex decision trees, we must first establish a rigorous theoretical framework. The limitations of simple dependency tracking are mathematical: standard graph theory deals with connectivity, whereas decision modeling deals with probability, utility, and causality.

### 2.1 Beyond the Static DAG: The Necessity of Conditional Logic

The traditional dependency graph, often visualized as a web of interconnected nodes, provides a snapshot of potential relationships. In software supply chains, for instance, the dependency graph aggregates manifest files to show all possible packages a project might use. However, this "build-time" view often diverges from the "runtime" reality. A conditional branch in code (`if (x) import A else import B`) means that the effective dependency graph is a function of the variable $x$. Standard graphs do not capture this; they model the superset of all possibilities, leading to bloated and inaccurate representations of risk and reachability.

This limitation is described as the "fallacy of the graph," where the diagram suggests a clean flow of logic, but the actual decision-making criteria—the "business logic"—are hidden in opaque snippets of code attached to nodes. To model a dependency chain that truly reflects decision-making, the graph formalism itself must support **conditional edges** and **state-dependent topology**. This means the graph is no longer a static artifact but a dynamic system where $G_t = f(G_{t-1}, State_t, Input_t)$. This dynamic re-wiring capability is essential for accurately modeling phenomena like supply chain ripple effects, where a disruption in one node dynamically activates backup dependencies that did not exist in the nominal state.

### 2.2 Probabilistic Graphical Models (PGMs)

When dependencies are uncertain—for example, if a server *might* fail or a supplier *might* delay—we turn to Probabilistic Graphical Models.

#### 2.2.1 Bayesian Networks: Inference vs. Decision

Bayesian Networks (BNs) extend the DAG by annotating nodes with Conditional Probability Distributions (CPDs). A node $X$ is dependent on its parents $Pa(X)$, defined by $P(X | Pa(X))$. BNs excel at **belief propagation**: observing a symptom in a leaf node allows the system to infer the probable state of root causes. This is widely used in fault diagnosis and risk assessment.

However, BNs suffer from a critical limitation regarding the user's query: they model the *environment*, not the *agent*. A BN can calculate the probability of an outcome, but it does not explicitly model the *choice* to intervene. In a BN, a variable taking a value is treated as an observation (evidence). In decision making, we need to distinguish between seeing a value and forcing a value. Furthermore, BNs lack a native concept of "utility" or "consequence" in terms of preference; they only track likelihood.

#### 2.2.2 Influence Diagrams (Decision Networks)

To address the gap left by BNs, **Influence Diagrams (IDs)** were developed as a generalization of Bayesian Networks specifically for decision analysis. They provide a precise language for sequential decision-making problems, which aligns perfectly with the requirement to answer "complicated chains with decision making".

The anatomy of an Influence Diagram distinguishes between three types of nodes, creating a semantic richness unavailable in standard dependency graphs:

1. **Chance Nodes (Ovals):** These represent uncertain variables, identical to nodes in a BN. They are governed by probabilistic dependencies on their parents.
2. **Decision Nodes (Rectangles):** These represent choices available to the decision-maker. Unlike chance nodes, they have no CPDs. Their value is determined by an optimization policy. The parents of a decision node represent the **information set**—the specific data available to the agent at the exact moment the decision is made. This explicitly models the temporal dependency of information.
3. **Utility/Value Nodes (Diamonds):** These represent the consequences or objectives. They quantify the preference for outcomes, encoding the cost-benefit analysis of the dependency chain. A utility node is a deterministic function of its parents, mapping the configuration of chance and decision nodes to a real number (the utility).

The power of the ID lies in its ability to compute the **Maximum Expected Utility (MEU)**. By unrolling the graph, algorithms can determine the optimal sequence of decisions that maximizes the final utility, effectively "solving" the dependency chain rather than just traversing it. This makes IDs the theoretical gold standard for strategic planning in uncertain environments.

### 2.3 Structural Causal Models (SCMs) and The Ladder of Causation

While Influence Diagrams handle decision making under uncertainty, they often assume a fixed statistical reality. To fully address the requirement for modeling "consequences," particularly in the context of root cause analysis and "what-if" scenarios (counterfactuals), we must ascend Judea Pearl's **Ladder of Causation**.

#### 2.3.1 Association, Intervention, and Counterfactuals

The distinction between observing a dependency and manipulating it is formalized in the **do-calculus**.

- **Association ($P(y|x)$):** What does finding $X=x$ tell me about $Y$? This is the level of standard BNs and correlation.
- **Intervention ($P(y|do(x))$):** What if I *do* $X=x$? The $do$ operator represents a physical intervention that breaks the natural dependency structure. If we force a server to reboot, we break the causal link from its previous state history. This is critical for modeling the consequences of decisions in a dependency graph.
- **Counterfactuals ($P(y_x|x', y')$):** This is the most complex level, asking: "Given that we observed $X=x'$ and $Y=y'$, what *would have* happened if we had done $X=x$?" This retrospective reasoning is essential for failure analysis in complex chains (e.g., "Would the system have crashed if we had doubled the cache size?").

#### 2.3.2 The Twin Network Algorithm

Modeling counterfactuals requires a specific graph transformation known as the **Twin Network**. This approach effectively creates two parallel universes within the model:

1. **The Factual World:** Representing the observed reality, used to update the beliefs about the exogenous background variables (the "noise" terms $U$) that drive the system's stochasticity.
2. **The Counterfactual World:** A copy of the graph where the intervention is applied ($do(X=x)$). This world shares the same background variables ($U$) as the factual world—this sharing is what links the "what if" to the "what happened."

By performing inference over this combined structure, systems can provide rigorous answers to questions about alternative consequences in dependency chains. This goes far beyond simple simulation; it uses the specific context of an event to tailor the prediction of its alternative.

### 2.4 Petri Nets vs. Statecharts

In the domain of rigorous systems modeling, particularly for concurrency and synchronization dependencies, the choice often lies between Petri Nets and Statecharts.

- **Petri Nets:** These are mathematical modeling languages for the description of distributed systems. They are excellent for analyzing deadlock and resource contention but can become unwieldy ("state explosion") for complex decision logic.
- **Statecharts:** An extension of Finite State Machines (FSMs), Statecharts introduce hierarchy (super-states) and orthogonality (parallel regions). This allows for a more compact and readable representation of complex dependency logic. For decision modeling, Statecharts are often preferred because they align closely with the mental model of "modes" (e.g., "Emergency Mode" vs. "Normal Operation"), where dependencies change based on the active mode.

------

## 3. Engineering Architectures for Decision Workflows

The theoretical models described above are abstract. To operationalize them in software and industrial systems, they are implemented through specific architectural patterns and standards. The following section analyzes how modern engineering frameworks instantiate dynamic dependency graphs.

### 3.1 Enterprise Decision Modeling: BPMN and DMN Integration

In the corporate and industrial sectors, the standard for modeling workflows is **BPMN (Business Process Model and Notation)** combined with **DMN (Decision Model and Notation)**. This pairing explicitly decouples the "process chain" from the "decision logic," a critical separation of concerns for maintainable systems.

#### 3.1.1 BPMN: The Control Flow Skeleton

BPMN visualizes the dependency chain as a sequence of activities, events, and gateways. It answers the "who, when, and where" of the workflow.

- **Gateways:** The `Exclusive Gateway` (XOR) and `Inclusive Gateway` (OR) are the primary mechanisms for structural decision making within the flow. They direct the token (the process instance) down specific paths based on data dependencies.
- **Limitations:** While BPMN handles the sequence, embedding complex boolean logic or probability calculations directly into BPMN gateways leads to rigid, hard-to-read diagrams. This is where DMN becomes essential.

#### 3.1.2 DMN: The Logic Engine

DMN models the "why and how" of a decision. A **Decision Requirements Diagram (DRD)** is a dependency graph specifically for logic. In a DRD:

- **Nodes** are Decisions (rectangles) or Input Data (ovals).
- **Edges** represent Information Requirements (solid arrows) or Knowledge Requirements (dashed arrows).

This structure allows for **chained decisions**. For example, a decision "Determine Fraud Risk" might depend on "Verify Identity" and "Check Transaction History." The output of "Determine Fraud Risk" then becomes an input for "Approve Loan." This explicit chaining allows the system to trace the *provenance* of a consequence back through the logical dependencies.

**Decision Tables:** The core of a DMN node is often a Decision Table, which maps combinations of inputs to outputs in a rigorous, spreadsheet-like format. This ensures that the "consequences" of any input combination are deterministic and verifiable, a requirement for regulatory compliance.

### 3.2 Hierarchical Task Networks (HTN) and Goal-Oriented Action Planning (GOAP)

In the fields of robotics and game AI, agents must autonomously navigate dependency chains to achieve goals. Here, the graph is not just traversed; it is *constructed* dynamically.

#### 3.2.1 Goal-Oriented Action Planning (GOAP)

GOAP utilizes a regressive planning approach. The agent starts with a **Goal** (e.g., "Repair Generator") and searches backward through the dependencies (Preconditions) of available actions.

- **Dynamic Chaining:** If "Repair Generator" requires "Has Wrench," the planner searches for an action that produces "Has Wrench" (e.g., "Pickup Wrench"). If "Pickup Wrench" requires "At Tool Shed," it adds a movement action.
- **Consequence Optimization:** The planner can evaluate multiple valid chains (plans) and select the one with the lowest "cost" (lowest consequence). This makes GOAP highly effective for creating "complicated chains" of behavior that appear intelligent and adaptive.

#### 3.2.2 Hierarchical Task Networks (HTN)

HTN planning decomposes high-level "Compound Tasks" into sequences of "Primitive Tasks" via "Methods."

- **Method Selection:** A compound task like "Transport Package" might have methods "By Truck" or "By Drone." The choice of method is a decision point governed by preconditions (e.g., "Package Weight < 2kg").
- **Dependency Enforcement:** HTNs are powerful because they enforce constraints at different levels of abstraction. A dependency at the high level (e.g., "Must have clearance") implicitly constrains all lower-level actions, ensuring that the generated chain is valid and coherent.
- **Hybrid Approaches:** Recent research suggests combining HTN (for strategic, long-term dependency management) with **Behavior Trees** (for reactive, low-level control). This hybrid architecture allows an agent to follow a long dependency chain while remaining robust to immediate environmental changes.

### 3.3 Statecharts and the Actor Model (XState)

For user interfaces and event-driven systems, dependency modeling often focuses on valid state transitions. **XState** is a prominent library that implements Statecharts and the Actor Model for modern web development.

#### 3.3.1 The Actor Model

In XState v5, the graph is composed of **Actors**. An Actor is an independent unit of state and logic that communicates via messages.

- **Dependency via Communication:** Dependencies are modeled as communication channels between actors. A "Parent" actor might spawn a "Child" actor to handle a sub-process. The Parent depends on the Child's completion or error signal to proceed.
- **Dynamic Topology:** The graph of actors is not static; actors can spawn and stop other actors at runtime based on decisions (events). This creates a highly dynamic dependency graph that can scale to model complex workflows.

#### 3.3.2 Guarded Transitions

Statecharts enforce decision making through **Guard Conditions**. A transition from State A to State B is only valid if the `cond` (condition) function returns true. This explicitly models the "decision" to traverse an edge, integrating the logic directly into the graph topology.

### 3.4 Agentic Workflows and Cyclic Graphs (LangGraph)

The advent of Large Language Models (LLMs) has given rise to "Agentic" workflows, where the decision logic is provided by a probabilistic AI model. **LangGraph** is a framework designed specifically for this, introducing **Cyclic Graphs** to the dependency modeling landscape.

#### 3.4.1 Cycles for Iterative Reasoning

Traditional dependency graphs are often acyclic (DAGs). However, intelligent decision-making is iterative: Plan $\rightarrow$ Execute $\rightarrow$ Observe $\rightarrow$ Refine Plan. LangGraph explicitly supports cycles, allowing the dependency chain to loop back on itself until a termination condition is met.

- **State Persistence:** The key to managing cycles is the shared **State Schema**. In LangGraph, a `State` object (e.g., containing message history, tool outputs) is passed between nodes. Each node modifies this state.
- **Conditional Edges:** LangGraph uses `add_conditional_edges` to define branching logic. Instead of a hard-coded next step, the graph executes a function (often an LLM call) to inspect the State and decide the next node. This allows for "dynamic routing" where the dependency chain adapts in real-time to the complexity of the task.

------

## 4. Data Persistence and Query Languages

Modeling these complex graphs conceptually is distinct from storing and querying them efficiently. The choice of database and query language dictates the system's ability to traverse "complicated chains" and evaluate conditions at scale.

### 4.1 Storage Paradigms: LPG vs. RDF

The two dominant models for graph data are **Labeled Property Graphs (LPG)** and **Resource Description Framework (RDF)**.

#### 4.1.1 Labeled Property Graphs (e.g., Neo4j)

LPGs allow nodes and edges to have internal structure (properties).

- **Decision Modeling:** This is crucial for decision graphs. An edge representing a dependency can carry properties like `weight`, `latency`, `cost`, or `probability`.
- **Performance:** LPGs generally offer superior performance for deep traversals (finding long chains) because they use "index-free adjacency," where connected nodes physically point to each other in storage. This makes them ideal for operational decision systems that need to traverse thousands of dependencies in milliseconds.

#### 4.1.2 RDF and Semantic Reasoning

RDF graphs are composed of subject-predicate-object triples. They excel at **interoperability** and **inference**.

- **Ontological Inference:** Using OWL (Web Ontology Language), an RDF store can *infer* dependencies that are not explicitly stored. For example, if "Service A depends on Database" and "PostgreSQL is a Database," a reasoner can infer that Service A has a dependency on the class of PostgreSQL instances.
- **Complexity:** While powerful for knowledge representation, RDF can be verbose and slower for the types of algorithmic traversals required for real-time decision making.

### 4.2 Advanced Traversal: Cypher 25 and Gremlin

To answer the user's request for "answering complicated chains," we need query languages that support algorithmic logic within the traversal.

#### 4.2.1 Cypher 25: Declarative Decision Logic

The latest iteration of Neo4j's query language, **Cypher 25**, introduces revolutionary features for decision modeling :

- **Quantified Path Patterns (QPP):** This allows users to define complex, repeating patterns in a path (e.g., `((:Task)-->(:Task))+`).
- **Stateful Traversal (`allReduce`):** This feature is critical for "consequence" analysis. It allows the query to maintain a "running state" as it traverses.
  - *Example:* Traversing a supply chain graph to find a route. With `allReduce`, the query can accumulate "Total Cost" or "Battery Level."
  - *Conditional Pruning:* Crucially, the traversal can *conditionally prune* paths based on this accumulated state. If `Battery Level` drops below zero, the database stops exploring that branch *during the traversal*, vastly improving performance over post-filtering.
- **Conditional Subqueries:** The `CALL {... }` syntax allows for branching logic (IF/ELSE) directly within the query, enabling the graph database to execute decision logic without returning data to the application layer.

#### 4.2.2 Gremlin: Imperative Traversal

Gremlin (Apache TinkerPop) is a functional, fluent language that allows for imperative graph traversal.

- **`choose()` and `branch()`:** These steps act as switch statements, directing the traverser to different parts of the graph based on node properties.
- **`sack()`:** Gremlin's `sack` operator is the imperative equivalent of `allReduce`. A "sack" is a local variable carried by the traverser. As it moves through the dependency chain, it can modify the sack (e.g., `sack(mult).by('risk_factor')`). This allows for the calculation of compound consequences (like cumulative risk) along a chain.

------

## 5. Causal Inference and GraphRAG

To fully satisfy the requirement of modeling "consequences," we must move beyond deterministic graph traversal to **Causal Inference**. This involves understanding not just what happens, but *why*, and what would happen under different conditions.

### 5.1 Causal Inference Libraries

Several Python ecosystems have matured to support causal modeling on graphs.

#### 5.1.1 DoWhy

**DoWhy** is a Python library that unifies causal inference under a single API. It explicitly models the "Assumptions" of a causal graph.

- **Workflow:**
  1. **Model:** Define the Causal Graph (using NetworkX or GML).
  2. **Identify:** Use graph criteria (backdoor/frontdoor) to identify the causal effect.
  3. **Estimate:** Compute the effect using statistical methods.
  4. **Refute:** This is unique to DoWhy. It tests the robustness of the model by adding random common causes or placebo treatments. If the estimated effect changes significantly, the dependency model is likely flawed.

#### 5.1.2 PyCID (Causal Influence Diagrams)

**PyCID** is specialized for decision-making contexts involving agents and incentives.

- **Game Theory Integration:** It extends standard Influence Diagrams to **Multi-Agent Causal Influence Diagrams (MACIDs)**. This is essential for modeling dependency chains where multiple independent actors (e.g., competing suppliers, adversarial agents) make decisions that affect the outcome.
- **Nash Equilibria:** PyCID can compute Nash Equilibria in these graphs, predicting the stable outcome of a complex interaction of decisions.

### 5.2 GraphRAG: Automated Graph Construction

Manually building these complex dependency graphs is often unfeasible for large datasets. **GraphRAG (Retrieval-Augmented Generation)** leverages LLMs to automate this process.

#### 5.2.1 Architecture of GraphRAG

GraphRAG transforms unstructured text into a structured knowledge graph:

1. **Source Documents:** Ingests reports, logs, or documentation.
2. **LLM Extraction:** An LLM processes chunks of text to identify entities (Nodes) and relationships (Edges), along with "claims" or "covariates" (properties).
3. **Community Detection:** Algorithms like Leiden are used to detect hierarchical communities within the graph.
4. **Summarization:** The system generates summaries for each community.

#### 5.2.2 Causal Reasoning with GraphRAG

Recent advancements (**GraphRAG-Causal**) extend this to specific causal detection.

- **Method:** The LLM is prompted to identify specific causal language (e.g., "precipitated," "led to," "resulted from") and map these to directed edges in the graph.
- **Hybrid Retrieval:** When a user asks a question ("What are the consequences of X?"), the system uses a hybrid approach: vector search finds semantically relevant nodes, while graph traversal follows the causal edges to retrieve the full "chain of events." This grounded retrieval significantly reduces hallucinations compared to standard RAG.

------

## 6. Visualization and Human-Interaction

For "complicated chains," the graph is not just a computational structure but a communication tool. Effective visualization is required to help humans understand decision logic and consequences.

### 6.1 Visualizing Causal Chains and Counterfactuals

Standard node-link diagrams often become "hairballs" at scale. Specialized visualization techniques are required.

- **Interactive Decision Maps:** Tools like **CausalSynth** allow users to visualize the distribution of variables and interactively "drag" nodes to simulate interventions. This visualizes the *derivative* of the dependency chain—showing how sensitive an outcome is to a specific input.
- **Counterfactual Visualization:** To visualize "what if," systems can display "ghost paths"—faint, alternative branches in the graph that represent the counterfactual trajectory alongside the actual execution path. This technique helps users understand the "road not taken".

### 6.2 Human-in-the-Loop (HITL) UX

For high-stakes decisions, the graph must support human intervention.

- **Breakpoints and Signals:** In workflow engines like Temporal or LangGraph, the dependency chain can be designed with "Breakpoint" nodes. The execution pauses, the state is persisted, and a signal is awaited.
- **UX Design:** The user interface for HITL should not just show a "Approve/Reject" button. It should present the **Context Subgraph**—the specific upstream dependencies and downstream predicted consequences that led to the decision point. This empowers the human to make an informed decision rather than a blind one.

------

## 7. Applications and Case Studies

The theoretical and engineering concepts discussed manifest in critical industrial applications.

### 7.1 Supply Chain Resilience: Modeling the Ripple Effect

The "Ripple Effect" describes how a localized disruption propagates through a supply chain, often amplifying in impact.

- **Graph Model:** Nodes represent suppliers, warehouses, and transport links. Edges represent material flow.
- **Simulation vs. Optimization:** While optimization models (Linear Programming) find the most efficient static state, **Simulation Modeling** (Agent-Based or Discrete Event) is required to understand the ripple effect. Simulation runs the dependency graph through time, allowing distinct decision policies (e.g., "Order from backup if inventory < 10%") to interact dynamically.
- **Quantitative Metrics:** These models calculate metrics like **Time-to-Recovery (TTR)** and **Value-at-Risk (VaR)**, providing a quantitative measure of the "consequence" of specific dependency structures.

### 7.2 Neuro-Symbolic AI: The Future of Dependency Modeling

The frontier of this field lies in **Neuro-Symbolic AI**, which combines the pattern-matching power of Neural Networks with the reasoning structure of Symbolic Graphs.

- **Concept:** A neural network might process raw sensor data (e.g., a camera feed) to detect a "Fire" symbol. This symbol is then fed into a symbolic dependency graph (Influence Diagram) to reason about the consequences and trigger the correct "Extinguish" decision.
- **Benefit:** This approach provides the robustness of deep learning with the interpretability and guarantee of symbolic logic, addressing the "black box" problem in critical decision chains.

------

## 8. Conclusion

Modeling a graph of dependencies that can answer complicated chains with decision making and consequences requires a fundamental departure from the static DAG. It demands a **Composite Architecture**:

1. **Topology:** A **Labeled Property Graph** (Neo4j) to store the intricate web of entities and their stateful properties.
2. **Logic:** A formal **Influence Diagram** or **DMN** layer to mathematically define the rules of choice and utility.
3. **Dynamics:** An **Agentic Workflow Engine** (LangGraph, XState) to execute the graph, handling cycles, state persistence, and conditional routing.
4. **Causality:** A **Structural Causal Model** (DoWhy) to perform the rigorous "twin network" calculations required for counterfactual reasoning.

By integrating these disparate technologies, we move from simply mapping what is connected to deeply understanding how systems behave, fail, and evolve.

------

## Technical Appendix: Comparative Framework Analysis

Table 1 summarizes the capabilities of the primary frameworks discussed, providing a quick reference for architects selecting the appropriate tool for their specific dependency modeling needs.

| **Framework**                | **Primary Domain**      | **Decision Support**       | **Consequence Modeling**            | **Graph Topology**           |
| ---------------------------- | ----------------------- | -------------------------- | ----------------------------------- | ---------------------------- |
| **Bayesian Networks**        | Data Science, Diagnosis | Low (Inference only)       | Probabilistic belief update         | DAG (Acyclic)                |
| **Influence Diagrams**       | Decision Analysis       | High (MEU, Utility)        | Expected Utility, Value of Info     | DAG + Decision/Utility Nodes |
| **BPMN + DMN**               | Business Process        | High (Deterministic)       | Rule-based outcomes                 | Flowchart + Logic Trees      |
| **HTN / GOAP**               | Robotics, Game AI       | High (Planning)            | Goal satisfaction, Cost opt.        | Dynamic/Constructed Tree     |
| **Statecharts (XState)**     | UI, Embedded Systems    | High (Guarded Transitions) | State transitions                   | Cyclic, Hierarchical         |
| **LangGraph**                | LLM Agents              | High (Probabilistic/LLM)   | Simulation via Generative AI        | Cyclic, State-Schema driven  |
| **Structural Causal Models** | Etiology, Research      | Medium (Intervention)      | **Counterfactuals ($do$-calculus)** | DAG + Error Terms            |