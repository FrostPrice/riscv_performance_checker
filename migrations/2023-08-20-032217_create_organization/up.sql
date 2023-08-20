-- Your SQL goes here
CREATE TABLE organizations (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    clock INTEGER NOT NULL,
    cpi_instruction_r INTEGER NOT NULL,
    cpi_instruction_i INTEGER NOT NULL,
    cpi_instruction_l INTEGER NOT NULL,
    cpi_instruction_s INTEGER NOT NULL,
    cpi_instruction_b INTEGER NOT NULL,
    cpi_instruction_u INTEGER NOT NULL,
    cpi_instruction_j INTEGER NOT NULL
);