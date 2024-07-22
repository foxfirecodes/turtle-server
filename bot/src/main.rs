use bollard::{container::StartContainerOptions, Docker};
use dotenv::dotenv;
use poise::serenity_prelude as serenity;

struct Data {
    docker: Docker,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn startserver(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    ctx.data()
        .docker
        .start_container("turtle-server-mc-1", None::<StartContainerOptions<String>>)
        .await?;
    ctx.say(":turtle: started the server!").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let docker = Docker::connect_with_socket_defaults().expect("failed to connect to Docker");
    let version = docker.version().await.unwrap();
    println!("connected to Docker {}", version.version.unwrap());

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![startserver()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("registering commands...");
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                println!("started discord bot");
                Ok(Data { docker })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
