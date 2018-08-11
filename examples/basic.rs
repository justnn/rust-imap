extern crate imap;
extern crate native_tls;

use imap::client::UnauthenticatedClient;
use native_tls::TlsConnector;

// To connect to the gmail IMAP server with this you will need to allow unsecure apps access.
// See: https://support.google.com/accounts/answer/6010255?hl=en
// Look at the gmail_oauth2.rs example on how to connect to a gmail server securely.
fn main() {
    let domain = "imap.gmail.com";
    let port = 993;
    let socket_addr = (domain, port);
    let ssl_connector = TlsConnector::builder().build().unwrap();
    let unauth_client = UnauthenticatedClient::secure_connect(socket_addr, domain, &ssl_connector).unwrap();

    let mut imap_socket = match unauth_client.login("username", "password") {
        Ok(c) => c,
        Err((e, _unauth_client)) => {
            println!("failed to login: {}", e);
            return;
        },
    };

    match imap_socket.capabilities() {
        Ok(capabilities) => for capability in capabilities.iter() {
            println!("{}", capability);
        },
        Err(e) => println!("Error parsing capability: {}", e),
    };

    match imap_socket.select("INBOX") {
        Ok(mailbox) => {
            println!("{}", mailbox);
        }
        Err(e) => println!("Error selecting INBOX: {}", e),
    };

    match imap_socket.fetch("2", "body[text]") {
        Ok(msgs) => for msg in &msgs {
            print!("{:?}", msg);
        },
        Err(e) => println!("Error Fetching email 2: {}", e),
    };

    imap_socket.logout().unwrap();
}
