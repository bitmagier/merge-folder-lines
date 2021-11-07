use std::io::{self, BufRead};

use structopt::StructOpt;

fn read_stdin() -> Vec<String> {
    let mut result = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        result.push(line.expect("unable to read input line"));
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_contained_in() {
        assert_eq!(false, is_contained_in("/root", "/opt"));
        assert_eq!(false, is_contained_in("/root", "/root/bla"));
        assert!(is_contained_in("/root/bla", "/root"));
        assert!(is_contained_in("/root", "/root"));
    }

    #[test]
    fn test_is_line_redundant() {
        let lines = vec![
            "/data".to_string(),
            "/data/media".to_string(),
            "/data/foo".to_string(),
            "/data/media/music".to_string(),
            "/data/media2".to_string(),
        ];
        assert_eq!(false, is_line_redundant(0, &lines));
        assert!(is_line_redundant(1, &lines));
        assert!(is_line_redundant(2, &lines));
        assert!(is_line_redundant(3, &lines));
        assert!(is_line_redundant(4, &lines));

        let lines = &lines[1..].to_vec();
        assert_eq!(false, is_line_redundant(0, lines));
        assert_eq!(false, is_line_redundant(1, lines));
        assert!(is_line_redundant(2, lines));
        assert_eq!(false, is_line_redundant(3, lines));
    }
}

// returns true iff candidate_path is contained under reference_path (or both are equal)
fn is_contained_in(candidate_path: &str, reference_path: &str) -> bool {
    if candidate_path == reference_path {
        return true;
    }
    candidate_path.len() > reference_path.len()
        && candidate_path.chars().nth(reference_path.len()).expect("illegal reference") == '/'
        && reference_path == &candidate_path[..reference_path.len()]
}

fn is_line_redundant(line_idx: usize, lines: &Vec<String>) -> bool {
    assert!(line_idx < lines.len(), "line_idx bigger than number of lines available");

    for i in 0..lines.len() {
        if i != line_idx && is_contained_in(&lines[line_idx], &lines[i]) {
            return true;
        }
    }
    false
}

fn merge(path_lines: &mut Vec<String>) {
    if path_lines.len() < 2 {
        return;
    }

    let mut i = 0;
    while i < path_lines.len() {
        if is_line_redundant(i, path_lines) {
            path_lines.remove(i);
        } else {
            i = i + 1
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "merge-folder-names", about = "Merges a list of unix folder/file names to a set of distinct locations.\
\nTakes input (one line per folder/file name) from STDIN. Output is printed to STDOUT.\
\nReport bugs to bitmagier@mailbox.org")]
struct Cli {}

fn main() {
    Cli::from_args();

    let mut path_lines = read_stdin();
    merge(&mut path_lines);

    for l in path_lines {
        println!("{}", l);
    }
}

