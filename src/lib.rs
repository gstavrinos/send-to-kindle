#![doc(
    html_logo_url = "https://raw.githubusercontent.com/gstavrinos/send-to-kindle/master/media/send-to-kindle256.png", 
    html_favicon_url = "https://raw.githubusercontent.com/gstavrinos/send-to-kindle/master/media/send-to-kindle128.png"
)]

//!
//! # Send to Kindle
//!
//! send-to-kindle is a command-line utility and rust library for sending files to your kindle app
//! or devices by (ab)using the www.amazon.com/sendtokindle web interface.
//!
//! **For this reason, it should be used with caution. Getting suspended by Amazon's spam
//! prevention systems is always a possibility. USE send-to-kindle AT YOUR OWN RISK!**
//!
//! # Command-line tool basic usage
//!
//! ```
//! cargo run -- --username <username> --password <password> --directory <path_to_books>
//! --extension epub
//! ```
//!
//! The `--directory` flag can be swapped with the `--file` flag to just send a single file. If an
//! extension is provided, it will ensure that the selected file has the requested extension.
//! 
//! For more info on the command-line utility and flags for corner cases, use the `--help` flag.
//!
//! # Library usage
//!
//! Just two functions are provided: One for a list of strings representing the files to be
//! uploaded (<a href="fn.send_files_to_kindle.html">send_files_to_kindle</a>), and one for a path to a file or directory that can be filtered using a string
//! for the files' extension (<a href="fn.send_to_kindle.html">send_to_kindle</a>). (epub, azw3, mobi etc).
//!
//! For more info, click on each function's definition and read the extensive documentation there.
//!
//!

use thirtyfour::prelude::{WebDriverResult, By};
use thirtyfour::{FirefoxCapabilities, WebDriver};
use thirtyfour::common::capabilities::firefox::FirefoxPreferences;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Username of the amazon account connected to kindle (app or device)
   #[arg(short, long)]
   pub username: String,

   /// Password of the amazon account connected to kindle (app or device)
   #[arg(short, long)]
   pub password: String,

   /// The single file that will be sent to kindle. Ignored if a directory is also specified
   #[arg(short, long, default_value_t = String::from(""))]
   pub file: String,

   /// The directory from which all files will be sent to kindle
   #[arg(short, long, default_value_t = String::from(""))]
   pub directory: String,

   /// Only the file(s) with this extension (e.g. epub) will be sent to kindle
   #[arg(short, long, default_value_t = String::from(""))]
   pub extension: String,

   /// Seconds to wait to upload per file, before giving up.
   #[arg(long, default_value_t = 60)]
   pub file_timeout: usize,

   /// Run the geckodriver daemon
   #[arg(long, default_value_t = true)]
   pub geckodriver_daemon: bool,

   /// Enable debugging mode, that runs the browser with GUI, does not automatically send the files and prompts for user input in terminal to close the window. Only for development purposes.
   #[arg(long, default_value_t = false)]
   pub debugging_mode: bool,

   /// Bypass the amazon(.com) url in case you need something else (e.g. .jp). You most probably won't need to use this.
   #[arg(long, default_value_t = String::from("https://www.amazon.com/sendtokindle"))]
   pub amazon_url: String,

}

impl Default for Args {
    fn default() -> Args {
        Args {
            username: String::from(""),
            password: String::from(""),
            file: String::from(""),
            directory: String::from(""),
            extension: String::from(""),
            file_timeout: 60,
            geckodriver_daemon: true,
            debugging_mode: false,
            amazon_url: String::from("https://www.amazon.com/sendtokindle"),
        }
    }
}

impl Args {
    pub fn new(u: &str, p: &str) -> Args {
        let mut args = Args::default();
        args.username = String::from(u);
        args.password = String::from(p);
        return args;
    }
}

