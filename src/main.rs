trait Notifier {
    fn send(&self, message: &str);
}

struct EmailNotifier;

impl Notifier for EmailNotifier {
    fn send(&self, message: &str) {
        println!("Sending email: {}", message);
    }
}

struct SmsNotifier {
    notifier: Box<dyn Notifier>,
}

impl SmsNotifier {
    fn new(notifier: Box<dyn Notifier>) -> Self {
        Self { notifier }
    }
}

impl Notifier for SmsNotifier {
    fn send(&self, message: &str) {
        println!("Sending SMS: {}", message);
        self.notifier.send(message);
    }
}

struct SlackNotifierDecorator {
    notifier: Box<dyn Notifier>,
}

impl SlackNotifierDecorator {
    fn new(notifier: Box<dyn Notifier>) -> Self {
        Self { notifier }
    }
}

impl Notifier for SlackNotifierDecorator {
    fn send(&self, message: &str) {
        println!("Sending Slack: {}", message);
        self.notifier.send(message);
    }
}

fn main() {
    let email_notify = Box::new(EmailNotifier);
    email_notify.send("Hello, world!");

    let sms_notify = Box::new(SmsNotifier::new(email_notify));
    sms_notify.send("Hello, world!");

    let slack_notify = Box::new(SlackNotifierDecorator::new(sms_notify));
    slack_notify.send("Hello, world!");
}
