# Architectural Impact Analysis: Tools and Algorithms for Measuring the Blast Radius of Codebase Changes

## 1. Executive Summary: The Architecture-Implementation Gap

In the lifecycle of large-scale software systems, architectural decisions are pivotal moments that shape the trajectory of a codebase. These choices are formally captured in Architectural Decision Records (ADRs). However, as systems evolve, the link between the static prose of an ADR (e.g., "ADR-007: Leverage AWS services") and the dynamic, living codebase often erodes. When a decision must be reversed or deprecated—such as migrating from AWS to Azure due to cost considerations—engineers face a critical blind spot: the "blast radius" of that change is unknown.

The "blast radius" in software engineering refers to the comprehensive scope of components, modules, and dependencies that will be destabilized by a specific modification. When the modification is architectural—such as removing a foundational cloud provider dependency—the blast radius is rarely contained within a single module. It permeates through direct imports, transitive dependencies, configuration files, and, most elusively, implicit semantic couplings where code relies on the *behavior* of a technology without explicitly naming it.

This research report provides an exhaustive analysis of methods, tools, and algorithms to re-establish the link between ADRs and code, enabling precise impact analysis within a local Rust-based Command Line Interface (CLI) environment. The objective is to design a system that answers not only explicit queries ("Where is ADR-007 referenced?") but also implicit and ambiguous ones ("Why are we using Redis?", "What breaks if we remove AWS support?").

The recommended architecture is a **Hybrid Tiered Analysis System** designed for local execution without heavy external dependencies. It progresses through three layers of fidelity:

1. **Tier 1: Lexical & Syntactic Analysis:** Utilizing high-performance text search (`ripgrep` engines) and concrete syntax tree parsing (`tree-sitter`) to identify explicit markers and structural patterns.
2. **Tier 2: Deterministic Structural Analysis:** Leveraging Language Server Protocol (LSP) data via `rust-analyzer` and SCIP (Source Code Intelligence Protocol) to build precise call graphs and dependency trees, quantifying the structural blast radius.
3. **Tier 3: Semantic & Vector-Based Analysis:** Employing local Large Language Models (LLMs) via the `candle` framework and embedded vector databases (`LanceDB`) to perform Retrieval-Augmented Generation (RAG). This layer resolves ambiguous queries by linking code implementations to the semantic intent described in ADRs.

This report details the theoretical underpinnings, algorithmic approaches, and practical implementation strategies for building such a tool in Rust, ensuring it remains performant on a standard developer machine while scaling capabilities as hardware resources allow.

------

## 2. Theoretical Foundations of Architectural Blast Radius

### 2.1 The Nature of Architectural Knowledge Management (AKM)

Architectural Knowledge Management (AKM) is the discipline of capturing the reasoning behind design choices to prevent knowledge vaporization over time. An Architectural Decision Record (ADR) is the primary artifact in this domain. It typically consists of a context, a decision, and consequences. For example, `ADR-007` might state: "Context: High storage costs. Decision: Use AWS S3. Consequence: Vendor lock-in."

While the ADR captures the intent at time $t=0$, the codebase evolves to $t=n$. The "Architecturally-Significant Requirements" (ASRs) embedded in the code—such as the requirement to handle S3 bucket policies—become diffused. The "blast radius" of deprecating `ADR-007` is effectively the volume of code $V_{impact}$ that satisfies the condition:

$$V_{impact} = \{ c \in Codebase \mid \text{DependsOn}(c, \text{Decision}(ADR_{007})) \}$$

The difficulty lies in defining the predicate $\text{DependsOn}$. It is not merely a syntactic import; it includes logical dependencies (e.g., error handling logic specific to S3 consistency models).

### 2.2 Defining the Blast Radius in Code

In distributed systems and modular monoliths, the blast radius is the extent to which a failure or change in one component affects the rest of the system. Minimizing blast radius is a design goal for resilience, but quantifying it is an analysis goal for refactoring.

We classify dependencies into four categories of visibility, each requiring different detection strategies:

| **Dependency Type**   | **Definition**                                               | **Detection Difficulty** | **Example**                              | **Recommended Tooling** |
| --------------------- | ------------------------------------------------------------ | ------------------------ | ---------------------------------------- | ----------------------- |
| **Explicit Tagged**   | Code explicitly marked with metadata linking it to an ADR.   | Low                      | `// ADR-007: Secrets config`             | `grep-searcher`         |
| **Direct Structural** | Code that imports or calls libraries mandated by the ADR.    | Medium                   | `use aws_sdk_s3::Client;`                | `scip`, `rust-analyzer` |
| **Transitive**        | Code that depends on the Direct Structural code.             | High                     | `fn my_func() { s3_wrapper.upload() }`   | `stack-graphs`          |
| **Semantic/Implicit** | Code implementing logic dictated by the ADR's rationale, without explicit links. | Very High                | A Retry mechanism tuned for AWS latency. | `candle`, `LanceDB`     |



