CREATE TABLE payments (
    reference_number TEXT PRIMARY KEY,

    customer_id UUID NOT NULL,
    shipment_id UUID NOT NULL,

    amount NUMERIC(12,2) NOT NULL,

    status TEXT NOT NULL,
    payment_method TEXT NOT NULL,

    gateway_transaction_id TEXT,
    failure_reason TEXT,

    paid_at TIMESTAMP WITH TIME ZONE NOT NULL,

    CONSTRAINT shipment_foreign_key
        FOREIGN KEY (shipment_id)
        REFERENCES shipments(id)
        ON DELETE CASCADE,

    CONSTRAINT unique_payment_per_shipment
        UNIQUE (shipment_id)
);

CREATE INDEX idx_payments_status
    ON payments(status);

CREATE INDEX idx_payments_payment_method
    ON payments(payment_method);