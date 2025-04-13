use {
    super::*,
    pretty_assertions::{assert_eq, StrComparison},
};

macro_rules! test {
  {
    name: $name:ident,
    $(justfile: $justfile:expr,)?
    $(args: ($($arg:tt),*),)?
    $(env: { $($env_key:literal : $env_value:literal,)* },)?
    $(stdin: $stdin:expr,)?
    $(stdout: $stdout:expr,)?
    $(stdout_regex: $stdout_regex:expr,)?
    $(stderr: $stderr:expr,)?
    $(stderr_regex: $stderr_regex:expr,)?
    $(status: $status:expr,)?
    $(shell: $shell:expr,)?
  } => {
    #[test]
    fn $name() {
      let test = crate::test::Test::new();

      $($(let test = test.arg($arg);)*)?
      $($(let test = test.env($env_key, $env_value);)*)?
      $(let test = test.justfile($justfile);)?
      $(let test = test.shell($shell);)?
      $(let test = test.status($status);)?
      $(let test = test.stderr($stderr);)?
      $(let test = test.stderr_regex($stderr_regex);)?
      $(let test = test.stdin($stdin);)?
      $(let test = test.stdout($stdout);)?
      $(let test = test.stdout_regex($stdout_regex);)?

      test.run();
    }
  }
}

pub struct Output {
    pub pid: u32,
    pub stdout: String,
    pub tempdir: TempDir,
}

#[must_use]
pub struct Test {
    pub args: Vec<String>,
    pub current_dir: PathBuf,
    pub env: BTreeMap<String, String>,
    pub justfile: Option<String>,
    pub shell: bool,
    pub status: i32,
    pub stderr: String,
    pub stderr_regex: Option<Regex>,
    pub stdin: String,
    pub stdout: String,
    pub stdout_regex: Option<Regex>,
    pub tempdir: TempDir,
    pub test_round_trip: bool,
    pub unindent_stdout: bool,
}

impl Test {
    pub fn new() -> Self {
        Self::with_tempdir(tempdir())
    }

    pub fn with_tempdir(tempdir: TempDir) -> Self {
        Self {
            args: Vec::new(),
            current_dir: PathBuf::new(),
            env: BTreeMap::new(),
            justfile: Some(String::new()),
            shell: true,
            status: EXIT_SUCCESS,
            stderr: String::new(),
            stderr_regex: None,
            stdin: String::new(),
            stdout: String::new(),
            stdout_regex: None,
            tempdir,
            test_round_trip: true,
            unindent_stdout: true,
        }
    }

    pub fn arg(mut self, val: &str) -> Self {
        self.args.push(val.to_owned());
        self
    }

    pub fn args<'a>(mut self, args: impl AsRef<[&'a str]>) -> Self {
        for arg in args.as_ref() {
            self = self.arg(arg);
        }
        self
    }

    pub fn create_dir(self, path: impl AsRef<Path>) -> Self {
        fs::create_dir_all(self.tempdir.path().join(path.as_ref())).unwrap();
        self
    }

    pub fn current_dir(mut self, path: impl AsRef<Path>) -> Self {
        path.as_ref().clone_into(&mut self.current_dir);
        self
    }

    pub fn env(mut self, key: &str, val: &str) -> Self {
        self.env.insert(key.to_string(), val.to_string());
        self
    }

    pub fn justfile(mut self, justfile: impl Into<String>) -> Self {
        self.justfile = Some(justfile.into());
        self
    }

    pub fn justfile_path(&self) -> PathBuf {
        self.tempdir.path().join("justfile")
    }

    #[cfg(unix)]
    #[track_caller]
    pub fn symlink(self, original: &str, link: &str) -> Self {
        std::os::unix::fs::symlink(
            self.tempdir.path().join(original),
            self.tempdir.path().join(link),
        )
        .unwrap();
        self
    }

    pub fn no_justfile(mut self) -> Self {
        self.justfile = None;
        self
    }

    pub fn shell(mut self, shell: bool) -> Self {
        self.shell = shell;
        self
    }

    pub fn status(mut self, exit_status: i32) -> Self {
        self.status = exit_status;
        self
    }

    pub fn stderr(mut self, stderr: impl Into<String>) -> Self {
        self.stderr = stderr.into();
        self
    }

    pub fn stderr_regex(mut self, stderr_regex: impl AsRef<str>) -> Self {
        self.stderr_regex = Some(Regex::new(&format!("^(?s){}$", stderr_regex.as_ref())).unwrap());
        self
    }

    pub fn stdin(mut self, stdin: impl Into<String>) -> Self {
        self.stdin = stdin.into();
        self
    }

    pub fn stdout(mut self, stdout: impl Into<String>) -> Self {
        self.stdout = stdout.into();
        self
    }

    pub fn stdout_regex(mut self, stdout_regex: impl AsRef<str>) -> Self {
        self.stdout_regex = Some(Regex::new(&format!("^{}$", stdout_regex.as_ref())).unwrap());
        self
    }

    #[allow(unused)]
    pub fn test_round_trip(mut self, test_round_trip: bool) -> Self {
        self.test_round_trip = test_round_trip;
        self
    }

    pub fn tree(self, mut tree: Tree) -> Self {
        tree.map(|_name, content| unindent(content));
        tree.instantiate(self.tempdir.path()).unwrap();
        self
    }

    pub fn unindent_stdout(mut self, unindent_stdout: bool) -> Self {
        self.unindent_stdout = unindent_stdout;
        self
    }

    pub fn write(self, path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Self {
        let path = self.tempdir.path().join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
        self
    }
}

