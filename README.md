# git-prompt

zsh的git-prompt太慢，rust重写一个：

1. 编译

```
git clone git@github.com:jayli/git-prompt.git
cd git-prompt
sh build.sh
cp target/debug/git-prompt /usr/local/bin/
```

2. 配置

修改`~/.oh-my-zsh/lib/git.zsh`：

```diff
 function git_prompt_info() {
+  echo "${ZSH_THEME_GIT_PROMPT_PREFIX}$(git-prompt)${ZSH_THEME_GIT_PROMPT_SUFFIX}"
+  return 0
   # If we are on a folder not tracked by git, get out.
   # Otherwise, check for hide-info at global and local repository level
```
