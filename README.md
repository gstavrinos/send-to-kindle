<img src="https://raw.githubusercontent.com/gstavrinos/send-to-kindle/master/media/send-to-kindle.png" style="width: 330px;"/>
# Send to Kindle

send-to-kindle is a command-line utility and rust library for sending files to your kindle app
 or devices by (ab)using the www.amazon.com./sendtokindle web interface.

**For this reason, it should be used with caution. Getting suspended by Amazon's spam
prevention systems is always a possibility. USE send-to-kindle AT YOUR OWN RISK!**

# Command-line tool basic usage

```
cargo run -- --username <username> --password <password> --directory <path_to_books>
--extension epub
```

The `--directory` flag can be swapped with the `--file` flag to just send a single file. If an 
extension is provided, it will ensure that the selected file has the requested extension.
 
For more info on the command-line utility and flags for corner cases, use the `--help` flag.

# Library usage

Just two functions are provided: One for a list of strings representing the files to be
uploaded (<a href="https://docs.rs/send_to_kindle/fn.send_files_to_kindle.html">send_files_to_kindle</a>), and one for a path to a file or directory that can be filtered using a string
for the files' extension (<a href="https://docs.rs/send_to_kindle/fn.send_to_kindle.html">send_to_kindle</a>). (epub, azw3, mobi etc).

For more info, click on each function's definition and read the extensive documentation there.
