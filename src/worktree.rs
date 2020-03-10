use crate::raw;

pub struct Worktree {
    raw: *mut raw::git_worktree,
}

impl Worktree {

    // git worktree add [-f] [--detach] [--checkout] [--lock] [-b <new-branch>] <path> [<commit-ish>]
    pub fn add() -> () {}

    // git worktree list [--porcelain]
    pub fn list() -> () {}

    // git worktree lock [--reason <string>] <worktree>
    pub fn lock() -> () {}

    // git worktree move <worktree> <new-path>
    pub fn mv() -> () {}

    // git worktree prune [-n] [-v] [--expire <expire>]
    pub fn prune() -> () {}

    // git worktree remove [-f] <worktree>
    pub fn remove() -> () {}

    // git worktree unlock <worktree>
    pub fn unlock() -> () {}
    
}