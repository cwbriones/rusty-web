error_chain! {
    foreign_links {
        Hyper(::hyper::Error);
        Serde(::serde_json::Error);
        Io(::std::io::Error);
    }
}
