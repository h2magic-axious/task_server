-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS tasks (
    "id" UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    "title" VARCHAR(200) NOT NULL,
    "effective" BOOLEAN DEFAULT true,
    "lifetime" INT DEFAULT 60,
    "created_time" TIMESTAMP DEFAULT now(),
    "doing_time" TIMESTAMP NOT NULL,
    "loop" BOOLEAN DEFAULT false,
    "running" JSON DEFAULT NULL,
    "failed" JSON DEFAULT NULL
);
