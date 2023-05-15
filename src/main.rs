extern crate ansi_term;
extern crate rand;
use ansi_term::Style;
use rand::Rng;
use std::env::args;
use std::process::Command;
const FAILED: [&str; 10] = [
    "Well, congratulations! You've managed to set a new record for failure. Quite the accomplishment, really",
    "You know, it's impressive how consistently you find new and innovative ways to fall short of expectations",
    "Failure seems to be your superpower. I'm starting to think you should put it on your resume",
    "Don't worry, I'm here to offer my unwavering support as you continue to disappoint everyone around you",
    "Ah, failure strikes again! It's almost impressive how reliable you are in your inability to succeed",
    "If there were an Olympic event for failure, I have no doubt you'd take home the gold medal every time",
    "It's amazing how your failures have become such a consistent part of your identity. You've really embraced it.",
    "Keep up the good work! You're doing an excellent job at demonstrating how not to accomplish anything.",
    "They say practice makes perfect, so at this rate, you must be on your way to perfection.",
    "Failure may be your middle name, but hey, at least you're consistent. That counts for something, right?"
];
const SUCCESS : [&str; 10] = [
    "Wow, you actually did it! I guess even a broken clock is right twice a day.",
    "Look who finally managed to stumble upon success. Don't worry, I'm as surprised as you are.",
    "Well, well, well... I must admit, your success caught me off guard. I hope it's not a fluke.",
    "Color me impressed! You've defied all odds and achieved something worthwhile. Who would have thought?",
    "Success has a strange way of finding even the most unlikely candidates. You're living proof of that.",
    "Huh, you're actually capable of accomplishing something meaningful. Consider me pleasantly surprised.",
    "Congratulations, you've achieved greatness! I never thought I'd see the day, but miracles do happen.",
    "Well, I'll be. You've managed to succeed where so many others have failed. Enjoy the moment.",
    "Bravo! I'll give credit where it's due—you've accomplished something worthy of recognition.",
    "Success seems to suit you, my friend. I hope it's not a fleeting moment, but a sign of greater things to come."
];
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if Command::new("cargo")
        .args(args().skip(1).collect::<Vec<String>>())
        .spawn()?
        .wait()?
        .success()
    {
        println!(
            "\n\n{}\n",
            Style::new()
                .bold()
                .paint(SUCCESS[rand::thread_rng().gen_range(0..SUCCESS.len())])
        );
    } else {
        println!(
            "\n\n{}\n",
            Style::new()
                .bold()
                .paint(FAILED[rand::thread_rng().gen_range(0..FAILED.len())])
        );
    }

    Ok(())
}
