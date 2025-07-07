use std::fmt::Display;

use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, Input};

pub fn select<T: Display>(prompt: &str, options: &[T]) -> usize {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .max_length(5_usize)
        .default(0)
        .items(options)
        .interact()
        .unwrap();

    selection
}

pub fn basic_prompt(query: &str) -> String {
    return Input::with_theme(&ColorfulTheme::default())
        .with_prompt(query)
        .interact_text()
        .unwrap();
}

pub fn confirm_prompt(query: &str) -> bool {
    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(query)
        .interact()
        .unwrap();

    confirmed
}
