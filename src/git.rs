pub mod git {

    use std::process::Command;

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
        let underscores = str::replace(title, " ", "_");
        // Get separately the first letter of the title and the rest of the sentence
        let first_word = &underscores[0..1];
        let other_words = &underscores[1..underscores.len()];
        // Combine the first letter + the rest of the sentence in lowercase
        format!("{}{}", first_word, other_words.to_lowercase())
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
}
