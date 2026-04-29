use std::collections::HashSet;
use std::fs;
use std::process::Command;

#[test]
fn changelog_references_all_merged_prs() {
    let changelog_ids = extract_changelog_pr_ids();
    let git_pr_ids = extract_git_merged_pr_ids();
    let missing: Vec<_> = git_pr_ids
        .iter()
        .filter(|id| !changelog_ids.contains(id))
        .collect();
    assert!(
        missing.is_empty(),
        "CHANGELOG.md is missing PR IDs: {:?}",
        missing
    );
}

fn extract_changelog_pr_ids() -> HashSet<u32> {
    let content = fs::read_to_string("CHANGELOG.md").expect("CHANGELOG.md must exist");
    regex_find_pr_ids(&content)
}

fn extract_git_merged_pr_ids() -> HashSet<u32> {
    let output = Command::new("git")
        .args(["log", "--all", "--format=%s"])
        .output()
        .expect("git must be available");
    let subjects = String::from_utf8_lossy(&output.stdout);
    regex_find_pr_ids(&subjects)
}

fn regex_find_pr_ids(text: &str) -> HashSet<u32> {
    let mut ids = HashSet::new();
    for line in text.lines() {
        let remaining = &*line;
        let mut start = 0;
        while start < remaining.len() {
            if let Some(pos) = remaining[start..].find('#') {
                let after = &remaining[start + pos + 1..];
                let num: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
                if !num.is_empty() {
                    if let Ok(id) = num.parse::<u32>() {
                        ids.insert(id);
                    }
                }
                start += pos + 1;
            } else {
                break;
            }
        }
    }
    ids
}
