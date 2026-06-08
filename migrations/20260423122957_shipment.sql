
CREATE TABLE shipments (
    id UUID PRIMARY KEY,
    tracking_number TEXT,
    sender_name TEXT NOT NULL,

    recipient_name TEXT NOT NULL,
    recipient_street TEXT NOT NULL,
    recipient_city TEXT NOT NULL,
    recipient_state TEXT NOT NULL,
    recipient_country TEXT NOT NULL,
    recipient_postal_code TEXT NOT NULL,
    recipient_contact TEXT NOT NULL,

    weight DOUBLE PRECISION NOT NULL,
    length DOUBLE PRECISION NOT NULL,
    width DOUBLE PRECISION NOT NULL,
    height DOUBLE PRECISION NOT NULL,

    contents TEXT NOT NULL,
    status TEXT NOT NULL,

    estimated_delivery_date TIMESTAMP,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,

    cost DOUBLE PRECISION NOT NULL,

    proof_of_delivery JSONB NOT NULL,

    service_provider_id UUID
);