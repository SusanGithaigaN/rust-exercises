// TOPIC 12 — Traits and Generics
// EXERCISE TYPE: Revisit
// =============================================
// Do this a few days after completing the topic. Write from scratch.
//
// TASK: Build a trait-based notification system.
//
// trait Notifiable {
//     fn send(&self, message: &str);
//     fn channel_name(&self) -> &str;
// }
//
// struct EmailNotifier { address: String }
// struct SlackNotifier { channel: String }
//
// Implement Notifiable for both:
//   EmailNotifier: send prints "Email to {address}: {message}"
//                  channel_name returns the address
//   SlackNotifier: send prints "Slack #{channel}: {message}"
//                  channel_name returns the channel
//
// fn broadcast<T: Notifiable>(notifiers: &[T], message: &str)
//   Calls send on every notifier in the slice.
//
// fn notify_all(notifiers: &[&dyn Notifiable], message: &str)
//   Same but uses trait objects (dynamic dispatch).
//   This allows mixing different notifier types in one slice.
//
// Expected output:
//   Email to alice@example.com: server down
//   Slack #ops: server down
//
// Run: cargo run --bin revisit

fn main() {
    // TODO: implement all traits, structs, and functions above main, then use here
}
