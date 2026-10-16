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

// Print "Email to <address>: <message>". channel_name returns the address.
impl Notifiable for EmailNotifier {
    fn send(&self, message: &str) {
        todo!()
    }
    fn channel_name(&self) -> &str {
        todo!()
    }
}

// Print "Slack #<channel>: <message>". channel_name returns the channel name.
impl Notifiable for SlackNotifier {
    fn send(&self, message: &str) {
        todo!()
    }
    fn channel_name(&self) -> &str {
        todo!()
    }
}

// Call send on every item in the slice.
fn broadcast<T: Notifiable>(notifiers: &[T], message: &str) {
    todo!()
}

// Same as broadcast but works with a mix of notifier types via trait objects.
fn notify_all(notifiers: &[&dyn Notifiable], message: &str) {
    todo!()
}

fn main() {
    let email = EmailNotifier { address: String::from("alice@example.com") };
    let slack = SlackNotifier { channel: String::from("ops") };

    email.send("server down"); // Email to alice@example.com: server down
    slack.send("server down"); // Slack #ops: server down

    let emails = vec![EmailNotifier { address: String::from("a@x.com") }];
    broadcast(&emails, "ping"); // Email to a@x.com: ping

    let all: Vec<&dyn Notifiable> = vec![&email, &slack];
    notify_all(&all, "restart"); // Email to alice@example.com: restart  then  Slack #ops: restart
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/revisit.rs"));
