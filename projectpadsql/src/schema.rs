use diesel::prelude::*;

// https://gitter.im/diesel-rs/diesel?at=5d420302b0bf183ea3785273
table! {
    project {
        id -> Int4,
        name -> Varchar,
        icon -> Nullable<Binary>,
        has_dev -> Bool,
        has_uat -> Bool,
        has_stage -> Bool,
        has_prod -> Bool,
    }
}