### 2.3 The "Deprecation Cliff"

When an engineer proposes replacing a foundational technology (e.g., AWS to Azure), they face a "Deprecation Cliff." They can see the edge (the explicit imports), but they cannot see the bottom (the depth of the dependency chain). Existing tools like SonarQube or SaaS-based generic analysis tools often lack the context of *specific* ADRs. They report "code smells" or "security vulnerabilities" , but they rarely report "Architecture Violations" unless custom rules are written. Furthermore, relying on remote SaaS tools introduces latency and privacy concerns. A local Rust CLI provides the necessary speed and security.

------

## 3. Tier 1: Lexical and Syntactic Analysis (The Lightweight Layer)

The first line of defense in impact analysis is identifying explicit references. This tier prioritizes speed and low resource usage, suitable for running on every commit or as a quick CLI check. It answers the question: "Where is the text 'ADR-007' or the concept 'AWS' explicitly written?"

### 3.1 High-Performance Text Search with `grep-searcher`

For a Rust-based CLI, the gold standard for text search is the engine powering `ripgrep`. The `grep-searcher` and `grep-regex` crates provide a library interface to this engine, allowing for line-oriented searching that vastly outperforms standard iteration.

#### 3.1.1 Memory Mapping and Stream Processing

The `grep-searcher` crate allows the CLI to employ memory-mapped files (`mmap`). This technique maps the file's contents directly into the process's virtual memory space, allowing the operating system to handle paging. For large codebases (gigabytes of source code), this eliminates the overhead of repeated system calls for reading buffers. The `Searcher` struct can be configured with a `MmapChoice`, allowing the tool to heuristically determine when to use memory maps based on file size.

**Implementation Strategy:**

The tool instantiates a `Searcher` and a `Sink`. The `Sink` is a trait that receives matches.

Rust

```
// Conceptual Rust Implementation
let matcher = RegexMatcher::new(r"ADR-007|aws_sdk").unwrap();
let mut searcher = SearcherBuilder::new().build();
searcher.search_path(&matcher, "src/lib.rs", MySink::new())?;
```

The `MySink` implementation collects `SinkMatch` objects, which contain the line number and the byte offset. This provides the initial "seed" locations for the blast radius.

### 3.2 Syntactic Parsing with Tree-sitter

Text search is brittle; it cannot distinguish between a comment, a string literal, or active code. To understand the *structure* holding the ADR reference, we require a Concrete Syntax Tree (CST). `tree-sitter` is the industry standard for this, offering robust, error-tolerant parsing with Rust bindings.

#### 3.2.1 The Problem of Comment Association

One of the most difficult tasks in static analysis is associating comments with code nodes. In many Abstract Syntax Trees (ASTs), comments are discarded as whitespace. `tree-sitter` preserves them in the CST, but they appear as sibling nodes to the function or struct they describe, rather than children or attributes.

To associate `// ADR-007: Use S3` with the function `fn upload_artifact()`, we utilize Tree-sitter's S-expression query language. We must define a query that looks for a comment node immediately preceding a definition node.

**Tree-sitter Query Strategy:**

The query uses the `.` operator to enforce immediate precedence (adjacency) between nodes.

Scheme

```
(
  (line_comment) @adr_comment
 .
  [
    (function_item)
    (struct_item)
    (impl_item)
    (mod_item)
  ] @definition
  (#match? @adr_comment "ADR-007")
)
```

This query instructs the Tree-sitter engine to find a `line_comment` that is immediately followed by a `function_item`, `struct_item`, etc., where the comment text matches the regex "ADR-007".

#### 3.2.2 Extracting Scope and Granularity

Once a match is found, `tree-sitter` allows the tool to determine the *granularity* of the impact.

- **Module Level:** If the comment adheres to a `mod` declaration, the blast radius encompasses the entire file or directory.
- **Struct Level:** If attached to a `struct`, the blast radius includes the struct definition *and* all `impl` blocks associated with it.
- **Function Level:** If attached to a `function`, the radius is initially limited to that scope.

