mod models;
use std::{fs::File, io::Write};

use models::{VbrGetJob, VbrPostJob};

pub fn convert_file(read_path: String, write_path: String) {
    let file = File::open(read_path).expect("Failed to open file");

    let job: VbrGetJob = serde_json::from_reader(file).expect("Failed to parse file");

    let post_job = VbrPostJob::from(job);

    let mut file = File::create(write_path).expect("Failed to create file");

    file.write(serde_json::to_string(&post_job).unwrap().as_bytes())
        .expect("Failed to write to file");
}
