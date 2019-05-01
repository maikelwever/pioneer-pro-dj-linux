extern crate tokio;
extern crate tokio_codec;
extern crate bytes;
extern crate futures;

use std::net::{SocketAddr};
use std::io::{Read, Write};
use std::thread;
use std::sync::{Arc, Mutex};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{read_exact, write_all};
use tokio::codec::{BytesCodec, Decoder};
use tokio::prelude::*;
use bytes::{Bytes, BytesMut};

use super::db::{RecordDB};

#[derive(Debug, PartialEq)]
pub struct Artist {
    value: String,
}

type SharedLibrary = Mutex<RecordDB<Artist>>;

struct Library;
impl Library {
    pub fn start_page() -> Vec<u8> {
        vec![0xff, 0x20]
    }

    // This contains artist and playlists views
    // Seems to be structed data so this will be reusable for listing things in the displays.
    pub fn tbd(reference: u8) -> Bytes {
        Bytes::from(vec![
            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x40,0x01,0x0f,0x02,0x14,
            0x00,0x00,0x00,0x0c,0x06,0x06,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x11,0x00,
            0x00,0x00,0x01,0x11,0x00,0x00,0x00,0x00,

// ==========================================

            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x41,0x01,0x0f,0x0c,0x14,0x00,0x00,
            0x00,0x0c,0x06,0x06,0x06,0x02,0x06,0x02,0x06,0x06,0x06,0x06,0x06,0x06,0x11,0x00,0x00,0x00,
            0x00,0x11,0x00,0x00,0x00,

            // Index??
            0x02,

            0x11,0x00,0x00,0x00,0x12,0x26,0x00,0x00,0x00,0x09,0xff,0xfa,

            // ARTIST
            0x00,0x41,0x00,0x52,0x00,0x54,0x00,0x49,0x00,0x53,0x00,0x54,

            0xff,0xfb,0x00,0x00,0x11,0x00,0x00,0x00,0x02,0x26,0x00,0x00,0x00,0x01,0x00,0x00,0x11,0x00,
            0x00,0x00,

            0x81,

            0x11,0x00,0x00,0x00,0x00,0x11,0x00,0x00,0x00,0x00,0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,0x11,0x00,0x00,0x00,0x00,

// ==========================================

            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x41,0x01,0x0f,0x0c,0x14,0x00,0x00,
            0x00,0x0c,0x06,0x06,0x06,0x02,0x06,0x02,0x06,0x06,0x06,0x06,0x06,0x06,0x11,0x00,0x00,0x00,
            0x00,0x11,0x00,0x00,0x00,

            // Index??
            0x03,

            0x11,0x00,0x00,0x00,0x10,0x26,0x00,0x00,0x00,0x08,0xff,0xfa,

            // ALBUM
            0x00,0x4a,0x00,0x4f,0x00,0x4e,0x00,0x41,0x00,0x53,

            0xff,0xfb,0x00,0x00,0x11,0x00,0x00,0x00,0x02,0x26,0x00,0x00,0x00,0x01,0x00,0x00,0x11,0x00,
            0x00,0x00,

            0x82,

            0x11,0x00,0x00,0x00,0x00,0x11,0x00,0x00,0x00,0x00,0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,0x11,0x00,0x00,0x00,0x00,

// ==========================================

            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x41,0x01,0x0f,0x0c,0x14,0x00,0x00,
            0x00,0x0c,0x06,0x06,0x06,0x02,0x06,0x02,0x06,0x06,0x06,0x06,0x06,0x06,0x11,0x00,0x00,0x00,
            0x00,0x11,0x00,0x00,0x00,

            // Index???
            0x04,

            0x11,0x00,0x00,0x00,0x10,0x26,0x00,0x00,0x00,0x08,0xff,0xfa,

            // TRACK
            0x00,0x54,0x00,0x52,0x00,0x41,0x00,0x43,0x00,0x4b,

            0xff,0xfb,0x00,0x00,0x11,0x00,0x00,0x00,0x02,0x26,0x00,0x00,0x00,0x01,0x00,0x00,0x11,0x00,
            0x00,0x00,

            0x83,

            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,

// ==========================================

            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x41,0x01,0x0f,0x0c,0x14,0x00,0x00,
            0x00,0x0c,0x06,0x06,0x06,0x02,0x06,0x02,0x06,0x06,0x06,0x06,0x06,0x06,0x11,0x00,0x00,0x00,
            0x00,0x11,0x00,0x00,0x00,

            // Index??, Hopefully this is the List Item Id
            0x0c,

            0x11,0x00,0x00,0x00,0x0c,0x26,0x00,0x00,0x00,0x06,0xff,0xfa,

            // KEY
            0x00,0x4b,0x00,0x45,0x00,0x59,

            0xff,0xfb,0x00,0x00,0x11,0x00,0x00,0x00,0x02,0x26,0x00,0x00,0x00,0x01,0x00,0x00,0x11,0x00,
            0x00,0x00,

            0x8b,

            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,

// ==========================================

            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x41,0x01,0x0f,0x0c,0x14,0x00,0x00,
            0x00,0x0c,0x06,0x06,0x06,0x02,0x06,0x02,0x06,0x06,0x06,0x06,0x06,0x06,0x11,0x00,0x00,0x00,
            0x00,0x11,0x00,0x00,0x00,

            // Index??
            0x05,

            0x11,0x00,0x00,0x00,0x16,0x26,0x00,0x00,0x00,0x0b,0xff,0xfa,

            // PLAYLIST
            0x00,0x50,0x00,0x4c,0x00,0x41,0x00,0x59,0x00,0x4c,0x00,0x49,0x00,0x53,0x00,0x54,

            0xff,0xfb,0x00,0x00,0x11,0x00,0x00,0x00,0x02,0x26,0x00,0x00,0x00,0x01,0x00,0x00,0x11,0x00,
            0x00,0x00,

            0x84,

            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,

// ==========================================

            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x41,0x01,0x0f,0x0c,0x14,0x00,0x00,
            0x00,0x0c,0x06,0x06,0x06,0x02,0x06,0x02,0x06,0x06,0x06,0x06,0x06,0x06,0x11,0x00,0x00,0x00,
            0x00,0x11,0x00,0x00,0x00,

            // Index?
            0x16,

            0x11,0x00,0x00,0x00,0x14,0x26,0x00,0x00,0x00,0x0a,0xff,0xfa,

            // SOME TEXT
            0x00,0x48,0x00,0x49,0x00,0x53,0x00,0x54,0x00,0x4f,0x00,0x52,0x00,0x59,

            0xff,0xfb,0x00,0x00,0x11,0x00,0x00,0x00,0x02,0x26,0x00,0x00,0x00,0x01,0x00,0x00,0x11,0x00,
            0x00,0x00,

            0x95,

            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,

// ==========================================

            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x41,0x01,0x0f,0x0c,0x14,0x00,0x00,
            0x00,0x0c,0x06,0x06,0x06,0x02,0x06,0x02,0x06,0x06,0x06,0x06,0x06,0x06,0x11,0x00,0x00,0x00,
            0x00,0x11,0x00,0x00,0x00,

            // Index?
            0x12,

            0x11,0x00,0x00,0x00,0x12,0x26,0x00,0x00,0x00,0x09,0xff,0xfa,

            // SOME TEXT
            0x00,0x53,0x00,0x45,0x00,0x41,0x00,0x52,0x00,0x43,0x00,0x48,

            0xff,0xfb,0x00,0x00,0x11,0x00,0x00,0x00,0x02,0x26,0x00,0x00,0x00,0x01,0x00,0x00,0x11,0x00,
            0x00,0x00,

            0x91,

            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,
            0x11,0x00,0x00,0x00,0x00,

// ==========================================

            0x11,0x87,0x23,0x49,0xae,0x11,0x05,0x80,0x00,reference,0x10,0x42,0x01,0x0f,0x00,0x14,
            0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
        ])
    }
}

