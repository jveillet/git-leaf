pub mod git {

    use std::process::Command;
    use regex::Regex;

    /// Test if the git comand line tool is present on the system
    ///
    /// # Result
    ///
    /// * `bool` - true if the command succeeded
    ///
    pub fn is_present() -> bool {
        let output = Command::new("git")
            .arg("--version")
            .output()
            .expect("git was not found on the system");

        output.status.success()
    }
    /// Create a named git branch
    ///
    /// # Arguments
    ///
    /// * `branch_name` - A reference to a strign containing the full branch name that need to be created
    ///
    /// # Panics
    ///
    /// Panics if the git command line tool failes to do a checkout
    ///
    pub fn create_branch(branch_name: &str) {
        Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(branch_name)
            .output()
            .expect("the git checkout command failed");
    }
    /// Format the issue title
    ///
    /// # Arguments
    ///
    /// * `title` - A reference to a string containing the title sentence (with whitespaces)
    ///
    /// # Result
    ///
    /// * `String` - String containing the formatted title
    /// - Underscores instead of whitespaces
    /// - Only the first word is capitalized
    /// - All the other words in the title are lowercase
    ///
    /// # Examples
    ///
    /// let title = format_title("My Awesome Title");
    /// println!("{}", title);
    /// => My_awesome_title
    ///
    pub fn format_title(title: &str) -> String {
        // Replace all whitespaces with underscores
        let underscores = replace_white_space(&title, "_");
        // Remove special characters
        let underscores = remove_special_characters(&underscores);
        // Get separately the first letter of the title and the rest of the sentence
        let first_word = &underscores[0..1];
        let other_words = &underscores[1..underscores.len()];
        // Combine the first letter + the rest of the sentence in lowercase
        format!("{}{}", first_word, other_words.to_lowercase())
    }

    /// Replace white space in a string
    ///
    /// # Arguments
    ///
    /// * `text` - A string containing the text to format
    /// * `sep` - A  string containing the separator to use to replace white space
    ///
    /// # Result
    ///
    /// * `String` - String containing the text with white spaces replaced by a character
    ///
    /// # Examples
    ///
    /// let text = replace_white_space("My Awesome Title", "-");
    /// println!("{}", text);
    /// => My-Awesome-Title
    ///
    pub fn replace_white_space(text: &str, sep: &str) -> String {
        str::replace(text, " ", sep)
    }

    /// Remove special characters in a String
    ///
    /// # Arguments
    ///
    /// * `text` - A string containing the text to format
    ///
    /// # Result
    ///
    /// * `String` - String containing the text whit special characters replaced by white spaces
    /// White spaces are also trimmed from the final result
    ///
    /// # Examples
    ///
    /// let text = remove_special_characters("My Awesome Title !!");
    /// println!("{}", text;
    /// => My Awesome Title
    ///
    pub fn remove_special_characters(text: &str) -> String {
        let regex = Regex::new(r"[^\w\d\s]").unwrap();
        let result = regex.replace_all(&text, "").to_string();
        result.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_git_present() {
        assert_eq!(git::is_present(), true);
    }

    #[test]
    fn should_format_title() {
        let sentence = String::from("My Awesome Issue Title");
        let title = git::format_title(&sentence);
        assert_eq!(title, "My_awesome_issue_title");
    }

    #[test]
    fn should_escape_special_characters() {
        let sentence = String::from("My awesome text @ ! #");
        let title = git::remove_special_characters(&sentence);
        assert_eq!(title, "My awesome text");
    }

    #[test]
    fn should_replace_whitespace() {
        let sentence = String::from("My awesome text");
        let result = git::replace_white_space(&sentence, "-");
        assert_eq!(result, "My-awesome-text");
    }
}
