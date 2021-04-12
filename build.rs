fn main() {
    #[cfg(windows)]
    build_winres().expect("failed to compile the executable description and shell icon");
}

#[cfg(windows)]
fn build_winres() -> std::io::Result<()> {
    use winres::*;

    println!("cargo:rerun-if-changed=icon/icon.ico");
    let mut res = WindowsResource::new();
    res.set_icon("icon/icon.ico");
    res.compile()?;
    Ok(())
}