By traversing the CST using cursors (`tree_sitter::Cursor`), the CLI can efficiently determine the byte range of the impacted block. This range is critical for the next tier of analysis.

### 3.3 Limitations of Tier 1

While fast, Tier 1 suffers from low recall for transitive dependencies. If Function A is tagged `// ADR-007` but Function B calls Function A without a tag, Tier 1 will miss Function B. This necessitates a structural analysis layer.

------

## 4. Tier 2: Deterministic Structural Analysis (The Precise Layer)

While Tier 1 finds *where* the ADR is mentioned, Tier 2 determines *what* relies on those mentions. This involves analyzing the call graph and type dependency graph. For a Rust ecosystem, this is best achieved by interfacing with compiler data.

### 4.1 Leveraging Rust-Analyzer as a Library

`rust-analyzer` (RA) is the primary language server for Rust. While typically used as a binary via LSP, its internal crates (`ra_ap_ide`, `ra_ap_hir`, `ra_ap_syntax`) are published on crates.io and can be consumed as libraries. This allows a local CLI to load a Cargo workspace and perform semantic queries programmatically.

#### 4.1.1 The Salsa Database Architecture

RA uses `salsa`, a query framework for on-demand, incremental computation. For our blast radius tool, we construct a `RootDatabase` and load the user's workspace. We can then issue queries such as `find_all_references` for a specific symbol.

**Operational Workflow:**

1. **Symbol Resolution:** From Tier 1, we identify a function `upload_artifact` at `src/s3.rs:45`.
2. **LSP Query:** The tool converts this file/line pair into a `FilePosition`.
3. **Find Usages:** It invokes `analysis.find_all_references(position)`. The RA database traverses the graph and returns a list of `Reference` objects.
4. **Transitive Closure:** For every reference found (e.g., `fn process_data` calls `upload_artifact`), the tool performs a recursive search to find callers of `process_data`.

**Algorithmic Complexity:**

The `find_all_references` operation in a large workspace can be computationally expensive (O(N) where N is the number of files, although indices optimize this). However, because RA is incremental, subsequent queries are faster.

### 4.2 SCIP: A Portable Indexing Standard

Loading the full `rust-analyzer` database into a CLI tool can result in high memory usage (gigabytes of RAM). A more lightweight approach for the "consumer" tool is to generate an offline index using **SCIP (Source Code Intelligence Protocol)**. SCIP is a language-agnostic format for indexing code, recording definitions, references, and documentation.

#### 4.2.1 Generating and Querying SCIP

The workflow separates the heavy lifting (indexing) from the querying.

1. **Index Generation:** The user runs `rust-analyzer scip. > index.scip`. This binary uses the full power of the compiler to resolve types and writes the result to a Protobuf file.
2. **Index Consumption:** Our `adr-blast` CLI reads `index.scip`. This file contains a graph where nodes are symbols and edges are relationships (Definition, Reference, Implementation).
3. **Graph Traversal:** The CLI uses the `scip` crate to deserialize the index. It constructs an in-memory directed graph (using `petgraph`).

**Why SCIP for Blast Radius?**

- **Decoupling:** The analysis tool doesn't need to compile the code itself; it just reads the map.
- **Speed:** Graph algorithms on the in-memory graph (BFS/DFS) are instantaneous compared to on-demand compiler queries.
- **Portability:** If the user has a polyglot repo (Rust + TypeScript), SCIP indexers for both can be merged, allowing the blast radius to be traced across language boundaries (e.g., a React frontend calling a Rust backend API).

### 4.3 Advanced Name Resolution with `stack-graphs`

For scenarios where exact resolution is needed but a full SCIP index is too heavy or stale, GitHub's `stack-graphs` library offers a compelling alternative. `stack-graphs` is a Rust crate designed for incremental data dependency analysis.

It represents name binding (variable `x` refers to definition `x` in module `M`) as a path-finding problem in a graph.

- **Nodes:** Scope boundaries, definitions, and references.
- **Edges:** Lexical nesting and import statements.
- **Algorithm:** It maintains a symbol stack and a scope stack during traversal. A path is valid if the stacks are empty or compatible at the end.

Using `stack-graphs` allows the CLI to answer: "If I delete this definition in `lib.rs`, which specific imports in `main.rs` become invalid?" with incrementally updateable graphs.

### 4.4 Call Graph Algorithms for Blast Radius

Once the dependency graph is built (via RA or SCIP), determining the blast radius becomes a graph reachability problem. We apply a modified **PageRank** or **Impact Propagation** algorithm.

