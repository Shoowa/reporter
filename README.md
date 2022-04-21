# Guide

## Installation
1. [Install](https://www.rust-lang.org/tools/install) the Rust language.
2. Navigate to the root directory of this project.
3. `cargo build --release`


## Operation
The executable receives three arguments.
1. Starting time including a timezone.
2. Ending time includind a timezone.
3. A list of files.

```bash
./target/release/reporter --start="2017-05-05 03:20:00 -04:00" --end="2017-05-05 03:27:00 -04:00" ~/Downloads/log_sample.txt whatever.txt
```

The application will open valid files, and continue to operate after failing to open a file or failing to open a non-existent file.


## File format
The format of the file needs to resemble the following hypothetical log:
```
1493969101.638 | https | dev.acme.com | post | 200 | 149399101591904,149396911639670, | iad |  | 10.10.3.53 | 0.006
1493969101.638 | https | acme.com | post | 200 | 149396910159904,149396910163967, | iad |  | 10.10.3.52 | 0.006
1493969101.639 | https | dev.acme.com | get | 500 | 149396101592060,149396910168753, | iad |  | 10.13.4.52 | 0.007
1493969101.639 | https | dev.acme.com | post | 200 | 149399101540526,149396910137334, | iad |  | 10.10.3.1 | 0.003
1493969101.639 | https | acme.com | post | 200 | 149396910159904,149396910163960, | iad |  | 10.13.4.52 | 0.006
1493969101.639 | https | dev.acme.com | get | 500 | 149396101549698,149396910164458, | iad |  | 10.10.3.53 | 0.007
1493969101.640 | https | dev.acme.com | get | 500 | 149396101530544,149396910145293, | iad |  | 10.13.4.52 | 0.005
1493969101.640 | https | dev.acme.com | post | 200 | 149399101573491,149396910645813, | iad |  | 10.20.5.2 | 0.004
1493969101.641 | https | acme.com | get | 200 | 149396910155007,1493969101589112, | iad |  | 10.20.5.1 | 0.054
1493969101.642 | https | dev.acme.com | get | 200 | 149396101540734,149396910168192, | iad |  | 10.10.3.1 | 0.005
1493969101.642 | https | dev.acme.com | post | 200 | 149399101594772,149396910140014, | iad |  | 10.20.5.2 | 0.002
1493969101.643 | https | dev.acme.com | post | 200 | 149399101591904,149396911639670, | iad |  | 10.10.3.53 | 0.006
1493969101.644 | https | acme.com | post | 200 | 149396910159904,149396910163967, | iad |  | 10.10.3.52 | 0.006
1493969101.645 | https | dev.acme.com | get | 500 | 149396101592060,149396910168753, | iad |  | 10.13.4.52 | 0.007
1493969101.645 | https | dev.acme.com | post | 200 | 149399101540526,149396910137334, | iad |  | 10.10.3.1 | 0.003
1493969101.645 | https | acme.com | post | 200 | 149396910159904,149396910163960, | iad |  | 10.13.4.52 | 0.006
```


## Documentation
```
cargo doc --open
```


## Asynchronicity
This application relies on the Tokio runtime. But not all operations are truly asynchronous. [Opening a file](https://docs.rs/tokio/latest/tokio/fs/fn.read.html#) is still synchronous:
> This operation is implemented by running the equivalent blocking operation on a separate thread pool...

Even the reading of the buffer line by line isn't harnessing the asynchronous framework fully, because that functionality hasn't been converted to a streaming
operation. The ability to enable streaming relies on importing the [tokio-stream](https://docs.rs/tokio-stream/latest/tokio_stream/) crate, and including a trait that can change the _Lines_ struct representing the buffer in the `parse_bytes()` into a stream, and rewriting a few lines of code.
