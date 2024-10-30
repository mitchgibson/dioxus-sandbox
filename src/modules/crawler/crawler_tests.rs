use super::crawler;
use std::io::{self, Write};
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_crawl() {
        // Redirect stdout to capture printed output
        let mut output: Vec<u8> = Vec::new();
        let nodes;
        {
            let mut redirect = io::Cursor::new(&mut output);
            let stdout = io::stdout();
            let mut handle = stdout.lock();

            // Call the function that prints file paths
            nodes = crawler::crawl("/Users/mitchdelachevrotiere/dev/knak/packages/builder/src");

            // nodes is a HashMap<String, Node>, print it
            writeln!(handle, "{:?}", nodes).unwrap();

            // Ensure all output is written
            handle.flush().unwrap();

        }

        let k_form_select = nodes.get("k-form-select").unwrap();

        assert!(!nodes.is_empty(), "The nodes should not be empty");
        assert!(k_form_select.locations.len() == 9, "The locations of k-form-select should have 9 items");
        
    }

    #[test]
    fn test_get_all_file_paths() {
        let file_paths = crawler::get_all_file_paths("/Users/mitchdelachevrotiere/dev/knak/packages/builder/src");
        assert!(file_paths.len() > 0, "There should be more than 0 file paths");
        assert!(
            file_paths.iter().any(|path| path == Path::new("/Users/mitchdelachevrotiere/dev/knak/packages/builder/src/components/controls/attributes/DividerAttributes.vue")),
            "The file paths should contain DividerAttributes.vue"
        );
    }
}