**Algorithm 1: Weighted Impact Propagation**

Let $G = (V, E)$ be the dependency graph. Let $S \subset V$ be the set of "Source Nodes" identified in Tier 1 (explicitly tagged ADR code).

1. **Reverse Graph:** Construct $G^R$ by reversing all edges in $E$. (If A calls B, B impacts A).
2. **Breadth-First Search (BFS):** Start BFS from all $s \in S$ in $G^R$.
3. **Impact Scoring:**
   - Level 0 (Direct Sources): $Impact(v) = 1.0$.
   - Level 1 (Direct Callers): $Impact(v) = 0.9$.
   - Level $k$: $Impact(v) = 0.9^k$.
4. **Aggregation:** For any node $v$ reached by multiple paths, $Impact(v) = \min(1.0, \sum P_i)$.
5. **Thresholding:** Nodes with $Impact(v) > \epsilon$ are included in the blast radius report.

This algorithm allows the CLI to output a prioritized list: "This change will explicitly break module X (Impact 1.0), and likely destabilize module Y (Impact 0.72)."

------

## 5. Tier 3: Semantic Analysis & Vector Databases (The AI Layer)

Structural analysis fails when connections are not explicit code references. For example, if `ADR-007` mandates usage of AWS, and a developer implements a generic `StorageService` trait backed by S3, structural analysis sees the link. But if a developer writes a `RetryPolicy` specifically tuned for S3's eventual consistency model without importing S3 types, the structural link is invisible. Furthermore, answering "Why uses Redis?" requires understanding the *intent* and *semantics* of the code and documentation.

This tier introduces local Vector Databases and Large Language Models (LLMs) to bridge the semantic gap using Retrieval-Augmented Generation (RAG).

### 5.1 Local Embedding Models with Candle

To populate the vector DB, we need to generate embeddings for code and ADR text. Calling OpenAI APIs violates the "local-first" constraint and raises privacy concerns. `candle` is a minimalist ML framework for Rust developed by HuggingFace. It allows running models like `BERT`, `Jina-Embeddings`, or `StarCoder` directly in the Rust binary, utilizing CPU or Metal/CUDA if available, without a Python runtime.

#### 5.1.1 Model Selection for Code

Generic text models (like standard BERT) perform poorly on code because code contains high-entropy identifiers and rigorous structure. We require models trained on code-text pairs.

- **`all-MiniLM-L6-v2`:** A lightweight baseline (approx. 80MB). Good for natural language queries but mediocre for code structure.
- **`jina-embeddings-v2-base-code`:** A state-of-the-art model specifically tuned for code. It supports a context length of 8192 tokens, which is crucial for embedding entire files or large functions.
- **`bge-m3`:** Excellent for multilingual retrieval and dense retrieval tasks.

The CLI should download and cache the quantized (GGUF or ggml) version of these models to minimize disk footprint and startup time. Using `candle-transformers` and `tokenizers`, the Rust application can perform inference locally.

**Example Rust Implementation with Candle:**

Rust

```
use candle_core::{Device, Tensor};
use candle_transformers::models::bert::{BertModel, Config};

fn embed_code(text: &str, model: &BertModel, tokenizer: &Tokenizer) -> Tensor {
    let tokens = tokenizer.encode(text, true).unwrap();
    let token_ids = Tensor::new(tokens.get_ids(), &Device::Cpu).unwrap().unsqueeze(0).unwrap();
    let embeddings = model.forward(&token_ids, &tokens.get_type_ids()).unwrap();
    // Perform pooling (e.g., mean pooling) to get a single vector
    embeddings.mean(1).unwrap()
}
```

### 5.2 Embedded Vector Databases: LanceDB vs. Qdrant

The generated vectors must be stored and indexed for similarity search. For a local CLI, the database must be embedded (library-linked) rather than a separate server process.

#### 5.2.1 Qdrant (Local Mode)

`Qdrant` is written in Rust and offers a highly performant vector search engine. It supports a "local mode" where it runs within the application process.

- **Pros:** Mature, supports HNSW (Hierarchical Navigable Small World) graphs for fast search, rich filtering.
- **Cons:** Primarily designed as a server; the local library mode can still be heavy.

#### 5.2.2 LanceDB (Recommended)

`LanceDB` is a newer, serverless vector database that is natively embedded. It stores data in the Lance columnar format (based on Apache Arrow), which is optimized for ML workloads.

