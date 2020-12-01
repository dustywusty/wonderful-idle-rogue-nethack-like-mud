use actix::{Actor, StreamHandler};
use actix_web::{dev::Server, rt, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use signal_hook::{iterator::Signals, SIGINT};
use specs::{world::Builder, DispatcherBuilder, World, WorldExt};
use std::collections::VecDeque;
use std::sync::{mpsc, Mutex};
use std::{thread, time};

mod components;
use components::*;

mod systems;
use systems::*;

// ============================================================
// ============================================================

pub struct GameCommand {}

pub struct GameState {
    commands: Mutex<VecDeque<GameCommand>>,
    pub ecs: World,
}

impl GameState {
    pub fn foobar() {
        println!("goobar");
    }
}

// ============================================================
// ============================================================

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

fn run_server(tx: mpsc::Sender<Server>) -> std::io::Result<()> {
    let mut sys = rt::System::new("test");

    // srv is server controller type, `dev::Server`
    let srv = HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run();
    // send server controller to main thread
    let _ = tx.send(srv.clone());

    // run future
    sys.block_on(srv)
}

// ============================================================
// ============================================================

fn main() {
    let mut gs = GameState {
        ecs: World::new(),
        commands: Mutex::new(VecDeque::new()),
    };

    let signals = Signals::new(&[SIGINT]);

    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();

    let (tx, rx) = mpsc::channel();

    println!("START SERVER");
    thread::spawn(move || {
        let _ = run_server(tx);
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    let srv = rx.recv().unwrap();

    println!("WATING 10 SECONDS");
    thread::sleep(time::Duration::from_secs(10));

    println!("STOPPING SERVER");
    // init stop server and wait until server gracefully exit
    rt::System::new("").block_on(srv.stop(true));

    // let _player = gs
    //     .ecs
    //     .create_entity()
    //     .with(Player {})
    //     .with(Name {
    //         name: "Dusty".to_string(),
    //     })
    //     .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintingSystem, "print_sys", &[])
        .build();

    loop {
        if gs.commands.lock().unwrap().is_empty() {
            GameState::foobar();
        }
        dispatcher.dispatch(&mut gs.ecs);
        gs.ecs.maintain();
    }
}
