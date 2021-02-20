# lnk_parser

`lnk_parser` is a full rust implementation to parse windows LNK files. The parsed data could formatted to JSON, JSONL and CSV (default).

## how to use the lib ?

Start by adding the lib to your `Cargo.toml` file as follows:

```
lnk_parser = "0.1.0"
```

or from the GitHub repo (latest updates):

```
lnk_parser = { git="https://github.com/AbdulRhmanAlfaifi/lnk_parser" }
```

### Example

Here is an example of using the lib to parse LNK file from path:

```rust
use std::fs::File;
fn main(){
    // Open the LNK file
    let file = File::open("samples\\WIN10\\1607_14393\\windows_generated.lnk").unwrap();
    // Pass the `File` instance to `from_reader` function.
    // `std::fs::File` implements `Read` & `Seek` traits.
    let lnk_file = LNKParser::from_reader(file);
    println!("{:?}", lnk_file);
}
```

sample output in JSON format:

```json
{
    "target_full_path": "C:\\Users\\u0041\\Desktop\\test\\test.txt",
    "lnk_file_metadata": {
        "full_path": "C:\\Users\\u0041\\Documents\\Projects\\LNKParser-rs\\samples\\WIN10\\1607_14393\\windows_generated.lnk",
        "mtime": "2021-02-08T12:52:20Z",
        "atime": "2021-02-13T19:14:07Z",
        "ctime": "2021-02-08T12:52:13Z"
    },
    "shell_link_header": {
        "file_attr": [
            "ARCHIVE"
        ],
        "mtime": "2021-02-08T12:41:58Z",
        "atime": "2021-02-08T12:41:03Z",
        "ctime": "2021-02-08T12:41:03Z",
        "file_size": 4
    },
    "link_target_id_list": {
        "id_list": [
            {
                "shell_item_data": {
                    "FileEntry": {
                        "is_file": false,
                        "file_size": 0,
                        "last_modified": "2021-02-08T12:46:24Z",
                        "file_attr_flags": [
                            "DIRECTORY"
                        ],
                        "name": "test",
                        "extention_block": {
                            "ctime": "2021-02-08T12:46:24Z",
                            "atime": "2021-02-08T12:46:24Z",
                            "file_ref": {
                                "mft_entry": 91461,
                                "sequence_number": 3
                            },
                            "primary_name": "test"
                        }
                    }
                }
            },
            {
                "shell_item_data": {
                    "FileEntry": {
                        "is_file": true,
                        "file_size": 4,
                        "last_modified": "2021-02-08T12:42:00Z",
                        "file_attr_flags": [
                            "ARCHIVE"
                        ],
                        "name": "test.txt",
                        "extention_block": {
                            "ctime": "2021-02-08T12:41:04Z",
                            "atime": "2021-02-08T12:41:04Z",
                            "file_ref": {
                                "mft_entry": 90070,
                                "sequence_number": 3
                            },
                            "primary_name": "test.txt"
                        }
                    }
                }
            }
        ]
    },
    "link_info": {
        "volume_id": {
            "drive_type": "DRIVE_FIXED",
            "serial_number": "E02E-8A93"
        },
        "local_base_path": "C:\\Users\\u0041\\Desktop\\test\\test.txt"
    },
    "relative_path": "..\\..\\..\\..\\..\\Desktop\\test\\test.txt",
    "working_dir": "C:\\Users\\u0041\\Desktop\\test",
    "extra_data": {
        "extra_data_blocks": [
            {
                "Tracker": {
                    "machine_id": "win10",
                    "file_droid": "BD4FAD74-6A0A-11EB-8ECF-5076AFA95947",
                    "file_droid_birth": "BD4FAD74-6A0A-11EB-8ECF-5076AFA95947",
                    "volume_droid": "00D2581C-4749-44BD-9381-9BDFADF8A9DE",
                    "volume_droid_birth": "00D2581C-4749-44BD-9381-9BDFADF8A9DE"
                }
            }
        ]
    }
}
```
## how to use the binary ?

You can download the binary from the release section or from crates.io using the following command:

```
cargo install lnk_parser
```

That is it! you can execute as follows:

```
lnk_parser -h
```
# LNK File Structure

I wrote a blog post explains the LNK file structure, you can check it out from [here](https://u0041.co/blog/post/4).
