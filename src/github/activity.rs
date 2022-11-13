use crate::github::{client::Github, notifications::Notifications};

pub struct Activity {
    github: Github,
}

impl Activity {
    pub fn new(github: Github) -> Self {
        Self { github }
    }

    pub fn notifications(&self) -> Notifications {
        Notifications::new(self.github.clone())
    }
}
