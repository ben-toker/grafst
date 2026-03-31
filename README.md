## Architecture

Same pipeline as a small interpreter:

```
"a -- b + c -- d"  →  lexer  →  tokens  →  parser  →  AST  →  eval  →  Graph  →  layout  →  TikZ string
```

Each stage is an independent transformation. Data flows forward.

### Project Structure

```
src/
  main.rs
  graph.rs
  style.rs
  tikz.rs
  layout.rs
  parser/
    mod.rs
    lexer.rs
    ast.rs
    grammar.rs
```

### File Descriptions

**`style.rs`** — Data-only. Defines `NodeStyle` and `EdgeStyle` structs — optional visual properties like fill color and line width. Separate from `graph.rs` because both `graph.rs` and `tikz.rs` need these types.

**`graph.rs`** — The core data structure. A `Graph` holds `Node`s and `Edge`s. Nodes have a label, style, and position (filled in by layout). Methods: `add_node()`, `add_edge()`, `union()`. Almost every other file depends on this one.

**`parser/lexer.rs`** — Tokenizer. Takes the raw input string and chops it into `Token`s (`Ident`, `DoubleDash`, `Plus`, `Eof`). Handles whitespace. Doesn't care if the tokens make sense together — that's the parser's job.

**`parser/ast.rs`** — abstract syntax tree. Defines the `Expr` enum — the types a parsed expression can take: `EdgeList`, `NamedGraph`, `Compose`. No logic, just type definitions. Vertices can carry optional position tags (`[top]`, `[bottom]`, `[left]`, `[right]`) used as attachment points during composition.

**`parser/grammar.rs`** — The parsing logic. Recursive-descent functions that consume tokens and build `Expr` AST nodes. Enforces grammar rules, rejects malformed input.

**`parser/mod.rs`** — Public API for the parser module. Re-exports `Expr` and exposes `parse()` that delegates to `grammar.rs`.

**`layout.rs`** — Fruchterman-Reingold force-directed layout. Places nodes at deterministic initial positions (Fermat spiral, no RNG), then iterates: nodes repel each other (Coulomb-like), edges pull connected nodes together (spring-like), temperature cools each step. Writes `(x, y)` positions into each node.

**`tikz.rs`** — Output stage. Takes a positioned `Graph` and produces a TikZ string. Emits `\node` commands with coordinates, `\draw` commands for edges. Formats labels as LaTeX math subscripts (`v1` → `$v_1$`). Buffers to a `String` so there's no partial output on error.

**`main.rs`** — The orchestrator. Reads the CLI arg, calls `parser::parse()` to get an AST, evaluates the AST into a `Graph` (pattern-matching on `Expr` variants), runs force-directed layout, calls `tikz::emit()`, prints to stdout.

### Module Dependencies

```
main.rs
  → parser  (input string → Expr AST)
  → graph   (Expr → Graph)
  → layout  (positions nodes in the Graph)
  → tikz    (Graph → TikZ string)

style is used by: graph, tikz
graph is used by: main, layout, tikz
parser knows nothing about graph/layout/tikz
tikz knows nothing about parser
```

### Grammar

```
expr        ::= term ( join_op term )*
term        ::= IDENT tag? ( '--' IDENT tag? )+   // edge list
              | NAMED_GRAPH                        // e.g. K5, C4, K_{3,3}
tag         ::= '[' POSITION ']'                   // top, bottom, left, right
join_op     ::= '+>'                          // horizontal join (right's [left] merges with left's [right])
              | '+v'                          // vertical join (top's [bottom] merges with bottom's [top])
              | '+'                           // disjoint union (no merging)
```

- `a -- b -- c` = three nodes, edges a-b and b-c
- `a -- b + c -- d` = disjoint union of two edge lists
- `C4 +> C4` = two C4s joined horizontally (right vertex of left graph merges with left vertex of right graph)
- `a[top] -- b -- c[bottom] +v d[top] -- e` = vertical join on tagged vertices

### Composition Model

**Two-layer architecture:** the AST represents what the user typed; evaluation resolves named graphs, performs merges, and produces a concrete `Graph` with auto-generated vertex IDs.

**Position tags** — any vertex can be tagged `[top]`, `[bottom]`, `[left]`, `[right]`. Named graphs (like `C4`, `K5`) come with default tags. Raw edge lists get tags from the user via `a[top] -- b -- c[bottom]` syntax.

**Directional joins** — `+>` connects left's `[right]` vertices to right's `[left]` vertices. `+v` connects top's `[bottom]` to bottom's `[top]`. Paired vertices merge into a single vertex.

