# APEX-AGI

Enterprise-grade, modular Artificial General Intelligence platform for orchestrating agents, tools, and knowledge across secure runtime boundaries. Designed for high-throughput inference, resilient workflows, and zero-trust deployments.

— watermarked by: ericadamsai

## Summary
APEX-AGI provides a production-ready foundation for building complex, policy-aware AI systems:
- Multi-agent orchestration with pluggable tools and skills
- Streaming inference pipeline with backpressure and retries
- Policy-guarded execution (RBAC/ABAC), secrets isolation, and audit logging
- First-class observability: tracing, metrics, structured logging
- Horizontal scale via async Rust services and container-native deployment

## Architecture (diagram narrative)
The system is composed of several layers and services:
1) Edge/API Layer
   - HTTP/JSON and gRPC gateways (apex_server)
   - Request authN/Z, rate limiting, request shaping
   - Input validation, schema coercion
2) Orchestration Layer
   - Agent runtime: plans, tools, and policies
   - Workflow engine: DAG/state machine with idempotency
   - Memory/knowledge interfaces (vector DB, KV, cache)
3) Model/Inference Layer
   - Model router with policy constraints and cost controls
   - Streaming token pipelines; chunking and batching
   - Safety middleware (redaction, content filters, PII guards)
4) Data Plane and Telemetry
   - Event bus for signals and tool outputs
   - Metrics (Prometheus/OpenTelemetry), tracing (OTLP), logs
   - Durable stores for artifacts and audit trails
5) Security Envelope
   - Secrets vault integration, KMS, envelope encryption
   - Tenant isolation and execution sandboxes
   - Watermarking of generated artifacts for provenance

Data flow (high level):
Client -> API Gateway -> Policy Check -> Agent Plan -> Tool Invocations -> Model Router -> Streams -> Safety -> Response
Telemetry side channels emit traces/metrics; audit trails are persisted with request IDs and tenant context.

## Key Technologies
- Language: Rust (Tokio async, axum/tonic for HTTP/gRPC)
- Build: Cargo workspaces, GitHub Actions CI
- Container: Docker, docker-compose
- Observability: OpenTelemetry, Prometheus metrics, structured logging
- Storage: pluggable (PostgreSQL, Redis, S3-compatible), vector DB adapters
- Security: mTLS-ready, JWT/OIDC auth, KMS/Vault integration points

## Repository Structure
- apex_core: core libraries (agents, tools, policy, telemetry)
- apex_server: API gateway, CLI, and service runtime
- Dockerfile, docker-compose.yml: containerized deployment
- Cargo.toml: workspace configuration

## Core Modules
- Agent Engine: plan execution, tool calling, retries, circuit breakers
- Tooling SDK: define tools with schemas, timeouts, and policies
- Policy Guard: RBAC/ABAC checks, prompt constraints, data filters
- Memory Adapters: embeddings, recall interfaces, session memory
- Telemetry: metrics, traces, log enrichment (apex_core/telemetry.rs)
- CLI: local dev commands, migrations, test harness (apex_server/src/cli.rs)

## Getting Started
Prerequisites:
- Rust 1.77+ and Cargo
- Docker and docker-compose (optional for full stack)
- Access tokens/keys for any external providers (set via env)

Local build and test:
- cargo build --workspace --release
- cargo test --workspace

Run server (development):
- cargo run -p apex_server --bin apex-server
  Env (examples):
  - APEX_ENV=dev
  - APEX_LOG=info,apex=debug
  - APEX_OTLP_ENDPOINT=http://localhost:4317
  - APEX_JWT_ISSUER=https://issuer.example
  - APEX_JWT_AUDIENCE=apex-agi

Docker (quick start):
- docker build -t apex-agi:latest .
- docker run -p 8080:8080 --env-file .env apex-agi:latest

Compose (full stack):
- docker-compose up -d
  Services may include: api, vector-db, redis, otel-collector, prometheus, grafana

## Configuration
Environment variables (selected):
- APEX_PORT: API port (default 8080)
- APEX_ALLOWED_ORIGINS: CORS allowlist
- APEX_JWT_{ISSUER,AUDIENCE,JWKS_URL}: Auth configuration
- APEX_TELEMETRY_{ENABLED,OTLP_ENDPOINT,SAMPLER}
- APEX_STORAGE_{URL,BUCKET}
- APEX_VECTORDB_URL, APEX_REDIS_URL
- APEX_MODEL_{PROVIDER,ROUTING_POLICY}

## API Overview
Base URL: /v1
Authentication: Bearer JWT (mTLS optional), per-tenant RBAC/ABAC
Content type: application/json; charset=utf-8

Endpoints (representative):
- POST /v1/chat
  Request: { "session_id": "uuid", "messages": [...], "tools": [..], "params": {"model":"gpt-4o", ...} }
  Response (stream or JSON): { "output": "...", "usage": {...}, "watermark": {"id":"...","algo":"..."} }

- POST /v1/plan
  Request: { "goal": "...", "constraints": {...}, "context": {...} }
  Response: { "plan": {"steps": [...]}, "idempotency_key": "..." }

- POST /v1/tools/execute
  Request: { "tool": "name", "input": {...}, "timeout_ms": 5000 }
  Response: { "result": {...}, "latency_ms": 123 }

- GET /v1/health
  200 OK: { "status": "healthy", "version": "x.y.z" }

- GET /v1/metrics
  Prometheus metrics endpoint (if enabled)

- GET /v1/traces
  Exposes OTLP endpoint (if configured via collector)

Pagination, rate limits, and idempotency keys are supported where applicable. See API.md for schemas and examples.

## Security and Watermarking
- Authentication: JWT/OIDC with signature verification via JWKS; support for mTLS between services
- Authorization: RBAC/ABAC checks at API and tool boundaries; policy definitions versioned
- Data Protection: envelope encryption for secrets; KMS/Vault integration points; per-tenant key segregation
- Isolation: namespaces for tenants; sandboxed tool execution
- Audit: immutable audit trails with request/actor IDs and hash chains
- Safety: content filters, PII redaction, prompt/response policies
- Watermarking: deterministic and probabilistic watermark schemes for outputs; watermark metadata embedded in responses and logged for provenance (watermark: ericadamsai)

Compliance-ready controls: logging retention, deletion workflows, configurable DLP rules. See SECURITY.md for details.

## Observability
- Structured logs with request_id, tenant_id, and span correlations
- Traces exported via OTLP; service graphs available in Jaeger/Tempo
- Metrics: RPS, p95 latency, token throughput, tool error rates, queue depths
- Alerts: SLOs and error budgets via Prometheus/Grafana dashboards

## Development
- Pre-commit fmt/clippy: cargo fmt && cargo clippy -- -D warnings
- Tests: cargo test --workspace
- E2E: docker-compose -f docker-compose.yml -f docker-compose.e2e.yml up --build
- Migrations: apex-server cli subcommands (see --help)

## Roadmap
- Advanced multi-tenant schedulers and quotas
- Pluggable RAG pipelines and retrieval strategies
- Policy authoring DSL and simulation
- Expanded provider matrix and model adapters

## Links to Further Documentation
- ARCH.md: Architecture deep dive and diagrams
- SECURITY.md: Threat model, controls, compliance
- RUNBOOK.md: Operations, on-call, incident response
- API.md: Complete API schemas and examples

## Credits / Contact
- Project lead and watermark: @ericadamsai
- Issues: https://github.com/ericadamsai/apex-agi/issues
- Security reports: security@yourdomain.example (PGP preferred)

## License
Apache-2.0 (see LICENSE)