/// * **Parameters:**
///
///     - username: The amazon username as str
///     - password: The amazon password as str
///     - files: A list of files to be uploaded as String Vec
///     - file_timeout: Seconds assigned to each file before uploading times out
///     - url: The amazon url to be used
///     - daemon: Whether to run the geckodriver daemon or not. If not, it will have to be run externally
///     - debugging_mode: For development purposes, enables GUI on the browser, does not send the files and prompts for user input to end the process
/// * **Description:** Send the _files_ to the kindle app and devices associated with the _username_ and _password_ amazon account
///
/// * **Notes:** It is recommended to use the Args::default or Args::new functions to ensure some sane defaults and easier setup. Please note that the _files_ vector of strings is not included in the Args struct and it is expected to be constructed separately.
pub async fn send_files_to_kindle(username: &str, password: &str, files: Vec<String>, file_timeout: usize, url: &str, daemon: bool, debugging_mode: bool) -> WebDriverResult<()> {
    let mut gd_daemon = std::process::Command::new("echo").stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).spawn()?;
    if daemon {
        gd_daemon = std::process::Command::new("geckodriver").stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null()).spawn()?;
    }
    let user_agent = "Linux";

    let mut prefs = FirefoxPreferences::new();
    prefs.set_user_agent(user_agent.to_string())?;

    let mut caps = FirefoxCapabilities::new();
    caps.set_preferences(prefs)?;
    if !debugging_mode {
        caps.set_headless()?;
    }

    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    driver.goto(url).await?;
    println!("Reached {}", url);

    let signin_button = driver.find(By::Id("s2k-dnd-sign-in-button")).await?;
    signin_button.click().await?;
    println!("Found sign in button");

    let email_input = driver.find(By::Css("input[type='email']")).await?;
    email_input.send_keys(username).await?;
    println!("Found email input and sent user email");
    let continue_button = driver.find(By::Css("input[type='submit'][id='continue']")).await?;
    continue_button.click().await?;
    println!("Found and clicked continue button");
    let password_input = driver.find(By::Css("input[type='password'][id='ap_password']")).await?;
    password_input.send_keys(password).await?;
    println!("Found password input and sent user password");
    let sis_button = driver.find(By::Css("input[type='submit'][id='signInSubmit']")).await?;
    sis_button.click().await?;
    println!("Found and clicked sign in button");
    driver.execute(r#"

        let elem = document.getElementById("s2k-home-wrapper");
        
        var input = document.createElement("input");
        input.id = "hacky-file-input";
        input.type = "file";
        input.multiple = true;
        elem.appendChild(input);
    "#, Vec::new()).await?;

    let file_input = driver.find(By::Id("hacky-file-input")).await?;
    for f in files.clone() {
        file_input.send_keys(f).await?;
    }
    driver.execute(r#"
        let elem = document.getElementById("s2k-home-wrapper");

        function CustomDataTransfer() {
          var f = document.getElementById("hacky-file-input").files;
          this.dropEffect = 'all';
          this.effectAllowed = 'all';
          this.items = [];
          this.types = ['Files'];
          this.files = f;
        };

        var customDropEvent = new DragEvent('drop');
        Object.defineProperty(customDropEvent, 'dataTransfer', {
          value: new CustomDataTransfer()
        });
        var button_input = document.createElement("button");
        button_input.id = "hacky-button-file-input";
        button_input.addEventListener('click', function(e) {
            e.preventDefault();

            // the fake event will be called on the button click
            document.getElementById("s2k-dnd-area").dispatchEvent(customDropEvent);
          });
        elem.appendChild(button_input); // put it into the DOM

        "#, Vec::new()
        ).await?;
    let dnd_area = driver.find(By::Id("s2k-dnd-area")).await?;
    if !dnd_area.is_displayed().await? {
        println!("Something's off with the dnd area...");
    }
    else {
        println!("Found dnd area, we are ready to send some files!");
    }
    let file_input_button = driver.find(By::Id("hacky-button-file-input")).await?;
    file_input_button.click().await?;
    driver.find(By::Css(".s2k-r2s-file-item")).await?;
    println!("Found at least one item in the dnd area");
    let add_to_library_label = driver.find(By::Id("s2k-r2s-add2lib")).await?;
    let add_to_library_checkbox = add_to_library_label.find(By::Css("input[type='checkbox']")).await?;
    if add_to_library_checkbox.prop("checked").await?.unwrap() != "true" {
        add_to_library_label.click().await?;
    }
    println!("Found the 'Add to your library' checkbox, and ensured that it is checked");
    if !debugging_mode {
        let start = std::time::Instant::now();
        let send_button = driver.find(By::Id("s2k-r2s-send-button")).await?;
        send_button.click().await?;
        println!("Found and clicked the send button");
        let mut uploading = true;
        println!("Waiting for files to upload...");
        while uploading {
            uploading = !dnd_area.is_displayed().await?;
            if start.elapsed().as_secs() as usize > files.clone().len() * file_timeout {
                println!("Waited for more than {} seconds per file and still appears that not everything was done. Giving up. (If your connection is slow, try increasing the --file-timeout argument)", file_timeout);
                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(3));
            print!(".");
        }
        if !uploading {
            println!("\nEverything uploaded successfully! :)");
        }
    }
    else {
        println!("DEBUGGING MODE: Press enter to close the browser window...");
        let mut _s = String::new();
        std::io::stdin().read_line(&mut _s)?;
    }
    driver.close_window().await?;
    if daemon {
        gd_daemon.kill()?;
    }
    Ok(())
}

/// * **Parameters:**
///
///     - username: The amazon username as str
///     - password: The amazon password as str
///     - f: A path to a file or directory to be uploaded as str
///     - ext: A file format extension to filter the files to be uploaded as str
///     - file_timeout: Seconds assigned to each file before uploading times out
///     - url: The amazon url to be used
///     - daemon: Whether to run the geckodriver daemon or not. If not, it will have to be run externally
///     - debugging_mode: For development purposes, enables GUI on the browser, does not send the files and prompts for user input to end the process
/// * **Description:** Send the _files_ to the kindle app and devices associated with the _username_ and _password_ amazon account
///
/// * **Notes:** It is recommended to use the Args::default or Args::new functions to ensure some sane defaults and easier setup. Please note that the _f_ str should be selected either from the file or directory field of the Args struct. _f_ is filtered based on the _ext_ parameter and the a vector of strings is constructed and passed to _send_files_to_kindle_
pub async fn send_to_kindle(username: &str, password: &str, f: &str, ext: &str, file_timeout: usize, url: &str, daemon: bool, debugging_mode: bool) -> WebDriverResult<()> {
    let mut files = Vec::<String>::new();
    let source = std::path::Path::new(f);
    if source.is_file() {
        if ext == "" || ext == source.extension().unwrap_or(std::ffi::OsStr::new("")) {
            files.push(String::from(f));
        }
    }
    else if source.is_dir() {
        for file in std::fs::read_dir(f).unwrap() {
            let file_path = file.unwrap().path();
            if file_path.ends_with(ext) {
                files.push(file_path.display().to_string());
            }
        }
    }
    send_files_to_kindle(username, password, files, file_timeout, url, daemon, debugging_mode).await
}

