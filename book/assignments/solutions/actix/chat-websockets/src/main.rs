mod chat_server;

use std::time::{Duration, Instant};

use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_files as fs;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use chat_server::Join;
use log::{error, info, warn};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(2);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Define HTTP actor
struct WsChatSession {
    /// unique session id
    id: usize,
    /// The address of the chat server to communicate with
    server: Addr<chat_server::ChatServer>,
    /// Client must ping regularly in a time interval, otherwise will time out
    heartbeat: Instant,
    /// The name of the client if given
    name: Option<String>,
}

impl actix::Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Actor has started
    fn started(&mut self, ctx: &mut Self::Context) {
        // Start heart beat check once here
        self.heartbeat(ctx);

        let addr = ctx.address();
        // send Join message to chat server, wait until chat server responds with session id
        self.server
            .send(Join {
                recipient: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, recipient, ctx| {
                match res {
                    Ok(session_id) => recipient.id = session_id,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    /// Actor is stopping the session
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server.do_send(chat_server::Disconnect {
            id: self.id,
            name: self.name.clone(),
        });
        Running::Stop
    }
}

/// Handle messages from chat server, forward to peer socket
impl Handler<chat_server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: chat_server::Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

/// Handler for ws::Message messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        info!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(message)) => {
                if let Some((command, message)) = message.trim().split_once(":") {
                    match command {
                        "text" => {
                            self.server.do_send(chat_server::ClientMessage {
                                session_id: self.id,
                                message: message.to_string(),
                                name: self.name.clone(),
                            });
                        }
                        "name" => {
                            let name = message.trim();
                            if name.len() > 0 {
                                self.name = Some(name.to_string());
                            } else {
                                self.name = None;
                            }
                        }
                        c => {
                            warn!("Unsupported command '{}' found", c);
                        }
                    }
                } else {
                    warn!("Failed to parse message '{}'", message);
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
            }
            Ok(ws::Message::Continuation(_)) => (),
            _ => (),
        }
    }
}

impl WsChatSession {
    /// A helper method that sends a PING to client every second to determine if
    /// still connected. In case there is no response in time, disconnect client.
    ///
    /// This function should be called only once, the closure to `AsyncContext::run_interval` is
    /// executed periodically.
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |session, ctx| {
            if Instant::now().duration_since(session.heartbeat) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // Send chat server a Disconnect message
                session.server.do_send(chat_server::Disconnect {
                    id: session.id,
                    name: session.name.clone(),
                });

                // stop actor
                ctx.stop();

                // dont try to send a ping
                return;
            }

            // ping the actor
            ctx.ping(b"");
        });
    }
}

/// Entry point to websocket route / chat
async fn chat_index(
    req: HttpRequest,
    stream: web::Payload,
    service: web::Data<Addr<chat_server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        WsChatSession {
            id: 0,
            server: service.get_ref().clone(),
            heartbeat: Instant::now(),
            name: None,
        },
        &req,
        stream,
    );
    info!("{:?}", resp);
    resp
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let chat_server = chat_server::ChatServer::new().start();

    info!("Starting server");
    HttpServer::new(move || {
        App::new()
            .data(chat_server.clone())
            .service(web::resource("/ws/").route(web::get().to(chat_index)))
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
