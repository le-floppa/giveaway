use serenity::all::*;

#[allow(dead_code)]
pub async fn register_test_command(_ctx: &Context) {
    let _command = CreateCommand::new("test")
        .name("test")
        .description("A test command");
    
    let _ = CreateEmbed::default()
    .title("Test Command")
        .description("This is a test command response.")
        .color(0x00FF00);
}

pub async fn handle_test_command(ctx: &serenity::prelude::Context, command: serenity::model::application::CommandInteraction) {

    // Handle the test command

    if let Err(why) = command.create_response(&ctx.http, serenity::builder::CreateInteractionResponse::Message(

        serenity::builder::CreateInteractionResponseMessage::new().content("Test command executed!")

    )).await {

        eprintln!("Cannot respond to slash command: {}", why);

    }

}
