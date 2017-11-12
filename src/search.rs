extern crate fuzzymatch;

use std::fs::File;
use std::io::{BufReader,BufRead};
use std::vec::Vec;

use self::fuzzymatch::algo::fuzzy_match;

#[derive(Clone,Debug,Serialize,Deserialize)]
pub struct Match {
    pub score: i32,
    pub movie: String,
}

pub fn get_matches(pattern: &str) -> Vec<Match> {
    let mut movie_list: Vec<Match> = Vec::new();
    let file = File::open("data/tmdb-5000.txt").expect("cannot open file");

    for line in BufReader::new(file).lines() {
        let movie = line.unwrap();
        let (is_match, score, matched_indices) = fuzzy_match(pattern, &movie);
        if is_match {
            movie_list.push(Match {score: score, movie: highlight_matched_indices(&movie, matched_indices)});
        }
    }
    movie_list.sort_by_key(|k| -k.score);

    if movie_list.len() > 50 {
        return Vec::from(&movie_list[1..50]);
    } else {
        return movie_list;
    }
}


fn highlight_matched_indices(text: &str, matched_indices: Vec<usize>) -> String {
    let mut formatted_str = Vec::new();
    let mut last_idx = 0;

    for idx in matched_indices {
        // Take piece between last match and this match.
        formatted_str.push(text.chars().skip(last_idx).take(idx - last_idx).collect::<String>());
        // Add identifier for matches.
        formatted_str.push("<b>".to_string().clone());
        // Add actual match.
        formatted_str.push(text.chars().skip(idx).take(1).collect());
        // Add after element.
        formatted_str.push("</b>".to_string().clone());
        last_idx = idx + 1
    }
    // If there's characters left after the last match, make sure to append them.
    if last_idx != text.len() {
        formatted_str.push(text.chars().skip(last_idx).take_while(|_| true).collect::<String>());
    }

    return formatted_str.join("");
}