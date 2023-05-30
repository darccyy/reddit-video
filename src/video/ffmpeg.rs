use std::{ffi::OsStr, process};

pub struct FFMpegCommand(process::Command);

impl FFMpegCommand {
    /// Create ffmpeg command
    pub fn new(overwrite: bool) -> Self {
        let mut cmd = process::Command::new("ffmpeg");
        // cmd.args(["-loglevel", "warning"]);
        if overwrite {
            cmd.arg("-y");
        }
        Self(cmd)
    }

    /// Run ffmpeg command
    pub fn run(self) {
        let mut cmd = self.0;

        let result = cmd.output().expect("Failed to run ffmpeg command");

        if !result.status.success() {
            eprintln!("FFMPEG Error");
            eprintln!("{}", String::from_utf8_lossy(&result.stderr));
            std::process::exit(1);
        } else {
            println!("\x1b[1mSuccess!\x1b[0m");
        }
    }

    /// Print full command to stdout
    pub fn show_command(&self) {
        println!(
            "ffmpeg {}",
            self.0
                .get_args()
                .map(|x| x.to_string_lossy())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

impl FFMpegCommand {
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.0.arg(arg);
        self
    }
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.0.args(args);
        self
    }
}
