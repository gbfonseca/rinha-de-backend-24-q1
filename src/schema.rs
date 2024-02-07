// @generated automatically by Diesel CLI.

diesel::table! {
    clientes (id) {
        id -> Int4,
        nome -> Varchar,
        limite -> Int4,
        saldo_inicial -> Int4,
    }
}
