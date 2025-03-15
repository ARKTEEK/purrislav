use chrono::Utc;
use poise::serenity_prelude::{ChannelId, Color, CreateEmbed, CreateEmbedFooter, Mentionable};

pub fn create_birthday_embed(user_mentions: String) -> CreateEmbed {
  CreateEmbed::new()
      .title("🎉 **Happy Birthday**!")
      .color(Color::GOLD)
      .fields(vec![
        ("📅 Birthdays Today:", user_mentions, false),
      ])
      .footer(CreateEmbedFooter::new("Don't forget to set your birthdays!"))
}

pub fn create_birthday_info_embed(formatted_birthday: String, days_until: i64) -> CreateEmbed {
  CreateEmbed::new()
      .title("🎂 Birthday Information")
      .description("Here's the birthday info you requested!")
      .color(Color::GOLD)
      .fields(vec![
        ("🎉 Birthday:", formatted_birthday, false),
        ("", "".into(), false),
        ("📅 Next Celebration:", format!("In {} days!", days_until), false),
      ])
      .footer(CreateEmbedFooter::new("We're excited for the upcoming celebration!"))
}

pub fn create_birthday_set_embed(user_id: i64, date: String) -> CreateEmbed {
  CreateEmbed::new()
      .title("🎉 Birthday Set Successfully!")
      .description(format!("Birthday for <@{}> has been set to **{}**.", user_id, date))
      .color(Color::DARK_GREEN)
      .footer(CreateEmbedFooter::new("We're excited for the celebration!"))
}

pub fn create_birthday_delete_embed(user_id: i64) -> CreateEmbed {
  CreateEmbed::new()
      .title("🎉 Birthday Deleted Successfully!")
      .description(format!("Birthday for <@{}> has been deleted.", user_id))
      .color(Color::DARK_GREEN)
      .footer(CreateEmbedFooter::new("You can always set your birthday again!"))
}

pub fn create_error_embed(description: String, footer: String) -> CreateEmbed {
  CreateEmbed::new()
      .title("❗ Error Occured")
      .description(description)
      .color(Color::RED)
      .footer(CreateEmbedFooter::new(footer))
}

pub fn create_settings_embed(channel: ChannelId) -> CreateEmbed {
  CreateEmbed::new()
      .title("🗣 Announcments Channel Set!")
      .description(format!("Announcments channel has been set to **{}**.", channel.mention()))
      .color(Color::DARK_GREEN)
      .footer(CreateEmbedFooter::new("Announcments gonna be send there!"))
}

pub fn create_empty_birthday_embed() -> CreateEmbed {
  CreateEmbed::new()
      .title("🎉 No Birthdays Set!")
      .description("It looks like there are no birthdays set for this guild yet.")
      .color(Color::ORANGE)
      .footer(CreateEmbedFooter::new("Set your birthdays and make the guild special!"))
}
