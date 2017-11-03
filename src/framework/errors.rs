error_chain! {
    foreign_links {
        Hyper(::hyper::Error);
        Serde(::serde_json::Error);
        Io(::std::io::Error);
        Db(::diesel::result::Error);
    }

    errors {
        UnknownContentType(t: String) {
            description("unknown content type")
            display("invalid content type: '{}'", t)
        }
    }
}
