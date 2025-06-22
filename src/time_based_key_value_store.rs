use std::collections::HashMap;

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
pub struct TimeMap {
    map: HashMap<String, Vec<(String, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
    pub fn new() -> Self {
        TimeMap {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map
            .entry(key)
            .or_insert(vec![])
            .push((value, timestamp));
    }

    pub fn get(&mut self, key: String, timestamp: i32) -> String {
        if let Some(array_value) = self.map.get(&key) {
            let mut result = String::new();
            let (mut low, mut high) = (0 as i32, (array_value.len() - 1) as i32);

            while low <= high {
                let middle = low + (high - low) / 2;

                if array_value[middle as usize].1 <= timestamp {
                    low = middle + 1;
                    result = array_value[middle as usize].0.clone();
                } else {
                    high = middle - 1;
                }
            }

            return result;
        }

        String::new()
    }
}
