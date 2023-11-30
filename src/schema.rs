// @generated automatically by Diesel CLI.

diesel::table! {
    bin_files (id) {
        id -> Text,
        file -> Text,
    }
}

diesel::table! {
    organizations (id) {
        id -> Text,
        created_at -> Timestamp,
        clock -> Float,
        cpi_instruction_r -> Float,
        cpi_instruction_i -> Float,
        cpi_instruction_l -> Float,
        cpi_instruction_s -> Float,
        cpi_instruction_b -> Float,
        cpi_instruction_u -> Float,
        cpi_instruction_j -> Float,
    }
}

diesel::allow_tables_to_appear_in_same_query!(bin_files, organizations,);