pub fn client_response(mut stream: TcpStream, data: Vec<u8>) {
    if let Err(e) = stream.write(data.as_ref()) {
        eprintln!("{:?}", e);
    }
}

pub enum Event {
    RemoteDBServer,
    Unsupported,
}

pub fn get_package_type(buffer: &[u8]) -> Event {
    match buffer {
         &[0, 0, 0, 15, 82, 101, 109, 111, 116, 101, 68, 66, 83, 101, 114, 118, 101, 114, 0] => Event::RemoteDBServer,
         _ => {
             Event::Unsupported
         }
    }
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 64];
    match stream.read(&mut buf) {
        Ok(size) => {
            match get_package_type(&buf[..size]) {
                Event::RemoteDBServer => client_response(stream, Library::start_page()),
                Event::Unsupported => {},
            }
        },
        Err(err) => eprintln!("{:?}", err),
    }
}

#[derive(Debug)]
enum Request {
    Initiate(Bytes),
    QueryListItem(Bytes),
    FetchListItemContent(Bytes),
    Unimplemented,
}
#[derive(Debug)]
enum Response {
    Initiate(Bytes),
    Unimplemented(Bytes),
}

fn is_library_browsing_request(bytes: &[u8]) -> bool {
    bytes == [0x11, 0x87, 0x23, 0x49, 0xae, 0x11]
}

