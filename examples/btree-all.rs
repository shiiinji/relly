use anyhow::Result;

use relly::btree::{BTree, SearchMode};
use relly::buffer::{BufferPool, BufferPoolManager};
use relly::disk::{DiskManager, PageId};

fn main() -> Result<()> {
  let disk = DiskManager::open("simple.rly")?;
  let pool = BufferPool::new(10);
  let mut bugmgr = BufferPoolManager::new(disk, pool);

  let btree = BTree::new(PageId(0));
  let mut iter = btree.search(&mut bugmgr, SearchMode::Start)?;

  while let Some((key, value)) = iter.next(&mut bugmgr)? {
    println!("{:02x?} = {:02x?}", key, value);
  }
  Ok(())
}
