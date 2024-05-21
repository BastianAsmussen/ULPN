use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

// Implementation details: https://viden.stil.dk/display/OFFSKOLELOGIN/Implementering+af+tjeneste#tab-OIDC+-+Login+endpoint+og+parametre