impl Request {
    fn parse(input: BytesMut, client_context: &SharedClientContext) -> Result<Request, &'static str> {
        if input.len() == 5 {
            Ok(Request::Initiate(input.freeze()))
        } else if is_library_browsing_request(&input[0..=5]) {
            Ok(match input.len() {
                37 => Request::Initiate(
                    Bytes::from(vec![
                        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0xff, 0xff,
                        0xff, 0xfe, 0x10, 0x40, 0x00, 0x0f, 0x02, 0x14,
                        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
                        0x00, 0x11,
                    ])
                ),
                47 => {
                    Request::Initiate(Bytes::from(vec![
                        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
                        0x00, input[9], 0x10, 0x40, 0x00, 0x0f, 0x02, 0x14,
                        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x11, 0x00, 0x00, 0x10, 0x00, 0x11, 0x00, 0x00,
                        0x00, 0x08
                    ]))
                },
                42 => {
                    if let Ok(db) = client_context.db.lock() {
                        if let Ok(table) = db.table(input[12]) {
                            eprintln!("{:?}", table);
                        }
                    }
                    eprintln!("QueryPage: {:?}", input[12]);
                    Request::QueryListItem(Bytes::from(vec![]))
                },
                62 => Request::FetchListItemContent(Library::tbd(input[9])),
                _ => {
                    Request::Unimplemented
                },
            })
        } else {
            eprintln!("parsing TCP package failed; package = {:?}", input);
            Err("parsing TCP package failed")
        }
    }
}

type SharedClientContext = Arc<ClientContext>;
struct ClientContext {
    db: SharedLibrary,
}

impl ClientContext {
    pub fn new(db: RecordDB<Artist>) -> Self {
        Self {
            db: Mutex::new(db),
        }
    }
}

fn process(bytes: BytesMut, client_context: &SharedClientContext) -> Result<Response, &'static str> {
    if let Ok(request) = Request::parse(bytes, client_context) {
        Ok(match request {
            Request::Initiate(response) => Response::Initiate(response),
            Request::QueryListItem(response) => Response::Initiate(response),
            Request::FetchListItemContent(response) => Response::Initiate(response),
            Request::Unimplemented => Response::Unimplemented(Bytes::from("Unimplemented")),
        })
    } else {
        Err("Failed processing request into response")
    }
}

pub struct TcpServer;
impl TcpServer {
    fn library_server(address: &str, client_context: SharedClientContext) {
        let addr = address.parse::<SocketAddr>().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        tokio::run({
            listener
                .incoming()
                .map_err(|err| eprintln!("Failed to accept socket; error = {:?}", err))
                .for_each(move |socket| {
                    let framed = BytesCodec::new().framed(socket);
                    let (writer, reader) = framed.split();
                    let client_context = client_context.clone();

                    let responses = reader.map(move |bytes| {
                        let client_context = &client_context;
                        match process(bytes, client_context) {
                            Ok(Response::Initiate(response)) => response,
                            Ok(Response::Unimplemented(response)) => response,
                            Err(err) => Bytes::from(err),
                        }
                    });

                    let writes = responses.fold(writer, |writer, response| {
                        writer.send(response)
                    });

                    let msg = writes.then(move |_w| Ok(()));

                    tokio::spawn(msg)
                })
        });
    }

    fn library_initializer(address: &str) {
        let addr = address.parse::<SocketAddr>().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        let done = listener
            .incoming()
            .map_err(|e| println!("failed to accept socket; error = {:?}", e))
            .for_each(|socket| {
                let processor = read_exact(socket, vec![0; 19])
                    .and_then(|(socket, _bytes)| {
                        // TODO: Implement connection pool
                        //       Here we write back the TCP Port that the client will
                        //       respond back to.
                        write_all(socket, &[0xff, 0x20]).then(|_| Ok(()))
                    })
                    .map_err(|err| eprintln!("Failed responding to port: {:?}", err));
                tokio::spawn(processor)
            });
        tokio::run(done);
    }

    pub fn run() {
        thread::spawn(move || {
            TcpServer::library_initializer("0.0.0.0:12523");
        });

        thread::spawn(move || {
            let mut db = RecordDB::new();

            // Assign some artists
            if let Ok(Some(table)) = db.mut_table(0x02) {
                table.insert(Artist {
                    value: "Jonas Liljestrand".to_string(),
                });

                table.insert(Artist {
                    value: "Tokio".to_string(),
                });
            }

            let client_context = ClientContext::new(db);
            TcpServer::library_server("0.0.0.0:65312", Arc::new(client_context));
        });
    }
}
