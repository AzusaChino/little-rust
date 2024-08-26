use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "Responses")]
enum Messages {
    Ping,
    Pong,
}

enum Responses {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for Responses
where
    A: Actor,
    M: Message<Result = Responses>,
{
    fn handle(
        self,
        _: &mut <A as Actor>::Context,
        tx: Option<OneshotSender<<M as Message>::Result>>,
    ) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

#[derive(Default)]
struct MyActor {}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("Actor is stopped");
    }
}

impl Handler<Messages> for MyActor {
    type Result = Responses;

    fn handle(&mut self, msg: Messages, _: &mut Self::Context) -> Self::Result {
        match msg {
            Messages::Ping => {
                println!("Got Ping");
                Responses::GotPing
            }
            Messages::Pong => {
                println!("Got Pong");
                Responses::GotPong
            }
        }
    }
}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    // start new actor
    let addr = MyActor::default().start();

    // Send Ping message.
    // send() message returns Future object, that resolves to message result
    let ping_future = addr.send(Messages::Ping).await;
    let pong_future = addr.send(Messages::Pong).await;

    match pong_future {
        Ok(res) => match res {
            Responses::GotPing => println!("Ping received"),
            Responses::GotPong => println!("Pong received"),
        },
        Err(e) => println!("Actor is probably dead: {}", e),
    }

    match ping_future {
        Ok(res) => match res {
            Responses::GotPing => println!("Ping received"),
            Responses::GotPong => println!("Pong received"),
        },
        Err(e) => println!("Actor is probably dead: {}", e),
    }

    Ok(())
}
