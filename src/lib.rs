mod models;
use std::{fs::File, io::Write};
use anyhow::{Result, Context};

use models::{VbrGetJob, VbrPostJob};

pub fn convert_file(read_path: String, write_path: String) -> Result<()> {
    let file = File::open(read_path).with_context(|| format!("Filed to open json file"))?;

    let job: VbrGetJob = serde_json::from_reader(file).with_context(|| format!("Filed to parse json"))?;

    let post_job = VbrPostJob::from(job);

    let mut file = File::create(write_path).with_context(|| format!("Filed to create file"))?;

    file.write_all(serde_json::to_string(&post_job).unwrap().as_bytes()).with_context(|| format!("Filed to write json file"))?;

    Ok(())
}
