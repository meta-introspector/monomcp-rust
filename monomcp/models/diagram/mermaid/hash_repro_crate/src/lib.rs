use rustc_hash::FxHashMap;
use std::collections::HashMap;

#[test]
fn test_hash_map_creation() {
    let _map: FxHashMap<String, String> = FxHashMap::default();
}
