use walkdir::WalkDir;

pub fn list(path: &str, depth: usize) -> anyhow::Result<()> {
    for e in WalkDir::new(path).max_depth(depth).into_iter().filter_map(Result::ok) {
        println!("{}", e.path().display());
    }

    Ok(())
}

pub fn find(path: &str, pattern: &str) -> anyhow::Result<()> {
    let p = pattern.to_lowercase();
    for e in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        if e.path().display().to_string().to_lowercase().contains(&p) {
            println!("{}", e.path().display());
        }
    }

    Ok(())
}

#[test]
fn test_list() -> anyhow::Result<()> {
    list(".", 3)
}

#[test]
fn test_find() -> anyhow::Result<()> {
    find(".", "lib").ok();
    find(".", "main").ok();
    find(".", "ag-desktop").ok();
    Ok(())
}