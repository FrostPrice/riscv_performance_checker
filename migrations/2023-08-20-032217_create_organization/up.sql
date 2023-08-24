-- Your SQL goes here
CREATE TABLE organizations (
    id TEXT PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    clock FLOAT NOT NULL,
    cpi_instruction_r FLOAT NOT NULL,
    cpi_instruction_i FLOAT NOT NULL,
    cpi_instruction_l FLOAT NOT NULL,
    cpi_instruction_s FLOAT NOT NULL,
    cpi_instruction_b FLOAT NOT NULL,
    cpi_instruction_u FLOAT NOT NULL,
    cpi_instruction_j FLOAT NOT NULL
);