use super::spinner::SpinnerState;
use crate::features::commit::types::{GeneratedMessage, format_commit_message};
use crate::features::rebase::{RebaseAction, RebaseCommit};

use tui_textarea::TextArea;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Mode {
    Normal,
    EditingMessage,
    EditingInstructions,
    Generating,
    Help,
    RebaseList,
    RebaseEdit,
}

pub struct TuiState {
    pub messages: Vec<GeneratedMessage>,
    pub current_index: usize,
    pub custom_instructions: String,
    pub status: String,
    pub mode: Mode,
    pub message_textarea: TextArea<'static>,
    pub instructions_textarea: TextArea<'static>,
    pub rebase_textarea: TextArea<'static>,
    pub spinner: Option<SpinnerState>,
    pub dirty: bool,
    pub last_spinner_update: std::time::Instant,
    pub instructions_visible: bool,
    pub nav_bar_visible: bool,
    pub rebase_commits: Vec<RebaseCommit>,
    pub rebase_current_index: usize,
}

impl TuiState {
    pub fn new(initial_messages: Vec<GeneratedMessage>, custom_instructions: String) -> Self {
        let mut message_textarea = TextArea::default();
        let messages = if initial_messages.is_empty() {
            vec![GeneratedMessage {
                emoji: None,
                title: String::new(),
                message: String::new(),
            }]
        } else {
            initial_messages
        };
        if let Some(first_message) = messages.first() {
            message_textarea.insert_str(format_commit_message(first_message));
        }

        let mut instructions_textarea = TextArea::default();
        instructions_textarea.insert_str(&custom_instructions);

        let rebase_textarea = TextArea::default();

        Self {
            messages,
            current_index: 0,
            custom_instructions,
            status: "Press '?': help | 'Esc': exit".to_string(),
            mode: Mode::Normal,
            message_textarea,
            instructions_textarea,
            rebase_textarea,
            spinner: None,
            dirty: true,
            last_spinner_update: std::time::Instant::now(),
            instructions_visible: false,
            nav_bar_visible: true,
            rebase_commits: vec![],
            rebase_current_index: 0,
        }
    }

    pub fn set_status(&mut self, new_status: String) {
        self.status = new_status;
        self.spinner = None;
        self.dirty = true;
    }

    pub fn update_message_textarea(&mut self) {
        let current_message = &self.messages[self.current_index];
        let message_content = format!(
            "{}\n\n{}",
            current_message.title,
            current_message.message.trim()
        );

        let mut new_textarea = TextArea::default();
        new_textarea.insert_str(&message_content);
        self.message_textarea = new_textarea;
        self.dirty = true;
    }

    pub fn set_rebase_commits(&mut self, commits: Vec<RebaseCommit>) {
        self.rebase_commits = commits;
        self.rebase_current_index = 0;
        self.dirty = true;
    }

    pub fn next_rebase_commit(&mut self) {
        if self.rebase_current_index < self.rebase_commits.len().saturating_sub(1) {
            self.rebase_current_index += 1;
            self.dirty = true;
        }
    }

    pub fn prev_rebase_commit(&mut self) {
        if self.rebase_current_index > 0 {
            self.rebase_current_index = self.rebase_current_index.saturating_sub(1);
            self.dirty = true;
        }
    }

    pub fn toggle_rebase_action(&mut self) {
        if let Some(commit) = self.rebase_commits.get_mut(self.rebase_current_index) {
            commit.suggested_action = match commit.suggested_action {
                RebaseAction::Pick => RebaseAction::Reword,
                RebaseAction::Reword => RebaseAction::Squash,
                RebaseAction::Squash => RebaseAction::Fixup,
                RebaseAction::Fixup => RebaseAction::Drop,
                RebaseAction::Drop => RebaseAction::Pick,
                RebaseAction::Edit => RebaseAction::Pick,
            };
            self.dirty = true;
        }
    }

    pub fn update_rebase_textarea(&mut self) {
        if let Some(commit) = self.rebase_commits.get(self.rebase_current_index) {
            let mut new_textarea = TextArea::default();
            new_textarea.insert_str(&commit.message);
            self.rebase_textarea = new_textarea;
            self.dirty = true;
        }
    }
}
