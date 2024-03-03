#![doc(html_root_url = "https://docs.rs/bowling_score/1.2.1")]
//! bowling_score bowling score app for Rust
//!

use std::error::Error;

use bscore::bgame::bowling_score;

/// main
pub fn main() -> Result<(), Box<dyn Error>> {
  bowling_score(false, "")
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use bscore::bgame::bscore;

  /// test score
  #[test]
  fn test_score() {
    assert_eq!(bscore("xxxxxxxxxxxxxx", true).unwrap(), [300, 300, 300]);
  }
}
