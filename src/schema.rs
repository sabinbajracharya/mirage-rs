table! {
    allows (id) {
        id -> Integer,
        pid_endpoint -> Integer,
        pid_content -> Integer,
        status_code -> Integer,
        request_method -> Text,
        flag -> Integer,
    }
}

table! {
    apis (id) {
        id -> Integer,
        pid -> Integer,
        body -> Text,
        status_code -> Integer,
    }
}

table! {
    contents (id) {
        id -> Integer,
        pid_endpoint -> Integer,
        body -> Text,
        status_code -> Integer,
        request_method -> Text,
    }
}

table! {
    endpoints (id) {
        id -> Integer,
        path -> Text,
    }
}

joinable!(allows -> contents (pid_content));
joinable!(contents -> endpoints (pid_endpoint));

allow_tables_to_appear_in_same_query!(
    allows,
    apis,
    contents,
    endpoints,
);
