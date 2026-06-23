CREATE TABLE users(
    id UUID NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    hash_password TEXT NOT NULL,
    phone_number TEXT NOT NULL,
    status TEXT NOT NULL,
    role TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
)