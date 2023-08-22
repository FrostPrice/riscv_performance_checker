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
        clock -> Double,
        cpi_instruction_r -> Double,
        cpi_instruction_i -> Double,
        cpi_instruction_l -> Double,
        cpi_instruction_s -> Double,
        cpi_instruction_b -> Double,
        cpi_instruction_u -> Double,
        cpi_instruction_j -> Double,
    }
}

diesel::allow_tables_to_appear_in_same_query!(bin_files, organizations,);
