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

**`parser/ast.rs`** — abstract syntax tree. Defines the `Expr` enum — the types a parsed expression can take: `EdgeList` and `Compose`. No logic, just type definitions.

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
expr        ::= term ( '+' term )*     // union
term        ::= IDENT ( '--' IDENT )+  // edge list
```

- `a -- b -- c` = three nodes, edges a-b and b-c
- `a -- b + c -- d` = union of two edge lists (disjoint unless labels overlap)

---

## Milestones

### M1: Edge Lists

- **`style.rs`** — `NodeStyle` and `EdgeStyle` structs with optional fields. Derive `Clone`, `Debug`, `Default`.
- **`graph.rs`** — `Graph`, `Node`, `Edge`. Methods: `new()`, `add_node()`, `add_edge()`.
- **`parser/ast.rs`** — `Expr::EdgeList(Vec<String>)`.
- **`parser/lexer.rs`** — `Token` enum: `Ident(String)`, `DoubleDash`, `Eof`. Tokenize function that skips whitespace.
- **`parser/grammar.rs`** — Parse `ident -- ident -- ...` into `Expr::EdgeList`.
- **`parser/mod.rs`** — Re-export `Expr`, expose `parse()`.
- **`layout.rs`** — Fruchterman-Reingold force-directed layout.
- **`tikz.rs`** — Emit `\begin{tikzpicture}...\end{tikzpicture}` with nodes and edges.
- **`main.rs`** — Wire it: arg → parse → build graph → layout → emit → print.

**Test:** `graphtex 'a -- b -- c'` → compilable TikZ.

---

### M2: Composition (Union)

- **`graph.rs`** — Add `union()` method. Merges two graphs; nodes matched by label, edge sets combined.
- **`parser/ast.rs`** — Add `Expr::Compose(Box<Expr>, Box<Expr>)`.
- **`parser/lexer.rs`** — Add `Plus` token.
- **`parser/grammar.rs`** — Parse `expr + expr`.
- **`main.rs`** — Extend eval to handle `Compose` by building both graphs and calling `union()`.

**Test:** `graphtex 'a -- b -- c + d -- e -- f'` → two disjoint triangles-ish in one TikZ picture.

---

### Future

- **Bridge edges** — modifier syntax to connect specific vertices across subgraphs after seeing the output
- **Named graphs** — `K5`, `C5`, `W5`, `K_{3,3}`, `P5` shorthand
- **Layout options** — circle, bipartite layouts alongside force-directed
- **Highlighting** — visually distinguish a subgraph within a larger graph
- **LaTeX integration** — `graphtex.sty` package with `\graphtex{}` macro
