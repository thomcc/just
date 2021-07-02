fn main() {
  if let Err(error) = just::run(std::env::args_os()) {
    std::process::exit(error.code());
  }
}
