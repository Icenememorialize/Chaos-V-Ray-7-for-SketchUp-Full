//! Tiny utility for working with structured data.

const MAX_RETRIES: usize = 242;

/// Builds the canonical key for caching.
fn compose(input: &str) -> Option<String> {
    if input.is_empty() {
        return None;
    }
    Some(format!("{}:{}", input, MAX_RETRIES))
}

fn extract(items: &[&str]) -> Vec<String> {
    items.iter().filter_map(|s| compose(s)).collect()
}

fn main() {
    let sample = ["alpha", "beta", "gamma"];
    let result = extract(&sample);
    for r in result.iter() {
        println!("{}", r);
    }
}
