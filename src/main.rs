use mail::email_service::send_email;

static SENDER: &str = "pawanbisht62@gmail.com";
static RECEIVER: &str = "pawan.bisht@knoldus.in";

fn main() {
    println!("{}", send_email(SENDER, RECEIVER));
}

