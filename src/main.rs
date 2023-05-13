use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let remote_addr = "192.168.0.100:5060";

    let local_addr = "0.0.0.0";
    let socket = UdpSocket::bind(local_addr)?;

    let sip_message = "INVITE sip:alice@example.com SIP/2.0\r\n\
        Via: SIP/2.0/UDP 192.168.0.1:5060\r\n\
        From: <sip:bob@example.com>;tag=1234\r\n\
        To: <sip:alice@example.com>\r\n\
        Call-ID: 5678\r\n\
        CSeq: 1 INVITE\r\n\
        Contact: <sip:bob@example.com>\r\n\
        Content-Type: application/sdp\r\n\
        Content-Length: 0\r\n\r\n";

    socket.send_to(sip_message.as_bytes(), remote_addr)?;
    Ok(())
}
