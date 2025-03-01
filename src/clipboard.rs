use copypasta::{ClipboardContext, ClipboardProvider};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::error::Error;
use zeroize::Zeroize;

/// paste the given password to the OS clipboard.
pub fn paste_to_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = match ClipboardContext::new() {
        Ok(c) => c,
        Err(e) => {
            let box_err: Box<dyn Error> =
                format!("Could not create clipboard context: {}", e).into();
            return Err(box_err);
        }
    };
    if let Err(e) = ctx.set_contents(content.to_owned()) {
        let box_err: Box<dyn Error> =
            format!("Could not paste password to clipboard: {}", e).into();
        return Err(box_err);
    }
    let clipboard_content = match ctx.get_contents() {
        Ok(c) => c,
        Err(e) => {
            let box_err: Box<dyn Error> =
                format!("Could not read clipboard for verification: {}", e).into();
            return Err(box_err);
        }
    };
    if content != clipboard_content {
        let box_err: Box<dyn Error> = "Failed to paste password to clipboard!".to_string().into();
        return Err(box_err);
    }
    println!("\nPasted password to clipboard.");
    Ok(())
}

pub fn clear_clipboard(content: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = match ClipboardContext::new() {
        Ok(c) => c,
        Err(e) => {
            let box_err: Box<dyn Error> =
                format!("Could not create clipboard context: {}", e).into();
            return Err(box_err);
        }
    };
    let mut clipboard_content = match ctx.get_contents() {
        Ok(c) => c,
        Err(e) => {
            let box_err: Box<dyn Error> =
                format!("Could read clipboard for verification: {}", e).into();
            return Err(box_err);
        }
    };
    if content == clipboard_content {
        clipboard_content.zeroize();
        let random_clipboard_content: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(content.len())
            .map(char::from)
            .collect();
        if let Err(e) = ctx.set_contents(random_clipboard_content.to_owned()) {
            let box_err: Box<dyn Error> = format!("Could not overwrite clipboard: {}", e).into();
            return Err(box_err);
        }
        let empty_clipboard_value = if cfg!(windows) { " " } else { "" };

        if let Err(e) = ctx.set_contents(empty_clipboard_value.to_owned()) {
            let box_err: Box<dyn Error> = format!("Could not clear clipboard: {}", e).into();
            return Err(box_err);
        }
        println!("Cleared clipboard.");
    } else {
        println!("Clipboard has changed, doing nothing.");
    }

    Ok(())
}
