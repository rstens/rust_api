# Rust API Template

A starter Rust project for building a production-ready API with:

- 🚀 Axum (web framework)
- 🐘 Postgres with SQLx
- 📦 Docker Compose for local development
- ⚙️ GitHub Actions for CI/CD
- 🧪 Unit + integration tests
- 📋 Makefile for common tasks
- 🐳 Devcontainer (VS Code / GitHub Codespaces ready)
- 🔍 REST Client file (`.http`) for quick endpoint testing

## 🚀 Quickstart

```bash
make db-up        # start Postgres
make migrate      # run migrations
make seed-rust    # insert demo data
make run          # run the API
```

API runs at <http://localhost:3000>
