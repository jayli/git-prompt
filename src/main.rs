use git2::{Repository, StatusOptions};
use std::env;
use std::path::PathBuf;
use anyhow::{Result, Context};

fn find_git_root(mut dir: PathBuf) -> Option<PathBuf> {
    // 判断是否果是在一个 git 目录中
    let is_git_repo: bool = Repository::discover(&dir).is_ok();
    if is_git_repo {
        while dir.exists() {
            let try_dir = Repository::open(&dir);
            match try_dir {
                Ok(_) => {
                    return Some(dir);
                },
                Err(_) => {
                    if !dir.pop() {
                        break;
                    }
                }
            } // end match
        } // end while
    } else {
        return None;
    }
    return None;
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
        let try_open = Repository::open(&git_root);

        match try_open {
            Ok(repo) => {
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
            },
            Err(_) => {}
        }
    } else {
        // println!("");
    }

    Ok(())
}
