use std::collections::HashMap;
use std::hash::BuildHasher;

// convert HashMap to String
pub fn convert_map_to_url_query_param<K: ToString, V: ToString, S: BuildHasher>(
    map: &HashMap<K, V, S>,
) -> String {
    let mut s = String::new();
    for (k, v) in map.iter() {
        s.push_str(&k.to_string());
        s.push_str("=");
        s.push_str(&v.to_string());
        s.push_str("&");
    }

    return s;
}

#[cfg(test)]
mod test_collections {

    use super::*;

    #[test]
    fn test_convert_map_to_string() {
        crate::_log::init();

        let mut payload: HashMap<&str, &str> = HashMap::new();
        payload.insert("client_id", "xxx");
        payload.insert("client_key", "yyy");
        payload.insert("scope", "read");

        let s = convert_map_to_url_query_param(&payload);
    }
}
