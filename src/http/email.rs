use gloo::{
    storage::{LocalStorage, Storage},
    timers::future::TimeoutFuture,
};

/// verify CAPTCHA with email
/// dev mode, CAPTCHA is '1' will returns true, else false
pub async fn valid_captcha(email: &str, captcha: &str) -> bool {
    _ = email;
    _ = captcha;
    TimeoutFuture::new(1_000).await;
    captcha == "1"
}

/// send a CAPTCHA to specified email address
pub async fn send_captcha(email: &str) {
    _ = email;
    TimeoutFuture::new(1_000).await;
}

const STORED_KEY: &str = "email";

/// record the login status of current email to local
pub fn login(email: &str) {
    let storage = LocalStorage::raw();
    storage.set(STORED_KEY, email).expect("store fail");
}

pub fn logout() {
    let storage = LocalStorage::raw();
    storage.clear().expect("clear storage fail");
}

pub fn get_logged_email() -> Option<String> {
    let storage = LocalStorage::raw();
    storage.get(STORED_KEY).expect("get email from storage fail")
}
