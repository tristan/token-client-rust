extern crate ws;
extern crate url;
extern crate openssl;

use self::openssl::ssl::{SslConnectorBuilder, SslMethod, SslStream, SslVerifyMode};
use self::ws::util::TcpStream;

use protobuf::Message;
use super::WebSocketResources;
use signal::SignalService;
use std::rc::Rc;
use std::cell::RefCell;

use super::ChatService;
use self::url::percent_encoding::{utf8_percent_encode, QUERY_ENCODE_SET};
use signal::cipher::{IV_OFFSET, IV_LENGTH, CIPHER_KEY_SIZE,
                     CIPHERTEXT_OFFSET, MAC_SIZE, VERSION_LENGTH,
                     decrypt_cbc, verify_mac};
use signal::message::{TokenMessage};
use signal::protocol::{SignalProtocolStore};
use super::process_envelope;

trait MessageHandler {
    fn on_message(&mut self, _: TokenMessage);
}

impl<F> MessageHandler for F where F: FnMut(TokenMessage) {
    fn on_message(&mut self, msg: TokenMessage) {
        self(msg)
    }
}

struct WebsocketHandler<F> where F: MessageHandler {
    ws: ws::Sender,
    signaling_key: Vec<u8>,
    store: SignalProtocolStore,
    handler: Rc<RefCell<Box<F>>>
    //handler: Box<FnMut(&mut ChatService, &SignalService::Envelope)>
}

const GRATI: ws::util::Token = ws::util::Token(1);

impl<F> ws::Handler for WebsocketHandler<F> where F: MessageHandler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        if let Some(addr) = try!(shake.remote_addr()) {
            println!("Connection with {} now open", addr);
        }
        // schedule keepalive ping
        try!(self.ws.timeout(5_000, GRATI));
        Ok(())
    }

    fn build_request(&mut self, url: &url::Url) -> ws::Result<ws::Request> {
        // create request and add X-Signal-Agent header
        let mut req = try!(ws::Request::from_url(url));
        {
            let mut headers = req.headers_mut();
            headers.push(("X-Signal-Agent".to_string(), "Token-Client-Rust".as_bytes().to_vec()));
        }
        Ok(req)
    }
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        //println!("Received message {:?}", msg);
        let mut wsmsg = WebSocketResources::WebSocketMessage::new();
        wsmsg.merge_from_bytes(&msg.into_data()).unwrap();
        let field_type = wsmsg.get_field_type() as u32;
        // make sure this is a request message
        if field_type == 1 {
            let request = wsmsg.get_request();
            let ciphertext = request.get_body().to_vec();
            let iv: Vec<u8> = ciphertext[IV_OFFSET..IV_OFFSET+IV_LENGTH].to_vec();
            let cipherkey: Vec<u8> = self.signaling_key[..CIPHER_KEY_SIZE].to_vec();
            let mac: Vec<u8> = self.signaling_key[CIPHER_KEY_SIZE..].to_vec();
            if ! verify_mac(&ciphertext, &mac) {
                println!("INVALID MAC!");
            } else {
                let ciphertext = ciphertext[CIPHERTEXT_OFFSET..(CIPHERTEXT_OFFSET + ciphertext.len() - VERSION_LENGTH - IV_LENGTH - MAC_SIZE)].to_vec();
                let plaintext = decrypt_cbc(&ciphertext, &cipherkey, &iv).unwrap();
                let mut envelope = SignalService::Envelope::new();
                envelope.merge_from_bytes(&plaintext).unwrap();
                let res = process_envelope(&mut self.store, &envelope);
                match res {
                    Ok((_, _, msg)) => {
                        if msg.is_some() {
                            self.handler.borrow_mut().on_message(msg.unwrap());
                        }
                    },
                    Err(error) => {
                        println!("Error processing envelope: {:?}", error);
                    }
                };
            }
            // createWebSocketResponse
            let mut resp = WebSocketResources::WebSocketResponseMessage::new();
            resp.set_id(request.get_id());
            if request.get_verb() == "PUT" && request.get_path() == "/api/v1/message" {
                // send 200
                resp.set_status(200);
                resp.set_message("OK".to_string());
            } else {
                // send 400
                resp.set_status(400);
                resp.set_message("Unknown".to_string());
            }
            let mut resp_msg = WebSocketResources::WebSocketMessage::new();
            resp_msg.set_field_type(WebSocketResources::WebSocketMessage_Type::RESPONSE);
            resp_msg.set_response(resp);
            match self.ws.send(resp_msg.write_to_bytes().unwrap()) {
                Ok(_) => {},
                Err(error) => {
                    println!("Error sending websocket response: {:?}", error);
                }
            };
        };
        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        if reason.is_empty() {
            println!("<<< Closing<({:?})>\nHit any key to end session.", code);
        } else {
            println!("<<< Closing<({:?}) {}>\nHit any key to end session.", code, reason);
        }
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("<<< Error<{:?}>", err);
    }

    fn on_timeout(&mut self, event: ws::util::Token) -> ws::Result<()> {
        if event == GRATI {
            // send gratuitous pong
            try!(self.ws.pong(vec![]));
            // reschedule the timeout
            self.ws.timeout(5_000, GRATI)
        } else {
            Err(ws::Error::new(ws::ErrorKind::Internal, "Invalid timeout token encountered!"))
        }
    }

    fn upgrade_ssl_client(&mut self, sock: TcpStream, _: &url::Url) -> ws::Result<SslStream<TcpStream>> {
        let mut builder = SslConnectorBuilder::new(SslMethod::tls()).map_err(|e| {
            ws::Error::new(ws::ErrorKind::Internal, format!("Failed to upgrade client to SSL: {}", e))
        })?;
        builder.builder_mut().set_verify(SslVerifyMode::empty());

        let connector = builder.build();
        connector
            .danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication(sock)
            .map_err(From::from)
    }
}

struct Factory<F> where F: MessageHandler {
    signaling_key: Vec<u8>,
    store: SignalProtocolStore,
    handler: Rc<RefCell<Box<F>>>
}

impl<F> ws::Factory for Factory<F> where F: MessageHandler {
    type Handler = WebsocketHandler<F>;
    fn connection_made(&mut self, out: ws::Sender) -> WebsocketHandler<F> {
        WebsocketHandler {
            ws: out,
            signaling_key: self.signaling_key.clone(),
            store: self.store.clone(),
            handler: self.handler.clone()
        }
    }
}

impl ChatService {

    pub fn websocket_connect<F>(&mut self, signaling_key: &[u8;52], handler: F) -> Result<(),String>
        where F: FnMut(TokenMessage) {

        let password: String = {
            utf8_percent_encode(self.password.as_str(), QUERY_ENCODE_SET).collect()
        };
        let url = format!("ws{}/v1/websocket/?login={}&password={}",
                          &self.base_url[4..], self.token_id.to_string(), password);
        let parsed = try!(url::Url::parse(&url)
                          .map_err(|err| format!("Unable to parse {} as url due to {:?}", url, err)));

        let factory = Factory {
            store: self.store.clone(),
            signaling_key: signaling_key.to_vec(),
            handler: Rc::new(RefCell::new(Box::new(handler)))
        };

        let mut ws = match ws::WebSocket::new(factory) {
            Ok(ws) => ws,
            Err(error) => {
                return Err(format!("Failed to create WebSocket due to: {:?}", error));
            }
        };

        try!(ws.connect(parsed).map_err(|err| format!("Failed to create WebSocket due to: {:?}", err)));
        try!(ws.run().map_err(|err| format!("Failed to create WebSocket due to: {:?}", err)));
        Ok(())
    }
}