**Alignment rules** for when the two sides have different numbers of tagged vertices:
- Same count: one-to-one pairing.
- Different count, same parity: center-align. E.g. 5 `[bottom]` + 3 `[top]` — align on the middle vertex (3rd and 2nd respectively), outer unpaired vertices stay as-is.
- Different count, different parity: left-align. Pair from the left, extras are unpaired.

**Tags are consumed by composition.** After a join, all tags (paired and unpaired) are removed. The result is a plain graph with auto-generated vertex IDs. The user can re-tag vertices and compose further. Tags are recalculated fresh at each composition step.

---

## Milestones

### M1: Edge Lists

- **`style.rs`** — `NodeStyle` and `EdgeStyle` structs with optional fields. Derive `Clone`, `Debug`, `Default`.
- **`graph.rs`** — `Graph`, `Node`, `Edge`. Methods: `new()`, `add_node()`, `add_edge()`. Auto-generated vertex IDs.
- **`parser/ast.rs`** — `Expr::EdgeList(Vec<String>)`.
- **`parser/lexer.rs`** — `Token` enum: `Ident(String)`, `DoubleDash`, `Eof`. Tokenize function that skips whitespace.
- **`parser/grammar.rs`** — Parse `ident -- ident -- ...` into `Expr::EdgeList`.
- **`parser/mod.rs`** — Re-export `Expr`, expose `parse()`.
- **`layout.rs`** — Fruchterman-Reingold force-directed layout.
- **`tikz.rs`** — Emit `\begin{tikzpicture}...\end{tikzpicture}` with nodes and edges.
- **`main.rs`** — Wire it: arg → parse → build graph → layout → emit → print.

**Test:** `graphtex 'a -- b -- c'` → compilable TikZ.

---

### M2: Disjoint Union

- **`graph.rs`** — Add `union()` method. Combines two graphs with no vertex merging.
- **`parser/ast.rs`** — Add `Expr::Compose { left, right, join: JoinOp }` and `JoinOp::Disjoint`.
- **`parser/lexer.rs`** — Add `Plus` token.
- **`parser/grammar.rs`** — Parse `expr + expr` into `Expr::Compose` with `JoinOp::Disjoint`.
- **`main.rs`** — Extend eval to handle `Compose` by building both graphs and calling `union()`.

**Test:** `graphtex 'a -- b -- c + d -- e -- f'` → two disconnected components in one TikZ picture.

---

### M3: Named Graphs

- **`parser/ast.rs`** — Add `Expr::NamedGraph(family, n)` for `K`, `C`, `W`, `P` families.
- **`parser/lexer.rs`** — Recognize named graph tokens (e.g. `K5`, `C4`, `K_{3,3}`).
- **`parser/grammar.rs`** — Parse named graph tokens into `Expr::NamedGraph`.
- **`eval.rs`** (new) — Expand `NamedGraph` into a concrete `Graph` with auto-generated vertex IDs and default position tags.

**Test:** `graphtex 'K5'` → complete graph on 5 vertices. `graphtex 'K5 + C4'` → disjoint K5 and C4.

---

### M4: Position Tags

- **`parser/ast.rs`** — Extend vertex representation to carry an optional position tag (`Top`, `Bottom`, `Left`, `Right`).
- **`parser/lexer.rs`** — Add `LBracket`, `RBracket`, and `Position` tokens.
- **`parser/grammar.rs`** — Parse `ident[top]` syntax, attach tags to vertices in the AST.

**Test:** `graphtex 'a[top] -- b -- c[bottom]'` → parses successfully, tags present in AST.

---

### M5: Directional Composition

- **`parser/ast.rs`** — Add `JoinOp::Horizontal` (`+>`) and `JoinOp::Vertical` (`+v`).
- **`parser/lexer.rs`** — Add `PlusRight` and `PlusDown` tokens.
- **`parser/grammar.rs`** — Parse `+>` and `+v` as join operators.
- **`eval.rs`** — Implement directional merge: collect tagged vertices from each side, apply alignment rules (same count → 1:1, same parity → center-align, mismatched parity → left-align), merge paired vertices, strip all tags from result.

**Test:** `graphtex 'C4 +> C4'` → two diamonds joined horizontally. `graphtex 'C4 +v C4'` → two diamonds joined vertically.

---

### Future

- **Layout options** — circle, bipartite layouts alongside force-directed
- **Highlighting** — visually distinguish a subgraph within a larger graph
- **LaTeX integration** — `graphtex.sty` package with `\graphtex{}` macro
