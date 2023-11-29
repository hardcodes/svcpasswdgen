use copypasta::{ClipboardContext, ClipboardProvider};
use std::error::Error;

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
                format!("Could read clipboard for verification: {}", e).into();
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
