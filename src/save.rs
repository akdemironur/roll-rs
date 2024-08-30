use crate::context::{Context, Error};
/// Save a dice roll
#[poise::command(slash_command)]
pub async fn save(
    ctx: Context<'_>,
    #[description = "Dice notation to save"] dice: String,
    #[description = "Name for the saved roll"] name: String,
) -> Result<(), Error> {
    let mut saved_rolls = ctx.data().saved_rolls.lock().await;
    if let Some(old_dice) = saved_rolls.get(&name) {
        ctx.say(format!(
            "Updated saved roll '{}' from '{}' to '{}'",
            name, old_dice, dice
        ))
        .await?;
    } else {
        ctx.say(format!("Saved roll '{}' as '{}'", dice, name))
            .await?;
    }
    saved_rolls.insert(name.clone(), dice.clone());
    Ok(())
}

/// Remove a saved dice roll
#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Name of the saved roll to remove"] name: String,
) -> Result<(), Error> {
    let mut saved_rolls = ctx.data().saved_rolls.lock().await;
    if saved_rolls.remove(&name).is_some() {
        ctx.say(format!("Removed saved roll '{}'", name)).await?;
    } else {
        ctx.say(format!("No saved roll found with the name '{}'", name))
            .await?;
    }
    Ok(())
}

/// View all saved dice rolls
#[poise::command(slash_command)]
pub async fn view(ctx: Context<'_>) -> Result<(), Error> {
    let saved_rolls = ctx.data().saved_rolls.lock().await;
    if saved_rolls.is_empty() {
        ctx.say("No saved rolls").await?;
    } else {
        let response = saved_rolls
            .iter()
            .map(|(name, dice)| format!("{}: {}", name, dice))
            .collect::<Vec<_>>()
            .join("\n");
        ctx.say(response).await?;
    }
    Ok(())
}
