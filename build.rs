// use std::env;
// use std::fs;
// use std::path::Path;
// fn copy_src_dir_to_output_dir(src: &str, output: &str) -> std::io::Result<()> {
//     let src_path = Path::new(src);
//     let output_path = Path::new(output);
//
//     if !src_path.exists() || !src_path.is_dir() {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::NotFound,
//             "Source directory does not exist or is not a directory",
//         ));
//     }
//
//     fs::create_dir_all(output_path)?;
//
//     for entry in fs::read_dir(src_path)? {
//         let entry = entry?;
//         let file_type = entry.file_type()?;
//
//         let dest_path = output_path.join(entry.file_name());
//
//         if file_type.is_dir() {
//             copy_src_dir_to_output_dir(
//                 &entry.path().to_string_lossy(),
//                 &dest_path.to_string_lossy(),
//             )?;
//         } else {
//             fs::copy(entry.path(), dest_path)?;
//         }
//     }
//
//     Ok(())
// }
//
fn main() {
    // let output_binary_dir = env::var("OUT_DIR").unwrap();
    // let src_dir = env::current_dir().unwrap().join("compiler_backend");
    // println!("Src Dir: {}", src_dir.to_string_lossy());
    // println!("Output Binary Dir: {}", output_binary_dir);
    // copy_src_dir_to_output_dir(&src_dir.to_string_lossy(), &output_binary_dir).unwrap();
}
