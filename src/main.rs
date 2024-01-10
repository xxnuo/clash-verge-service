mod service;

#[cfg(windows)]
fn main() -> windows_service::Result<()> {
    service::main()
}

#[cfg(not(windows))]
fn main() -> anyhow::Result<()> {
    service::main()
}