- **Pros:**
  - **Zero-Copy:** Can read data directly from disk without serialization overhead.
  - **Disk-Based:** Does not require loading the entire index into RAM, making it scalable for large codebases on limited hardware.
  - **Hybrid Search:** Supports full-text search (FTS) alongside vector search. This is critical for our use case (finding "AWS" keyword + semantic matches).
- **Implementation:** The CLI creates a hidden directory `.adr-blast/lancedb` to store the index.

### 5.3 RAG Pipeline for Architectural Analysis

Retrieval-Augmented Generation (RAG) is typically used for chatbots. Here, we use it for *Analysis*. We want to retrieve code snippets that are semantically relevant to the ADR text.

#### 5.3.1 Semantic Chunking Strategy

Code cannot be chunked arbitrarily (e.g., every 500 characters) because this breaks syntax. We must use the **Tree-sitter** data from Tier 1 to perform *Semantic Chunking*.

1. **Extract Entities:** The tool walks the file and extracts full `function` and `struct` bodies.
2. **Enrichment:** Each chunk is prepended with metadata: "File: `src/s3.rs`, Module: `storage`, Struct: `S3Client`". This provides context to the embedding model.
3. **Indexing:** The chunk is embedded and stored in LanceDB with its metadata.

#### 5.3.2 Indexing ADRs

ADRs are markdown files. They should be chunked by section (Context, Decision, Consequences). Each section is embedded. We also extract the "Title" and "Tags" for hybrid search filtering.

### 5.4 Resolving Ambiguity: The "Why?"

The user asks: *"Why are we using Redis?"*

This query is ambiguous. It could refer to session storage, caching, or message brokering.

**Algorithm 2: Hybrid Rationale Discovery**

1. **Query Embedding:** Embed the user string $Q = \text{"Why are we using Redis?"}$.
2. **ADR Retrieval:** Search the *ADR Table* in LanceDB for vectors close to $Q$.
   - *Result:* `ADR-004: Caching Strategy` (Distance 0.15). Content: "We chose Redis over Memcached for persistence."
3. **Code Retrieval:** Search the *Code Table* for vectors close to $Q$.
   - *Result:* `src/cache/redis.rs` (Distance 0.2).
4. **Synthesized Answer:** The tool correlates the two. "Redis is used in `src/cache/redis.rs`. This usage is governed by `ADR-004`, which states it was selected for persistence."

This effectively creates a semantic bridge between the documentation and the implementation.

------

## 6. Visualization and User Experience (UX)

Analyzing a blast radius might return 500 affected files. Dumping this to `stdout` is useless. We need an interactive Terminal User Interface (TUI).

### 6.1 TUI Construction with Ratatui

`ratatui` is the successor to `tui-rs` and the standard for Rust TUIs. It allows creating split-pane interfaces (like `htop`).

- **Widgets:** We use the `List` and `Table` widgets  to display results.
- **Tree View:** To show the dependency hierarchy (ADR -> Direct -> Transitive), we utilize `tui-realm-treeview`. This allows the user to expand/collapse branches of the blast radius.

### 6.2 Visualizing the Blast Radius

The UI should present a "Risk Heatmap":

- **Red Nodes:** Explicitly tagged or structurally dependent files (Tier 1 & 2).
- **Yellow Nodes:** Semantic matches (Tier 3). "Suspected dependency."
- **Green Nodes:** Files analyzed but deemed safe.

### 6.3 Performance Considerations for UX

- **Async Rendering:** The semantic search can take seconds. The TUI must remain responsive. We use `tokio` for async tasks (searching LanceDB) and a dedicated rendering thread for the TUI.
- **Progress Indicators:** Use `ratatui`'s `Gauge` widget to show indexing progress.

------

## 7. Implementation: The `adr-blast` Tool Architecture

We synthesize the three tiers into a single cohesive tool: `adr-blast`.

### 7.1 System Architecture

The tool is a single binary with modular "engines."

```
+---------------------------------------------------------------+

| CLI (Clap + Ratatui) |
+-------------------------------+-------------------------------+

| Analysis Engine | Query Engine |
+-------------------------------+-------------------------------+

| Tier 1: Lexical (Ripgrep) | Tier 3: Semantic (Candle) |
| - explicitly tagged files | - embedding generation |
+-------------------------------+-------------------------------+

| Tier 2: Structural (SCIP) | Vector DB (LanceDB) |
| - transitive dependencies | - storage & similarity search|
+-------------------------------+-------------------------------+

| File System / Cache |
+---------------------------------------------------------------+
```

### 7.2 Data Flow: The Deprecation Workflow

