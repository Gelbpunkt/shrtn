use actix::{Actor, Context, Handler, Message, ResponseFuture};
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};

pub struct RedisActor {
    conn: MultiplexedConnection,
}

impl RedisActor {
    pub async fn new(redis_url: &str) -> Self {
        let client = Client::open(redis_url).unwrap(); // not recommended
        let conn = client.get_multiplexed_async_connection().await.unwrap();
        RedisActor { conn }
    }
}

impl Actor for RedisActor {
    type Context = Context<Self>;
}

#[derive(Message, Debug)]
#[rtype(result = "Result<Option<String>, redis::RedisError>")]
pub struct GetCommand(pub String);

impl Handler<GetCommand> for RedisActor {
    type Result = ResponseFuture<Result<Option<String>, redis::RedisError>>;

    fn handle(&mut self, msg: GetCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        Box::pin(async move {
            let val = con.get(msg.0).await?;
            Ok(val)
        })
    }
}

#[derive(Message, Debug)]
#[rtype(result = "Result<(), redis::RedisError>")]
pub struct SetCommand(pub String, pub String);

impl Handler<SetCommand> for RedisActor {
    type Result = ResponseFuture<Result<(), redis::RedisError>>;

    fn handle(&mut self, msg: SetCommand, _: &mut Self::Context) -> Self::Result {
        let mut con = self.conn.clone();
        Box::pin(async move {
            con.set(msg.0, msg.1).await?;
            Ok(())
        })
    }
}
