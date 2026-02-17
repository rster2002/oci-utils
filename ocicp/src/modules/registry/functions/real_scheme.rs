pub fn real_scheme(scheme: &str) -> &str {
    match scheme {
        "https+docker"
            | "docker+https" => "https",
        "http+docker"
            | "docker+http" => "http",
        _ => scheme,
    }
}