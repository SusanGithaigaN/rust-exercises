#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_channels() {
        let e = EmailNotifier {
            address: "a@x.com".into(),
        };
        let s = SlackNotifier {
            channel: "ops".into(),
        };
        assert_eq!(e.channel_name(), "email");
        assert_eq!(s.channel_name(), "slack");
    }

    #[test]
    fn check_send() {
        EmailNotifier {
            address: "a@x.com".into(),
        }
        .send("ping");
        SlackNotifier {
            channel: "ops".into(),
        }
        .send("ping");
    }

    #[test]
    fn check_broadcast() {
        broadcast(
            &[EmailNotifier {
                address: "a@x.com".into(),
            }],
            "ping",
        );
        broadcast::<EmailNotifier>(&[], "ping");
    }

    #[test]
    fn check_notify_all() {
        let e = EmailNotifier {
            address: "a@x.com".into(),
        };
        let s = SlackNotifier {
            channel: "ops".into(),
        };
        notify_all(&[&e, &s], "restart");
        notify_all(&[], "restart");
    }
}
