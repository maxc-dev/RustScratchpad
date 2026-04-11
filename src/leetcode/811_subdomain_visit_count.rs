use std::collections::HashMap;

impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        for domain in cpdomains {
            let parts: Vec<&str> = domain.split(" ").collect();
            let count = parts[0].parse::<i32>().unwrap();

            let mut sb = Vec::with_capacity(parts[1].len());
            for c in parts[1].chars().into_iter().rev() {
                if c == '.' {
                    *map.entry(sb.clone()).or_insert(0) += count;
                }
                sb.push(c);
            }
            *map.entry(sb).or_insert(0) += count;
        }

        map.into_iter()
            .map(|(k, v)| format!("{} {}", v, k.into_iter().rev().collect::<String>()))
            .collect::<Vec<_>>()
    }
}
