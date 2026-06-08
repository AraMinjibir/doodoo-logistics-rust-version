# DooDoo Logistics (Rust Version)

**Rust • Actix Web • Tokio • PostgreSQL • REST • WebSockets**

A production-oriented logistics and delivery backend, rewritten from a Scala/Play Framework system into Rust, demonstrating modern backend engineering with async architecture, strict domain modeling, and event-driven design.

Built as a modular monolith, it prioritizes correctness, maintainability, and performance using Rust’s type system and async runtime.

---

# 1. Problem Statement
Logistics systems require strict correctness and full traceability.

This system solves:

* Invalid shipment state transitions
* Lack of audit history in delivery systems
* Poor role separation in logistics workflows
* Blocking synchronous workflows in tracking/payment systems

### Deterministic Lifecycle Management

Prevents illegal state transitions (e.g., Created → Delivered without InTransit).

### Role-Based Access Control (RBAC)

Enforces Principle of Least Privilege:

* Customers
* Recipient
* Service Providers
* Support Agents
* Administrators

### Auditability

Maintains a permanent timestamped history of shipment status changes.

### Event-Driven Notifications

Ensures stakeholders receive updates asynchronously without blocking core API operations.

---

# 2. Core Capabilities by Role

The system enforces the **Principle of Least Privilege (PoLP)** across four distinct roles.

## Customer / Sender

* Create shipments with full validation
* Receive unique tracking numbers
* Track lifecycle via strict state machine
* Full shipment history & audit trail
* WebSocket-based live tracking updates
* Lifecycle:
 Created → InTransit → Assigned → OutForDelivery → Delivered

Invalid transitions are rejected at runtime via domain rules.

## Recipient (Consignee)

* Track incoming shipments
* Confirm delivery
* Provide Proof of Delivery (PoD)
* Receive shipment notifications

## Service Provider (Courier)

* Accept shipments
* Move shipments to In Transit
* Mark shipments as Delivered
* Provide delivery metadata
* Submit delivery notes

## Support Agent

### Shipment Lookup & Inquiry Resolution

* Search shipments by tracking number
* Filter shipments by status
* Investigate customer issues

### Complaint Management

* Create complaint linked to shipment
* Track complaint status
* Resolve operational issues

### Shipment Attention Monitoring

* Detect stalled shipments
* Flag shipments requiring manual review

---

# 3. Payment Management

* Shipment-linked payment processing
* Status tracking: Pending | Successful | Failed | Refunded
* Prevents dispatch before payment confirmation
* Revenue aggregation (daily, weekly, monthly)


### Event-Driven Notifications

* Payment success notification
* Payment failure alert
* Support agent alerts

### Supports:

* Async processing (Tokio runtime)
* Extensible integrations (Email / SMS / Push)
* Future message broker migration

### Shipment Flow Integration

* Prevent dispatch of unpaid shipments
* Enforce payment-before-dispatch rule

### Revenue Reporting

* Daily aggregation
* Weekly aggregation
* Monthly aggregation
* Half-open interval filtering [start, end)
* Database-level aggregation
* Monetary precision using Decimal types

---

# 4. Administrative Functionality

## User Account Management

Administrators can:

* Create user accounts
* Edit user profiles
* Assign roles
* Deactivate accounts
* Remove accounts

Ensures secure access control and operational integrity.

## Shipment Monitoring

Administrators can:

* Track shipment status
* View shipment details
* Monitor logistics flow
* Maintain operational oversight

---

# 5. Notification System (Event-Driven, Asynchronous)

Implemented using in-memory domain events:

* Decoupled business logic from notification handling

* Centralized EventBus abstraction

* Triggered on domain events:

  * Shipment created
  * Status updated
  * Payment processed
  * User updated

* Async processing powered by Tokio runtime (via Actix)

* Extensible to support:

  * Email
  * Push notifications
  * SMS

* Designed for future message broker integration

* Dedicated NotificationService layer

---

# 6. Architecture & Design

The system follows **Domain-Driven Design (DDD)** principles.

### Persistence Ignorance

Services depend on repository traits allowing database abstraction.

### Type-Safe Domain

Rust enums enforce valid states and roles at compile time.

### Modular Monolith Structure

```
API Clients (Web / Mobile / Admin)
                |
             REST / WS
                |
       Actix Web HTTP Layer
                |
         Application Services
                |
         Domain Logic (Pure Rust)
                |
          Repository Traits
                |
         PostgreSQL Database
```

---

# 7. Technical Stack

Backend:

* Rust
* Actix Web (HTTP API framework)

Persistence:

* PostgreSQL
* SQLx (compile-time checked queries)

Security:

* BCrypt password hashing
* JWT authentication

Async Processing:

* Tokio runtime (used internally by Actix)
* In-memory event bus

Real-time:

* WebSockets (tracking updates)

---

# 8. Testing Strategy

DooDoo Logistics prioritizes correctness using a testing pyramid.

### Unit Tests

* Business rule validation
* State transition enforcement
* RBAC logic

### Integration Tests

* Repository mapping
* SQL query validation
* Transaction handling

### End-to-End Tests

* Shipment lifecycle flow
* Payment integration
* Notification triggers

---

# 9. Roadmap & Operational Readiness

[x] Core Shipment Engine
[x] RBAC Implementation
[x] Support Module
[x] Payment Management
[ ] Observability (structured logging)
[ ] Health checks
[ ] CI/CD pipeline
[ ] Docker deployment

---

# 10. Running the Project Locally

```bash
# Clone repository
git clone git@gitlab.com:AraMjb/doodoo-logistics-rust.git

# Start PostgreSQL
docker-compose up -d

# Run application
cargo run
```

---

# 11. Design Goals

* Strong domain modeling
* Strict lifecycle enforcement
* Async non-blocking architecture
* High data consistency
* Modular monolith simplicity
* Production-ready structure
* Extensible event-driven design

---

# 12. Status Lifecycle

```
Created → Assigned → InTransit → Delivered
```

Invalid transitions are rejected at compile-time and runtime.

---

# 13. Key Features

* Shipment tracking
* Role-based access control
* Payment processing
* Complaint management
* Audit history
* Event-driven notifications
* Revenue reporting
* Admin monitoring
* WebSocket tracking updates

---

# 14. Project Purpose

This project demonstrates:

Real-world Rust backend architecture
Domain-Driven Design (DDD) in production systems
State machine modeling for logistics workflows
Async, non-blocking system design (Tokio + Actix)
Clean modular monolith architecture
Migration of enterprise system from Scala → Rust

---

# 15. License

MIT
