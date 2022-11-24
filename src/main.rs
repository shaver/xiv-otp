use std::env;
extern crate exitcode;
use google_authenticator::GoogleAuthenticator;
extern crate reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [TOTP SECRET]", args[0]);
        std::process::exit(exitcode::USAGE);
    }

    let secret = &args[1];
    let auth = GoogleAuthenticator::new();

    let code = auth.get_code(&secret, 0).expect(format!("couldn't generate code from secret {}", secret).as_str());

    println!("code: {}", code);

    let url = format!("http://127.0.0.1:4646/ffxivlauncher/{}", code);
    let body = reqwest::blocking::get(url).expect("couldn't make HTTP connection to XIVLauncher on port 4646");

    if !body.status().is_success() {
        eprintln!("Got error from XIVLauncher: {}", body.status());
        std::process::exit(exitcode::IOERR);
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    static _TEST_URL: &str = "otpauth://totp/Square%20Enix%20ID:PlayerName?secret=LOOP42SA6HFCKGWYGOAT7E4F2JFL17ZG&issuer=Square%20Enix%20ID&algorithm=SHA1&digits=6&period=30";
}

