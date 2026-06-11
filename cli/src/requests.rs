use serde::Serialize;

#[derive(Serialize)]
struct UserRequest {
    user: String,
    pass: String,
}

pub fn create_user(
    host: &str,
    user: &str,
    pass: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let body = UserRequest {
        user: user.into(),
        pass: pass.into(),
    };

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(format!("{}/api/v1/auth/create_professor_user", host))
        .json(&body)
        .send()?;

    let text = response.text()?;
    Ok(text)
}

pub fn change_pass(
    host: &str,
    user: &str,
    pass: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let body = UserRequest {
        user: user.into(),
        pass: pass.into(),
    };

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(format!("{}/api/v1/auth/change_professor_pass", host))
        .json(&body)
        .send()?;

    let text = response.text()?;
    Ok(text)
}

pub fn close_all(
    host: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    let client = reqwest::blocking::Client::new();

    let response = client
        .post(format!("{}/api/v1/auth/close_all", host))
        .send()?;

    let text = response.text()?;
    Ok(text)
}