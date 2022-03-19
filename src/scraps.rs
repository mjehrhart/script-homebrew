    //*********************************************************************************************************************************/
    // sandbox

    /* //1 checked
    let cargo = {
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg("/Users/matthew/dev/projects/script-homebrew/Cargo.toml")
            .output()
            .expect("failed to execute process")
    };
    let hash = cargo.stdout;
    let value = std::str::from_utf8(&hash).unwrap();
    let hash_value = value.to_owned();
    println!("cargo::{:?}", hash_value);

    //2 checked
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::fs::File;

    let tar_gz = File::create("script-homebrew.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_file(
        "script-homebrew",
        &mut File::open(
            "/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew",
        )
        .unwrap(),
    );

    //3
    let shasum = {
        Command::new("shasum")
            .arg("-a")
            .arg("256")
            .arg(
                "/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew.tar.gz",
            )
            .output()
            .expect("failed to execute process")
    };

    let hash = shasum.stdout;
    let value = std::str::from_utf8(&hash).unwrap();
    let hash_value = value.to_owned();
    let x = hash_value.split_once(' ').unwrap();

    println!("CHECKED shasum::{:?}", x.0); */

    //*********************************************************************************************************************************/

/* 
/// Example
/// ```rust
/// # main() -> Result<(), std::io::Error> {
/// let catcher = "!a \'crazy\' dog is 2.5 \"delete\" a #20 cat 65 is not".to_string();
/// formula::brew::ruby::Formula_Rb::read_string(catcher);
/// ```
/// or
/// ```
/// let path = "/Users/matthew/dev/projects/script-homebrew/temp.rb";
/// let this = formula::brew::ruby::Formula_Rb::read_ruby_template(path);
/// match this {
///     Ok(_) => {}
///     Err(_) => {}
/// }
/// ```
///
///  */