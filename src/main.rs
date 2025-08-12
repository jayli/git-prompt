use git2::{Repository, StatusOptions};
use std::env;
use std::path::PathBuf;
use anyhow::{Result, Context};

fn find_git_root(mut dir: PathBuf) -> Option<PathBuf> {
    while dir.exists() {
        if Repository::discover(&dir).is_ok() {
            return Some(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

fn extract_last_segment(input: &str) -> Option<&str> {
    input.rsplit('/').next()
}

fn main() -> Result<()> {
    // 获取当前工作目录
    let mut current_dir = env::current_dir().context("Failed to get current directory")?;
    
    // 查找 Git 仓库的根目录
    if let Some(git_root) = find_git_root(current_dir.clone()) {
        // 打开 Git 仓库
        let repo = Repository::open(&git_root).context("Failed to open repository")?;

        // 获取当前分支名称
        let head = repo.head()?;
        let branch_name = head.name().unwrap_or("<detached>");

        // 检查是否有未提交的更改
        let mut options = StatusOptions::new();
        options.include_untracked(true);
        let statuses = repo.statuses(Some(&mut options))?;
        let has_changes = !statuses.is_empty();

        let changed_symble: &str = if has_changes { "*" } else { "" };
        let branch_name_only = extract_last_segment(branch_name);

        println!("({}{})", branch_name_only.unwrap_or_default(), changed_symble);

        // println!("Git Root Directory: {:?}", git_root);
        // println!("Branch: {}", branch_name);
        // println!("Has Changes: {}", if has_changes { "Yes" } else { "No" });
    } else {
        println!("");
    }

    Ok(())
}