impl Test {
    #[track_caller]
    pub fn run(self) -> Output {
        fn compare<T: PartialEq + Debug>(name: &str, have: T, want: T) -> bool {
            let equal = have == want;
            if !equal {
                eprintln!("Bad {name}: {}", Comparison::new(&have, &want));
            }
            equal
        }

        fn compare_string(name: &str, have: &str, want: &str) -> bool {
            let equal = have == want;
            if !equal {
                eprintln!("Bad {name}: {}", StrComparison::new(&have, &want));
            }
            equal
        }

        if let Some(justfile) = &self.justfile {
            let justfile = unindent(justfile);
            fs::write(self.justfile_path(), justfile).unwrap();
        }

        let stdout = if self.unindent_stdout {
            unindent(&self.stdout)
        } else {
            self.stdout.clone()
        };

        let stderr = unindent(&self.stderr);

        let mut command = Command::new(executable_path("just"));

        if self.shell {
            command.args(["--shell", "bash"]);
        }

        let mut child = command
            .args(&self.args)
            .envs(&self.env)
            .current_dir(self.tempdir.path().join(&self.current_dir))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("just invocation failed");

        let pid = child.id();

        {
            let mut stdin_handle = child.stdin.take().expect("failed to unwrap stdin handle");

            stdin_handle
                .write_all(self.stdin.as_bytes())
                .expect("failed to write stdin to just process");
        }

        let output = child
            .wait_with_output()
            .expect("failed to wait for just process");

        let output_stdout = str::from_utf8(&output.stdout).unwrap();
        let output_stderr = str::from_utf8(&output.stderr).unwrap();

        if let Some(ref stdout_regex) = self.stdout_regex {
            assert!(
                stdout_regex.is_match(output_stdout),
                "Stdout regex mismatch:\n{output_stdout:?}\n!~=\n/{stdout_regex:?}/",
            );
        }

        if let Some(ref stderr_regex) = self.stderr_regex {
            assert!(
                stderr_regex.is_match(output_stderr),
                "Stderr regex mismatch:\n{output_stderr:?}\n!~=\n/{stderr_regex:?}/",
            );
        }

        if !compare("status", output.status.code(), Some(self.status)) |
            (self.stdout_regex.is_none() && !compare_string("stdout", output_stdout, &stdout)) |
            (self.stderr_regex.is_none() && !compare_string("stderr", output_stderr, &stderr))
        {
            panic!("Output mismatch.");
        }

        if self.test_round_trip && self.status == EXIT_SUCCESS {
            self.round_trip();
        }

        Output {
            pid,
            stdout: output_stdout.into(),
            tempdir: self.tempdir,
        }
    }

    fn round_trip(&self) {
        println!("Reparsing...");

        let output = Command::new(executable_path("just"))
            .current_dir(self.tempdir.path())
            .arg("--dump")
            .envs(&self.env)
            .output()
            .expect("just invocation failed");

        assert!(
            output.status.success(),
            "dump failed: {} {:?}",
            output.status,
            output,
        );

        let dumped = String::from_utf8(output.stdout).unwrap();

        let reparsed_path = self.tempdir.path().join("reparsed.just");

        fs::write(&reparsed_path, &dumped).unwrap();

        let output = Command::new(executable_path("just"))
            .current_dir(self.tempdir.path())
            .arg("--justfile")
            .arg(&reparsed_path)
            .arg("--dump")
            .envs(&self.env)
            .output()
            .expect("just invocation failed");

        assert!(output.status.success(), "reparse failed: {}", output.status);

        let reparsed = String::from_utf8(output.stdout).unwrap();

        assert_eq!(reparsed, dumped, "reparse mismatch");
    }
}

pub fn assert_eval_eq(expression: &str, result: &str) {
    Test::new()
        .justfile(format!("x := {expression}"))
        .args(["--evaluate", "x"])
        .stdout(result)
        .unindent_stdout(false)
        .run();
}