**Scenario:** User runs `adr-blast deprecate "ADR-007"` (Target: AWS).

1. **Parsing:** The tool reads `docs/adrs/007-aws.md`. Extracts keywords ("AWS", "S3", "Secrets Manager").
2. **Tier 1 Scan:**
   - `grep-searcher` scans for `// ADR-007`. Finds `src/secrets.rs`.
   - `grep-searcher` scans for `aws_sdk`. Finds `src/storage.rs` (untagged).
3. **Tier 2 Expansion:**
   - Tool loads `.adr-blast/index.scip`.
   - Finds that `src/secrets.rs` (struct `AwsSecrets`) is used by `src/auth.rs`.
   - Finds that `src/storage.rs` (struct `S3Storage`) is used by `src/video_processing.rs`.
   - *Blast Radius grows from 2 files to 4.*
4. **Tier 3 Semantic Check:**
   - Tool embeds ADR-007 text.
   - Queries LanceDB for code semantically similar to "AWS Infrastructure".
   - Finds `src/infra/terraform_gen.rs`. This file generates IaC but has no imports of the AWS SDK (it just writes strings). Structural analysis missed it. Semantic analysis catches it.
   - *Blast Radius grows to 5 files.*
5. **Reporting:**
   - The TUI displays the 5 files, grouped by confidence.
   - User interacts with the list to see *why* each file was included (e.g., "Reason: Calls `AwsSecrets`", or "Reason: Semantic similarity 0.89 to ADR text").

### 7.3 Managing Local Resources

To ensure the tool works on a laptop:

- **Lazy Loading:** Do not load the full SCIP graph or Vector Index into RAM. LanceDB handles disk-based vectors. The SCIP graph can be streamed or partially loaded.
- **Quantization:** Use 4-bit quantized GGUF models for `candle`. This reduces the embedding model size from ~500MB to ~100MB, with negligible accuracy loss for this use case.
- **Caching:** Store embeddings in `.adr-blast/`. Only re-compute embeddings if the file hash (SHA-256) changes. This makes subsequent runs near-instant.

------

## 8. Comparison of Algorithmic Approaches

| **Feature**           | **Regex/Grep (Baseline)** | **Rust-Analyzer (LSP)**  | **Stack-Graphs**     | **Vector Search (RAG)** | **Recommended Hybrid** |
| --------------------- | ------------------------- | ------------------------ | -------------------- | ----------------------- | ---------------------- |
| **Setup Cost**        | None                      | High (Heavy compilation) | Medium (Graph build) | High (Embedding)        | **Medium**             |
| **Speed**             | Instant                   | Slow (seconds/minutes)   | Fast (incremental)   | Medium (inference)      | **Fast (Cached)**      |
| **Accuracy**          | Low (False positives)     | 100% (for code)          | 100% (name binding)  | Probabilistic           | **High**               |
| **Transitive Deps**   | No                        | Yes                      | Yes                  | No                      | **Yes**                |
| **Implicit/Semantic** | No                        | No                       | No                   | Yes                     | **Yes**                |
| **Disk Usage**        | None                      | High (target dir)        | Low                  | Medium (Indices)        | **Medium**             |

**Conclusion:** The hybrid approach is the only one that satisfies all requirements: finding explicit tags (Tier 1), tracing the blast radius through calls (Tier 2), and answering ambiguous "Why?" questions (Tier 3).

------

## 9. Future Directions and Advanced Capabilities

### 9.1 Agentic Remediation

Once the blast radius is identified, the next logical step is remediation. Integration with "Agentic" workflows (using local LLMs via `candle`) could allow the tool to not just *report* the impact but *draft* the refactoring. For example, "I see you are removing AWS. `src/secrets.rs` breaks. I can scaffold an `AzureKeyVault` struct that implements the same `SecretsTrait`.".

### 9.2 Continuous Architectural Integrity

This tool need not be CLI-only. The analysis logic can be wrapped in a `git hook` or a CI/CD step. If a PR modifies an ADR but leaves the implementation untouched (or vice versa), the tool can flag a "Drift Warning."

### 9.3 Conclusion

The "blast radius" of architectural change is a multi-dimensional problem requiring a multi-dimensional solution. By treating code as text, structure, and meaning simultaneously, we can build tools that illuminate the hidden dependencies in our software. The Rust ecosystem, with its high-performance crates like `tree-sitter`, `rust-analyzer`, `candle`, and `lancedb`, provides the perfect substrate for building this next generation of architectural analysis tools locally, securely, and efficiently.