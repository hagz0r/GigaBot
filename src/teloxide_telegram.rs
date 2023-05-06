type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

use std::error::Error;
use mongodb::Collection;
use teloxide::{
    prelude::*,
    dispatching::dialogue,
    dispatching::UpdateHandler,
    dispatching::dialogue::InMemStorage,
    utils::command::BotCommands,
};

use crate::{models::conversation::ConversationBlock,
            models::user::AppUser,
            app_settings::{SETTINGS}, mongo::{
        get_collection,
        get_conversation,
        get_database,
    }, mongo};


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "You can use this commands:")]
enum Command {
    #[command(description = "Start new topic.")]
    Clear,
    #[command(description = "Donate me")]
    Donate,
    #[command(description = "Display help page.")]
    Help,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Conversation,
    //Empty,
}

pub async fn run(bot: Bot) {
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}


fn schema() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Clear].endpoint(clear))
        .branch(case![Command::Donate].endpoint(donate))
        .branch(case![Command::Help].endpoint(help));

    let msg_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::Conversation].endpoint(reply));


    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(msg_handler)
}

async fn clear(bot: Bot, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let id = msg.from().unwrap().id;
    let db = get_database("gigachat_rust").await;
    let collection = get_collection("teloxide", &db);


    mongo::clear_conversation(id, &collection).await;
    bot.send_message(id, "New conversation started.").await?;
    Ok(())
}


async fn donate(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id,
                     format!("ETH: {}", &*SETTINGS.eth_address)).await?;
    Ok(())
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn reply(bot: Bot, msg: Message) -> HandlerResult {
    let from = msg.from().unwrap();
    let db = get_database("gigachat_rust").await;
    let collection: Collection<AppUser> = get_collection("teloxide", &db);

    log::info!(
        "\nFrom: {} Id: {}\nMessage: {}",
        from.first_name,
        from.id,
        msg.text().unwrap()
    );

    let gigachat_answer = msg.text().unwrap().to_string();


    let block =
        ConversationBlock::from_telx(&msg, &gigachat_answer);


    let app_user =
        AppUser::new(from.clone(),
                     get_conversation(from.id, &collection).await.unwrap());


    mongo::upsert_user(&app_user, &collection).await.unwrap();
    mongo::add_message(&collection, &block).await.unwrap();

    bot.send_message(msg.chat.id, gigachat_answer).await?;


    Ok(())
}