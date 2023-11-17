// We can use the RefCell<T> type for its internal mutability properties. It can allow a type edit itself using an internal 
// method while appearing as immutable to external types. This takes onwership of the value as well and the checks
// are done at runtime as opposed to at compile time like the others. 
// It has the benefit of allowing memory safe code that would have otherwise not been allowed to compile run at runtime


// We can use a typical example of Mock objects 

// Here’s the scenario we’ll test: we’ll create a library that tracks a value against a maximum value and 
// sends messages based on how close to the maximum value the current value is. This library could be used to 
// keep track of a user’s quota for the number of API calls they’re allowed to make, for example.
// Our library will only provide the functionality of tracking how close to the maximum a value is and what
// the messages should be at what times. Applications that use our library will be expected to provide the 
// mechanism for sending the messages: the application could put a message in the application, send an email, 
// send a text message, or something else. The library doesn’t need to know that detail. All it needs is 
// something that implements a trait we’ll provide called Messenger

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}