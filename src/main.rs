fn main() {
  if let Err(code) = pub_just::run(std::env::args_os()) {
    std::process::exit(code);
  }
}
