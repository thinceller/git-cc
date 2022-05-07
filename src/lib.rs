use std::process::{self, Command};

use dialoguer::{theme::ColorfulTheme, Editor, FuzzySelect, Input};

#[derive(Debug)]
struct Message {
    commit_type: String,
    scope: String,
    description: String,
    body: String,
    breaking_change: String,
}

impl Message {
    fn format(self) -> String {
        let mut message = String::new();
        message.push_str(&self.commit_type);
        if self.scope.len() != 0 {
            message.push_str(format!("({})", self.scope).as_str());
        }
        message.push_str(": ");
        message.push_str(&self.description);
        if self.body.len() != 0 {
            message.push_str("\n\n");
            message.push_str(&self.body);
        }
        if self.breaking_change.len() != 0 {
            message.push_str("\n\n");
            message.push_str(format!("BREAKING CHANGE: {}", self.breaking_change).as_str());
        }
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_type_and_description() {
        let message = Message {
            commit_type: String::from("feat"),
            scope: String::from(""),
            description: String::from("add api"),
            body: String::from(""),
            breaking_change: String::from(""),
        };
        assert_eq!(message.format(), "feat: add api")
    }

    #[test]
    fn format_with_scope() {
        let message = Message {
            commit_type: String::from("feat"),
            scope: String::from("cli"),
            description: String::from("add api"),
            body: String::from(""),
            breaking_change: String::from(""),
        };
        assert_eq!(message.format(), "feat(cli): add api")
    }

    #[test]
    fn format_with_body() {
        let message = Message {
            commit_type: String::from("feat"),
            scope: String::from(""),
            description: String::from("add api"),
            body: String::from("This is a body"),
            breaking_change: String::from(""),
        };
        assert_eq!(
            message.format(),
            r#"feat: add api

This is a body"#
        )
    }

    #[test]
    fn format_with_scope_and_body() {
        let message = Message {
            commit_type: String::from("feat"),
            scope: String::from("cli"),
            description: String::from("add api"),
            body: String::from("This is a body"),
            breaking_change: String::from(""),
        };
        assert_eq!(
            message.format(),
            r#"feat(cli): add api

This is a body"#
        )
    }

    #[test]
    fn format_with_scope_and_body_and_breaking_change() {
        let message = Message {
            commit_type: String::from("feat"),
            scope: String::from("cli"),
            description: String::from("add api"),
            body: String::from("This is a body"),
            breaking_change: String::from("remove api"),
        };
        assert_eq!(
            message.format(),
            r#"feat(cli): add api

This is a body

BREAKING CHANGE: remove api"#
        )
    }
}

fn select_commit_type() -> String {
    // conventional commit type
    let selections = vec![
        "feat", "fix", "build", "chore", "ci", "docs", "perf", "refactor", "revert", "style",
        "test",
    ];

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of change that you're committing:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selections[selection].to_string()
}

fn write_scope() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write the commit scope (optional):")
        .allow_empty(true)
        .interact_text()
        .unwrap()
}

fn write_description() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write a short description:")
        .interact_text()
        .unwrap()
}

fn write_body() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write a detail description (optional):")
        .allow_empty(true)
        .interact_text()
        .unwrap()
}

fn write_breaking_change() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write a breaking change (optional):")
        .allow_empty(true)
        .interact_text()
        .unwrap()
}

fn create_message() -> Message {
    let commit_type = select_commit_type();
    let scope = write_scope();
    let description = write_description();
    let body = write_body();
    let breaking_change = write_breaking_change();
    let message = Message {
        commit_type,
        scope,
        description,
        body,
        breaking_change,
    };
    message
}

fn commit(message: &str) {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(format!("{}", message))
        .status()
        .expect("commit failed");
}

pub fn run() {
    let message = create_message();
    let formatted_message = message.format();

    let selections = vec!["Commit with it", "Continue with editor", "Cancel"];
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Are you OK this message?\n{}", formatted_message))
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    match selection {
        // Commit with it
        0 => {
            commit(&formatted_message);
        }
        // Continue with editor
        1 => {
            if let Some(rv) = Editor::new().edit(&formatted_message).unwrap() {
                commit(&rv);
            } else {
                process::exit(1);
            }
        }
        // Cancel
        2 => {
            process::exit(1);
        }
        _ => unreachable!(),
    }
}
