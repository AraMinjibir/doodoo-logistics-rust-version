# DooDoo Logistics (Rust Version)

**Rust • Actix Web • Tokio • PostgreSQL • SQLx • REST**

A production-oriented logistics and delivery backend rewritten from an existing Scala/Play Framework implementation into Rust. The project demonstrates modern backend engineering through asynchronous processing, Domain-Driven Design (DDD), strict domain modeling and event-driven architecture.

Designed as a modular monolith, it emphasizes correctness, maintainability, and scalability while showcasing the migration of an enterprise backend from Scala to Rust.

---

# Project Overview

DooDoo Logistics manages shipment lifecycles, payment processing, complaint resolution, user management, and operational monitoring for logistics platforms.

The system enforces business rules through deterministic state transitions, role-based authorization, audit history, and asynchronous domain events to ensure consistency and traceability throughout the shipment lifecycle.

---

# Business Problem

Logistics systems require strict correctness and operational visibility. This project addresses common challenges such as:

* Invalid shipment state transitions
* Weak role separation
* Missing audit history
* Blocking notification workflows
* Inconsistent payment enforcement

These challenges are solved through state-machine modeling, RBAC, event-driven notifications, and immutable audit records.

---

# Domain Model

The system models the logistics domain using explicit business entities.

| Aggregate    | Responsibility                     |
| ------------ | ---------------------------------- |
| Shipment     | Shipment lifecycle and tracking    |
| User         | Authentication and role management |
| Payment      | Shipment payment processing        |
| Complaint    | Customer issue management          |
| Notification | Asynchronous event notifications   |

Shipment lifecycle:

```
Created → Assigned → InTransit → Delivered
```

Invalid transitions are rejected through domain validation.

---

# Core Capabilities

* Shipment lifecycle management
* Role-Based Access Control (RBAC)
* JWT authentication
* Payment processing
* Complaint management
* Audit history
* Event-driven notifications
* Revenue reporting
* Administrative monitoring

---

# Workflow

```
Customer
    │
    ▼
Create Shipment
    │
    ▼
Payment Verified
    │
    ▼
Assign Courier
    │
    ▼
In Transit
    │
    ▼
Delivered
    │
    ▼
Publish Domain Event
    │
    ▼
Notification Service
```

---

# Architecture & Design Decisions

The project follows Domain-Driven Design (DDD) with a modular monolith architecture.

Key design decisions include:

* Domain-driven architecture
* Repository pattern with persistence ignorance
* Type-safe domain modeling using Rust enums
* Asynchronous processing with Tokio
* Event-driven notification system
* Clear separation between domain, application, and infrastructure layers

Architecture overview:

```
Clients
    │
REST API
    │
Actix Web
    │
Application Services
    │
Domain Layer
    │
Repository Traits
    │
PostgreSQL
```

---

# Technology Stack

**Backend**

* Rust
* Actix Web
* Tokio

**Persistence**

* PostgreSQL
* SQLx

**Security**

* JWT Authentication
* BCrypt Password Hashing

**Architecture**

* Domain-Driven Design (DDD)
* Repository Pattern
* Modular Monolith
* Event-Driven Design

---

# Testing Strategy

The project follows a layered testing approach.

* **Unit Tests** — domain rules and business logic
* **Integration Tests** — repositories, SQL queries, transactions
* **End-to-End Tests** — shipment lifecycle, payments, authentication

---

# Operational Readiness

### Live Demo

https://doodoo-logistics-rust.onrender.com

### Example Endpoints

```http
POST /shipments
GET  /shipments
GET  /shipments/{id}

POST /payments
GET  /payments/reference/{reference}
```

### Production Features

* JWT authentication
* Password hashing
* Database abstraction
* Async request handling
* Audit history
* Docker support

---

# Roadmap

## Completed

* Shipment Management
* Shipment State Machine
* RBAC
* Authentication
* Payment Processing
* Complaint Management
* Revenue Reporting
* Event Bus

---

# Running the Project

```bash
# Clone repository
git clone git@gitlab.com:AraMjb/doodoo-logistics-rust.git
cd doodoo-logistics-rust

# Configure environment
cp .env.example .env

# Start PostgreSQL
docker compose up -d

# Run database migrations
cargo sqlx migrate run

# Start the application
cargo run
```

## Docker

```bash
# Build image
docker build -t doodoo-logistics .

# Run container
docker run -p 8080:8080 doodoo-logistics
```
