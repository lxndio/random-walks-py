use crate::dp::DynamicProgram;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};
use std::path::Path;
use std::{fs, io};

/// Allow loading and saving data for a dynamic program.
pub trait Store {
    /// Save a dynamic program to files.
    fn to_files(&self, path: String) -> io::Result<()>;

    /// Load a precomputed dynamic program from files.
    fn from_files(path: String, time_limit: usize) -> io::Result<DynamicProgram>;
}

// impl Store for DynamicProgram {
//     fn to_files(&self, path: String) -> io::Result<()> {
//         let path = Path::new(&path);
//         let (limit_neg, limit_pos) = self.limits();
//
//         // Create directory if it does not exist
//         if !path.exists() {
//             fs::create_dir_all(path)?;
//         }
//
//         // Save data to files
//         for t in 0..=limit_pos as usize {
//             let file_path = path.join(format!("counts_{}.txt", t));
//             let file = File::create(file_path)?;
//             let mut writer = BufWriter::new(file);
//
//             for y in limit_neg..=limit_pos {
//                 for x in limit_neg..=limit_pos {
//                     writer.write(self.at(x, y, t).to_string().as_bytes())?;
//
//                     if x != limit_pos {
//                         writer.write(" ".as_bytes())?;
//                     }
//                 }
//
//                 writer.write("\n".as_bytes())?;
//             }
//
//             writer.flush()?;
//         }
//
//         Ok(())
//     }
//
//     fn from_files(path: String, time_limit: usize) -> io::Result<DynamicProgram> {
//         let path = Path::new(&path);
//
//         // Check if all necessary files exist
//         for t in 0..=time_limit {
//             let file_path = path.join(format!("counts_{}.txt", t));
//
//             if !file_path.exists() {
//                 return Err(Error::new(
//                     ErrorKind::NotFound,
//                     format!("File {} not found.", &file_path.display()),
//                 ));
//             }
//         }
//
//         // Load data from files
//         let mut dp = DynamicProgram::new(time_limit, SimpleStepper);
//
//         for t in 0..=time_limit {
//             let file_path = path.join(format!("counts_{}.txt", t));
//             let file = File::open(&file_path)?;
//             let reader = BufReader::new(file);
//
//             for (y, line) in reader.lines().enumerate() {
//                 let line = line?;
//                 let counts = line.split_whitespace();
//
//                 for (x, number) in counts.enumerate() {
//                     dp.set(
//                         x as isize - time_limit as isize,
//                         y as isize - time_limit as isize,
//                         t,
//                         number.parse().expect(&format!(
//                             "File {} contains non-parsable data.",
//                             &file_path.display()
//                         )),
//                     );
//                 }
//             }
//         }
//
//         Ok(dp)
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::dp::problems::Problems;
//     use crate::dp::store::Store;
//     use crate::dp::DynamicProgram;
//     use crate::stepper::simple::SimpleStepper;
//     use std::fs;
//
//     #[test]
//     fn test_to_files() {
//         let mut dp = DynamicProgram::new(10, SimpleStepper);
//         dp.count_paths();
//
//         assert!(dp.to_files(String::from("test_to_files/")).is_ok());
//
//         fs::remove_dir_all("test_to_files/").expect("Could not remove directory after testing.");
//     }
//
//     #[test]
//     fn test_from_files() {
//         let mut dp = DynamicProgram::new(10, SimpleStepper);
//         dp.count_paths();
//         dp.to_files(String::from("test_from_files/"))
//             .expect("Could not save files.");
//
//         let dp_loaded = DynamicProgram::from_files(String::from("test_from_files/"), 10)
//             .expect("Could not load files.");
//
//         assert_eq!(dp, dp_loaded);
//
//         fs::remove_dir_all("test_from_files/").expect("Could not remove directory after testing.");
//     }
// }
