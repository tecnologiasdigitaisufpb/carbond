use crate::{data, errors::APIError};
use log::debug;
use reqwest::{Response, StatusCode};
use uom::si::{f64::MassPerEnergy, mass_per_energy::pound_per_megawatt_hour};

#[derive(Debug)]
pub struct Unauthorized;

#[derive(Debug)]
pub struct Authorized {
    token: String,
}

pub(crate) struct Api<State = Unauthorized> {
    username: String,
    password: String,
    region: String,
    state: State,
}

const INVALID_REGION_TEXT: &str = "You requested data for an unrecognized ba";

impl Api<Unauthorized> {
    /// creates a new unauthorized instance of the API
    pub fn new(username: &str, password: &str, region: &str) -> Api {
        Self {
            username: String::from(username),
            password: String::from(password),
            region: String::from(region),
            state: Unauthorized,
        }
    }

    /// performs a login attempt to obtain a token from the API.
    pub async fn login(&self) -> Result<Api<Authorized>, APIError> {
        debug!("Request login to wattime API.");
        let client = reqwest::Client::new();
        let response = client
            .get("https://api2.watttime.org/v2/login")
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await;

        // handle request errors
        let response = match response {
            Ok(res) => {
                if res.status() == StatusCode::FORBIDDEN {
                    return Err(APIError::InvalidCredentials);
                }
                res
            }
            Err(e) => {
                return Err(APIError::Unhandled(e.to_string()));
            }
        };

        // handle body parsing errors
        let response: data::api::WattTimeLoginResponse = match response.json().await {
            Ok(res) => res,
            Err(e) => return Err(APIError::Deserialze(e.to_string())),
        };
        let api_authorized = Api {
            username: self.username.clone(),
            password: self.password.clone(),
            region: self.region.clone(),
            state: Authorized {
                token: response.token,
            },
        };
        debug!("Logged in successfully.");
        Ok(api_authorized)
    }
}

impl Api<Authorized> {
    /// Requests carbon intensity from WattTime.
    pub async fn get_watt_time_moer(&self) -> Result<MassPerEnergy, APIError> {
        let response = self.request_watt_time().await?;
        // WattTime returns a moer in CO2 lbs/MWh
        let moer: f64 = response.moer.parse().map_err(|_| {
            APIError::Unhandled("Moer value from API could not be parsed to f64.".to_owned())
        })?;
        let carbon_intensity: MassPerEnergy = MassPerEnergy::new::<pound_per_megawatt_hour>(moer);
        Ok(carbon_intensity)
    }

    async fn request_watt_time(&self) -> Result<data::api::WattTimeResponse, APIError> {
        debug!("Requesting moer for region {} from wattime.", self.region);
        let client = reqwest::Client::new();
        let url = format!(
            "https://api2.watttime.org/v2/index?ba={}&style=moer",
            self.region
        );
        let response = client.get(url).bearer_auth(&self.state.token).send().await;
        // handle request errors
        let response = match response {
            Ok(res) => {
                if let StatusCode::OK = res.status() {
                    res
                } else {
                    return Err(self.handle_error(res).await);
                }
            }
            Err(e) => {
                return Err(APIError::Unhandled(e.to_string()));
            }
        };
        // handle body parsing errors
        let response: data::api::WattTimeResponse = response
            .json()
            .await
            .map_err(|op| APIError::Deserialze(op.to_string()))?;
        debug!("Got {:#?}.", response);
        Ok(response)
    }

    async fn handle_error(&self, response: Response) -> APIError {
        match response.status() {
            StatusCode::BAD_REQUEST => {
                let response: data::api::WattTimeError = match response.json().await {
                    Ok(res) => res,
                    Err(e) => return APIError::Deserialze(e.to_string()),
                };
                match response.message.as_str() {
                    INVALID_REGION_TEXT => APIError::InvalidRegion(self.region.clone()),
                    _ => APIError::Unhandled(response.message),
                }
            }
            status => APIError::Unhandled(status.to_string()),
        }
    }
}
