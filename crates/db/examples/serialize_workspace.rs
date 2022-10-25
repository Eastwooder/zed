use std::{fs::File, path::Path};

const TEST_FILE: &'static str = "test-db.db";

fn main() -> anyhow::Result<()> {
    let db = db::Db::open_in_memory();
    if db.real().is_none() {
        return Err(anyhow::anyhow!("Migrations failed"));
    }
    let file = Path::new(TEST_FILE);

    let f = File::create(file)?;
    drop(f);

    db.write_kvp("test", "1")?;
    db.write_kvp("test-2", "2")?;

    let workspace_1 = db.workspace_for_worktree_roots(&[]);
    let workspace_2 = db.workspace_for_worktree_roots(&[]);
    let workspace_3 = db.workspace_for_worktree_roots(&[]);
    let workspace_4 = db.workspace_for_worktree_roots(&[]);
    let workspace_5 = db.workspace_for_worktree_roots(&[]);
    let workspace_6 = db.workspace_for_worktree_roots(&[]);
    let workspace_7 = db.workspace_for_worktree_roots(&[]);

    db.update_worktree_roots(&workspace_1.workspace_id, &["/tmp1"])
        .unwrap();
    db.update_worktree_roots(&workspace_2.workspace_id, &["/tmp1", "/tmp2"])
        .unwrap();
    db.update_worktree_roots(&workspace_3.workspace_id, &["/tmp1", "/tmp2", "/tmp3"])
        .unwrap();
    db.update_worktree_roots(&workspace_4.workspace_id, &["/tmp2", "/tmp3"])
        .unwrap();
    db.update_worktree_roots(&workspace_5.workspace_id, &["/tmp2", "/tmp3", "/tmp4"])
        .unwrap();
    db.update_worktree_roots(&workspace_6.workspace_id, &["/tmp2", "/tmp4"])
        .unwrap();
    db.update_worktree_roots(&workspace_7.workspace_id, &["/tmp2"])
        .unwrap();

    db.write_to(file).ok();

    println!("Wrote database!");

    Ok(())
}
