//2
/* Command::new("cd")
.arg("//Users/matthew/dev/projects/script-homebrew/target/release/")
.spawn()
.expect("failed to execute process"); */

//Option 2
// use flate2::write::GzEncoder;
// use flate2::Compression;
// use std::fs::File;

/* let tar_gz = File::create("zarchive.tar.gz")?;
let enc = GzEncoder::new(tar_gz, Compression::default());
let mut tar = tar::Builder::new(enc);
tar.append_file("gen.sh", &mut File::open("/Users/matthew/zz/homebrew/gen.sh").unwrap()); */

/* "class Temp < Formula
desc
homepage
url \"/Users/matthew/dev/projects/script-homebrew/target/release/script-homebrew.tar.gz\"
version
sha256
license

def install
    bin.install \"temp\"
end

end
" */

    
                        // //"Formula"
                        // else if alphabetic == Some('F') && next_char == &'o' {
                        //     // let o = self.expr.next();
                        //     // let r = self.expr.next();
                        //     // let m = self.expr.next();
                        //     // let u = self.expr.next();
                        //     // let l = self.expr.next();
                        //     // let a = self.expr.next();
                        //     // if o == Some('o') && r == Some('r') && m == Some('m') {
                        //     //     // println!("l {:?}",l);
                        //     //     // println!("{:?}",a);
                        //     //     // println!("{:?}",s);
                        //     //     // println!("s {:?}",s);
                        //     //     return Some(TokenKind::Object("Formula".to_string()));
                        //     // }
                        // } else {
                        //     // let catcher = String::from("");
                        //     // while let self.expr.next() != Some(' ') {
                        //     //     catcher.push()
                        //     // }
                        // }
