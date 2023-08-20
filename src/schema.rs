// @generated automatically by Diesel CLI.

diesel::table! {
    organizations (id) {
        id -> Nullable<Text>,
        created_at -> Timestamp,
        clock -> Integer,
        cpi_instruction_r -> Integer,
        cpi_instruction_i -> Integer,
        cpi_instruction_l -> Integer,
        cpi_instruction_s -> Integer,
        cpi_instruction_b -> Integer,
        cpi_instruction_u -> Integer,
        cpi_instruction_j -> Integer,
    }
}
