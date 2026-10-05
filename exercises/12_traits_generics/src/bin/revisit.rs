// Read ../../README.md before starting.

trait Notifiable {
    fn send(&self, message: &str);
    fn channel_name(&self) -> &str;
}

struct EmailNotifier {
    address: String,
}
struct SlackNotifier {
    channel: String,
}

impl Notifiable for EmailNotifier {
    fn send(&self, message: &str) {
        todo!()
    }
    fn channel_name(&self) -> &str {
        todo!()
    }
}

impl Notifiable for SlackNotifier {
    fn send(&self, message: &str) {
        todo!()
    }
    fn channel_name(&self) -> &str {
        todo!()
    }
}

fn broadcast<T: Notifiable>(notifiers: &[T], message: &str) {
    todo!()
}

fn notify_all(notifiers: &[&dyn Notifiable], message: &str) {
    todo!()
}

fn main() {
    let email = EmailNotifier {
        address: String::from("alice@example.com"),
    };
    let slack = SlackNotifier {
        channel: String::from("ops"),
    };

    email.send("server down");
    slack.send("server down");

    let emails = vec![EmailNotifier {
        address: String::from("a@x.com"),
    }];
    broadcast(&emails, "ping");

    let all: Vec<&dyn Notifiable> = vec![&email, &slack];
    notify_all(&all, "restart");
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
