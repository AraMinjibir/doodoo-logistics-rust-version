    CREATE TABLE support(
        id UUID NOT NULL,
        user_id UUID NOT NULL,
        shipment_id UUID NOT NULL,
        subject TEXT NOT NULL,
        description TEXT NOT NULL,
        status TEXT NOT NULL,
        created_at TIMESTAMP WITH TIME ZONE NOT NULL,
        resolved_at TIMESTAMP WITH TIME ZONE,
        comment JSONB NOT NULL DEFAULT '[]'
    );

    CREATE INDEX idx_support_status
    ON support(status)