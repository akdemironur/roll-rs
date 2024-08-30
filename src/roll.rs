use crate::context::{Context, Error};
use rand::Rng;
use regex::Regex;

fn roll(num_dice: u32, num_sides: u32) -> Vec<u32> {
    let rolls: Vec<u32> = {
        let mut rng = rand::thread_rng();
        (0..num_dice)
            .map(|_| rng.gen_range(1..=num_sides))
            .collect()
    };
    rolls
}

/// Roll dice using xdy±z notation or keep highest notation
#[poise::command(slash_command, aliases("z"))]
pub async fn r(
    ctx: Context<'_>,
    #[description = "Dice notation, e.g., 3d6+2, keep 3 5d6+5, or saved roll name"] input: String,
) -> Result<(), Error> {
    let dice = {
        let saved_rolls = ctx.data().saved_rolls.lock().await;
        saved_rolls.get(&input).cloned().unwrap_or(input)
    };

    let keep_regex = Regex::new(r"keep\s+(\d+)\s+(\d+)d(\d+)\s*([-+])?\s*(\d+)?").unwrap();
    let standard_regex = Regex::new(r"(\d+)d(\d+)\s*([-+])?\s*(\d+)?").unwrap();

    if let Some(caps) = keep_regex.captures(&dice) {
        handle_keep_highest_roll(ctx, &caps).await?;
    } else if let Some(caps) = standard_regex.captures(&dice) {
        handle_standard_roll(ctx, &caps).await?;
    } else {
        ctx.say("Invalid dice notation. Please use the format 'xdy±z' or 'keep N xdy±z', e.g., '3d6+2' or 'keep 3 5d6+5'.").await?;
    }

    Ok(())
}

async fn handle_keep_highest_roll(
    ctx: Context<'_>,
    caps: &regex::Captures<'_>,
) -> Result<(), Error> {
    let keep: u32 = caps[1].parse().unwrap();
    let num_dice: u32 = caps[2].parse().unwrap();
    let num_sides: u32 = caps[3].parse().unwrap();
    let modifier: i32 = caps
        .get(5)
        .map(|m| {
            let value: i32 = m.as_str().parse().unwrap_or(0);
            if caps.get(4).map_or("", |m| m.as_str()) == "-" {
                -value
            } else {
                value
            }
        })
        .unwrap_or(0);

    if num_dice > 100 || num_sides > 100 || keep > num_dice {
        ctx.say("Please use reasonable numbers (max 100 for both dice count and sides, and keep <= dice count)")
            .await?;
        return Ok(());
    }

    let mut rolls = roll(num_dice, num_sides);

    rolls.sort_unstable_by(|a, b| b.cmp(a));
    let kept_rolls: Vec<u32> = rolls.iter().take(keep as usize).cloned().collect();
    let total: i32 = kept_rolls.iter().sum::<u32>() as i32 + modifier;

    let modifier_str = if modifier != 0 {
        format!(" {:+}", modifier)
    } else {
        String::new()
    };

    let response = format!(
        "Rolling {}d{} keep highest {}{}: All rolls: {:?}\nKept rolls: {:?}\nTotal: {}",
        num_dice, num_sides, keep, modifier_str, rolls, kept_rolls, total
    );
    ctx.say(response).await?;
    Ok(())
}

async fn handle_standard_roll(ctx: Context<'_>, caps: &regex::Captures<'_>) -> Result<(), Error> {
    let num_dice: u32 = caps[1].parse().unwrap();
    let num_sides: u32 = caps[2].parse().unwrap();
    let modifier: i32 = caps
        .get(4)
        .map(|m| {
            let value: i32 = m.as_str().parse().unwrap_or(0);
            if caps.get(3).map_or("", |m| m.as_str()) == "-" {
                -value
            } else {
                value
            }
        })
        .unwrap_or(0);

    if num_dice > 100 || num_sides > 100 {
        ctx.say("Please use reasonable numbers (max 100 for both dice count and sides)")
            .await?;
        return Ok(());
    }

    let rolls = roll(num_dice, num_sides);

    let total: i32 = rolls.iter().sum::<u32>() as i32 + modifier;

    let modifier_str = if modifier != 0 {
        format!("{:+}", modifier)
    } else {
        String::new()
    };

    let response = format!(
        "Rolling {}d{}{}: {:?}\nTotal: {}",
        num_dice, num_sides, modifier_str, rolls, total
    );

    ctx.say(response).await?;
    Ok(())
}
