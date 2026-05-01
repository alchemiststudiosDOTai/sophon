use std::collections::HashSet;
use std::env;
use std::fs;
use std::process::Command;

#[test]
fn changelog_references_all_merged_prs() {
    let changelog_ids = extract_changelog_pr_ids();
    let merged_pr_ids = extract_github_merged_pr_ids().unwrap_or_else(|error| {
        if is_ci() {
            panic!("failed to fetch merged PR IDs from GitHub: {error}");
        }
        eprintln!("skipping changelog merged-PR coverage outside CI: {error}");
        HashSet::new()
    });

    if merged_pr_ids.is_empty() {
        return;
    }

    let mut missing: Vec<_> = merged_pr_ids.difference(&changelog_ids).copied().collect();
    missing.sort_unstable();

    assert!(
        missing.is_empty(),
        "CHANGELOG.md is missing PR IDs: {:?}",
        missing
    );
}

fn extract_changelog_pr_ids() -> HashSet<u32> {
    let content = fs::read_to_string("CHANGELOG.md").expect("CHANGELOG.md must exist");
    find_hash_ids(&content)
}

fn extract_github_merged_pr_ids() -> Result<HashSet<u32>, String> {
    let repo = env::var("GITHUB_REPOSITORY")
        .unwrap_or_else(|_| "alchemiststudiosDOTai/sophon".to_string());
    let api_url =
        env::var("GITHUB_API_URL").unwrap_or_else(|_| "https://api.github.com".to_string());
    let mut ids = HashSet::new();

    for page in 1.. {
        let url =
            format!("{api_url}/repos/{repo}/pulls?state=closed&base=main&per_page=100&page={page}");
        let output = github_api_request(&url)?;
        let pulls: Vec<serde_json::Value> = serde_json::from_slice(&output)
            .map_err(|error| format!("failed to parse GitHub API response: {error}"))?;

        if pulls.is_empty() {
            break;
        }

        for pull in &pulls {
            if pull.get("merged_at").is_none_or(serde_json::Value::is_null) {
                continue;
            }
            let number = pull
                .get("number")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| "GitHub API response is missing a numeric PR number".to_string())?;
            let id = u32::try_from(number)
                .map_err(|error| format!("PR number {number} does not fit in u32: {error}"))?;
            ids.insert(id);
        }

        if pulls.len() < 100 {
            break;
        }
    }

    if ids.is_empty() {
        return Err("GitHub API returned no merged PR IDs".to_string());
    }

    Ok(ids)
}

fn github_api_request(url: &str) -> Result<Vec<u8>, String> {
    let mut command = Command::new("curl");
    command.args([
        "--fail",
        "--silent",
        "--show-error",
        "--location",
        "-H",
        "Accept: application/vnd.github+json",
        "-H",
        "X-GitHub-Api-Version: 2022-11-28",
    ]);

    if let Some(token) = github_token() {
        command
            .arg("-H")
            .arg(format!("Authorization: Bearer {token}"));
    }

    let output = command
        .arg(url)
        .output()
        .map_err(|error| format!("failed to run curl: {error}"))?;

    if !output.status.success() {
        return Err(format!(
            "curl failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(output.stdout)
}

fn github_token() -> Option<String> {
    ["GITHUB_TOKEN", "GH_TOKEN"]
        .into_iter()
        .filter_map(|key| env::var(key).ok())
        .find(|token| !token.trim().is_empty())
}

fn is_ci() -> bool {
    env::var("CI").is_ok() || env::var("GITHUB_ACTIONS").is_ok()
}

fn find_hash_ids(text: &str) -> HashSet<u32> {
    let mut ids = HashSet::new();
    for candidate in text.split('#').skip(1) {
        let digits: String = candidate
            .chars()
            .take_while(|char| char.is_ascii_digit())
            .collect();
        if let Ok(id) = digits.parse::<u32>() {
            ids.insert(id);
        }
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::find_hash_ids;
    use std::collections::HashSet;

    #[test]
    fn find_hash_ids_extracts_numeric_hash_references() {
        assert_eq!(
            find_hash_ids("Added thing (#12), fixed #9, ignored #abc and #."),
            HashSet::from([9, 12])
        );
    }
}
