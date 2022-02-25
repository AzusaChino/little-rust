use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct ClusterMap {
    app_id: String,
    host_ip: String,
}

/*
  Use Result<T, anyhow::Error>, or equivalently anyhow::Result<T>, as the return type of any fallible function.
  Within the function, use ? to easily propagate any error that implements the std::error::Error trait.
 */
fn get_cluster_info() -> Result<ClusterMap> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: ClusterMap = serde_json::from_str(&config)?;
    Ok(map)
}

fn try_main() -> Result<()> {
    let path: String = String::from("config.json");
    let _content = std::fs::read(&path)
        .with_context(|| format!("Failed to read instrs from {}", path))?;

    // println!("{}", content.join(&0));
    Ok(())
}

#[test]
fn test_try_main() {
    if let Err(err) = try_main() {
        eprintln!("ERROR: {}", err);
        err.chain().skip(1).for_each(|cause| eprintln!("because: {}", cause));
        std::process::exit(1);
    }
}

pub struct ImportantThing {
    path: PathBuf,
}

impl ImportantThing {
    pub fn detach(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn do_it(mut it: ImportantThing) -> Result<Vec<u8>> {
        it.detach().context("Failed to detach the important thing")?;
        let path = &it.path;
        let ctn = std::fs::read(path)
            .with_context(|| format!("failed to read str from {}", path.display()))?;
        Ok(ctn)
    }
}
