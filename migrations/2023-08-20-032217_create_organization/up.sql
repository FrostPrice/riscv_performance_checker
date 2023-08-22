-- Your SQL goes here
CREATE TABLE organizations (
    id TEXT PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    clock DOUBLE NOT NULL,
    cpi_instruction_r DOUBLE NOT NULL,
    cpi_instruction_i DOUBLE NOT NULL,
    cpi_instruction_l DOUBLE NOT NULL,
    cpi_instruction_s DOUBLE NOT NULL,
    cpi_instruction_b DOUBLE NOT NULL,
    cpi_instruction_u DOUBLE NOT NULL,
    cpi_instruction_j DOUBLE NOT NULL
);