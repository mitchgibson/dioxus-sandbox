use super::crawler;
use std::io::{self, Write};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crawl() {
        // Redirect stdout to capture printed output
        let mut output: Vec<u8> = Vec::new();
        let result;
        {
            let mut redirect = io::Cursor::new(&mut output);
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            // Call the function that prints file paths
            result = crawler::crawl("/Users/mitchdelachevrotiere/dev/knak/packages/builder/src");

            // Ensure all output is written
            handle.flush().unwrap();
        }
        // Perform assertions on the result
        assert!(!result.is_empty(), "The result should not be empty");
    }
}
