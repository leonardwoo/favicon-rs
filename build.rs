
#[cfg(windows)]
extern crate winres;

fn main() {
    #[cfg(windows)]
    {
        // https://docs.rs/winres/0.1.0/winres/struct.WindowsResource.html
        let mut res = winres::WindowsResource::new();
        // res.set_icon("resources/favicon.ico");
        res.set_language(0x0409);
        res.set_manifest_file("resources/favicon.exe.manifest");
        res.compile().unwrap();
    }
}