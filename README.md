# DooDoo Logistics (Rust Version)

**Rust • Actix Web • Tokio • PostgreSQL • REST • WebSockets**

DooDoo Logistics (Rust Version) is a production-oriented logistics and delivery management backend inspired by industrial-grade systems such as DHL, Bolt Logistics, and FedEx. This implementation rewrites the original Scala/Play system in Rust while preserving its domain-driven architecture, strict state transitions, and asynchronous event-driven design.

The system is built as a **Modular Monolith**, focusing on correctness, maintainability, and performance without introducing unnecessary distributed complexity. It demonstrates how to design and implement a realistic logistics backend using Rust's strong type system, memory safety guarantees, and asynchronous runtime.

The goal is to showcase how a real-world logistics platform can be implemented in Rust using modern async architecture, strong domain modeling, and production-ready design principles.

---

# 1. Problem Statement

Logistics platforms must maintain **100% data consistency** and **operational transparency**.
DooDoo Logistics (Rust Version) addresses these challenges through:

### Deterministic Lifecycle Management

Prevents illegal state transitions (e.g., Created → Delivered without InTransit).

### Role-Based Access Control (RBAC)

Defines strict boundaries between:

* Customers
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
* Track shipment lifecycle
* View complete shipment history

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

### Payment Processing

* Create payment transactions
* Link payment to shipment
* Secure transaction recording

### Payment Status Tracking

* Pending
* Successful
* Failed
* Refunded

### Notifications & Alerts

* Payment success notification
* Payment failure alert
* Support agent alerts

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
Created → Accepted → InTransit → Delivered
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

* Real-world Rust backend architecture
* Async application design
* Domain-driven development
* State machine modeling
* Clean modular monolith structure
* Production-oriented system design

---

# 15. License

MIT
