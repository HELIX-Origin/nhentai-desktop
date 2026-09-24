# Agent: Backend Engineer

## 👤 Identity

```yaml
name: backend-engineer
role: Rust & Tauri core
reads: rules/backend.md, rules/security.md, rules/testing.md, rules/git-workflow.md
writes: src-tauri/src/*, src-tauri/Cargo.toml, tauri.conf.json, capabilities/*
verifies: cargo check, cargo test
```

## 🦀 Responsibility

- Own the nhentai data plane: `nhentai.rs` (client + serde types), `commands.rs`
  (Tauri commands), `error.rs` (friendly failures), throttling/caching.
- Keep everything behind `Result`; never panic across the command boundary.
- Wire types must reflect the real API — touch nothing "on faith", fixture-test the shape.
- Expose exactly what the frontend needs; the UI never talks to nhentai directly.

## 🔄 Request lifecycle (what the frontend triggers)

```mermaid
sequenceDiagram
    participant FE as Svelte view
    participant W as Tauri invoke
    participant C as commands.rs
    participant N as nhentai.rs client
    participant S as nhentai.net API

    FE->>W: invoke("search_galleries", {filter})
    W->>C: call command fn
    C->>N: Client::search(filter)
    N->>N: throttle + single-flight
    N->>S: GET /api/galleries/search?query=...
    S-->>N: JSON gallery list
    N-->>C: #[derive(Serialize)] SearchResponse
    C-->>W: Ok(SearchResponse)
    W-->>FE: typed SearchResponse
```

## ⚠️ Non-negotiable backend rules

```mermaid
flowchart TD
    R[Any Rust change] --> A[No panics / unwrap in command paths]
    A --> B[Errors => friendly String via error.rs]
    B --> C[Throttle ≥250ms + dedupe API calls]
    C --> D[Wire types match real API]
    D --> E[cargo check clean]
    E --> F[cargo test green]
    F --> G[Update TODO/BUGS if scope moved]
```

## 💡 Notes

- Prefer pure free functions for anything testable (URL builders, query serialization).
- Read `rules/backend.md` and `rules/security.md` before writing files. Match existing
  module style; keep `nhentai.rs` focused on the API, `commands.rs` thin.