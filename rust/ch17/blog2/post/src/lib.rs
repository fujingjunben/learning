pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_draft() {
        let mut post = Post::new();
        post.add_text("hello, world");
        assert_eq!("", post.content());
    }

    #[test]
    fn test_review() {
        let mut post = Post::new();
        post.add_text("hello, world");
        post.request_review();
        assert_eq!("", post.content());
    }

    #[test]
    fn test_publish() {
        let mut post = Post::new();
        post.add_text("hello, world");
        post.request_review();
        post.approve();
        assert_eq!("hello, world", post.content());
    }
}
