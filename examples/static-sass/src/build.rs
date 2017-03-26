extern crate rsass;
extern crate ructe;

use rsass::{OutputStyle, compile_scss};
use ructe::{StaticFiles, compile_templates};
use std::env;
use std::fs::{File, create_dir_all};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let base_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut statics = StaticFiles::new(&out_dir).unwrap();
    statics.add_files(&base_dir.join("static")).unwrap();
    add_static_sass(&mut statics, "style.scss".as_ref()).unwrap();

    let template_dir = base_dir.join("templates");
    compile_templates(&template_dir, &out_dir).expect("templates");
}

fn add_static_sass(statics: &mut StaticFiles, src: &Path) -> io::Result<()> {
    // TODO Find any referenced files!
    println!("cargo:rerun-if-changed={}", src.to_string_lossy());

    let mut scss_buf = Vec::new();
    // Define variables for all previously known static files.
    for (x, y) in statics.get_names() {
        writeln!(scss_buf, "${}: {:?};", x, y)?;
    }
    let mut f = File::open(src)?;
    f.read_to_end(&mut scss_buf)?;

    let css = compile_scss(&scss_buf, OutputStyle::Compressed).unwrap();

    // TODO Writing the css to an actual file should not be needed.
    let css_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("tmpcss");
    create_dir_all(&css_dir)?;
    let css_file = css_dir.join("style.css");
    File::create(&css_file).and_then(|mut f| f.write(&css))?;

    statics.add_file(&css_file)
}
