//! The state pattern for a BlogPost 

// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }

//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }

//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve())
//         }
//     }

// }

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }

// struct Draft {}

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

// }

// struct PendingReview {}

// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
// }


// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }


// ENCODING STATES AND BEHAVIOUR AS TYPE Takes advantage of the Rust type system and ownership rules to give 
// a more idiomatic solution compared to the STATE PATTERN

pub struct Post {
    content: String
}



impl Post {
    pub fn new() -> DraftPost {
        // We want every post to start out as a DraftPost 
        DraftPost {
            content: String::new(),
        }
    }

    // Only approved posts can have the content method because only them have content that can be viewed.
    // the others do not have the content method because the content field is private.
    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String
}

impl DraftPost {
    // Draft posts should be able to accept/receive text to add to their content field
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        // this takes ownership of self because once we request to rewiev a post we want to have the 
        // PendingReviewPost and not be able to use the DraftPost anymore
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        // this takes ownership of self because once we approve a pending post we want to have a published post
        // and not be able to use the PendingReviewPost anymore.
        Post {
            content: self.content,
        }
    }
}