# VBR Get to Post job converter

This is a simple app to convert a VBR Get json file to a VBR Post json file.

Similar to the vcli utils functionality.

```
Convert from VBR Get to Post

Options:
  -r, --read-path <READ_PATH>
  -w, --write-path <WRITE_PATH>
  -h, --help                     Print help
```

### Usage

```
job_convert.exe -r get_job.json -w post_job.json
```

### Build

```
cargo build --release
```
