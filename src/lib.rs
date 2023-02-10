mod models;

use fiberplane_pdk::{prelude::*, provider_data::ProviderData};
use models::*;
use serde::{Deserialize, Serialize};
use url::Url;

pub const CLOSEST_DISPENSER_QUERY: &str = "x-closest-dispenser";

pub const SHOWCASE_MIME_TYPE: &str = "application/vnd.fiberplane.providers.catnip.closest";

static COMMIT_HASH: &str = env!("VERGEN_GIT_SHA");
static BUILD_TIMESTAMP: &str = env!("VERGEN_BUILD_TIMESTAMP");

#[derive(ConfigSchema, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CatnipConfig {
    #[pdk(label = "Your API endpoint", placeholder = "Please specify a URL")]
    pub endpoint: String,

    #[pdk(label = "I accept the Terms of Use", checked_by_default)]
    pub accept: bool,

    #[pdk(label = "Number of retries if a request fails", max = 10)]
    pub num_retries: u8,
}

#[derive(QuerySchema, Deserialize, Serialize, Debug, Clone)]
struct CatnipClosestQuery {
    #[pdk(
        label = "Latitude (must be a floating point number)",
        placeholder = "52.3740300"
    )]
    pub latitude: String,

    #[pdk(
        label = "Longitude (must be a floating point number)",
        placeholder = "4.8896900"
    )]
    pub longitude: String,
}

pdk_query_types! {
    CLOSEST_DISPENSER_QUERY => {
        label: "Catnip: find closest dispenser",
        handler: closest_dispenser_query_handler(CatnipClosestQuery, CatnipConfig).await,
        supported_mime_types: [CELLS_MIME_TYPE],
    },
    STATUS_QUERY_TYPE => {
        handler: check_status(),
        supported_mime_types: [STATUS_MIME_TYPE]
    }
}

async fn closest_dispenser_query_handler(
    query_data: CatnipClosestQuery,
    config: CatnipConfig,
) -> Result<Blob> {
    let response = fetch_closest_user(
        &config,
        GeoLocation {
            latitude: query_data
                .latitude
                .parse()
                .map_err(|e| Error::Deserialization {
                    message: format!("latitude is an invalid number: {e}"),
                })?,
            longitude: query_data
                .longitude
                .parse()
                .map_err(|e| Error::Deserialization {
                    message: format!("longitude is an invalid number: {e}"),
                })?,
        },
    )
    .await?;
    let cells = match response {
        None => {
            vec![Cell::Text(TextCell::builder()
                .id("result".to_owned())
                .content("No dispenser was found!".to_string())
                .formatting(Formatting::default())
            .build())]
        }
        Some((distance, dispenser)) => {
            vec![Cell::Text(TextCell::builder()
                .id("result".to_owned())
                .content(format!("The closest dispenser to you ({}, {}) is\n{} ({})\n\t{} {}\n\t{} {}",
                    query_data.latitude,
                    query_data.longitude,
                    dispenser.name,
                    distance,
                    dispenser.address.street,
                    dispenser.address.suite,
                    dispenser.address.city,
                    dispenser.address.zipcode
                ))
                .formatting(Formatting::default())
            .build())]
        }
    };

    Cells(cells).to_blob()
}

fn check_status() -> Result<Blob> {
    ProviderStatus::builder()
        .status(Ok(()))
        .version(COMMIT_HASH.to_owned())
        .built_at(BUILD_TIMESTAMP.to_owned())
    .build()
    .to_blob()
}

async fn fetch_users(config: &CatnipConfig) -> Result<Vec<User>> {
    let base_url: Url = config.endpoint.parse().map_err(|e| Error::Config {
        message: format!("Invalid URL in configuration: {e:?}"),
    })?;
    let url = base_url.join("/users").map_err(|e| Error::Config {
        message: format!("Invalid URL in configuration: {e:?}"),
    })?;

    let request = HttpRequest::builder()
        .url(url.to_string())
        .headers(None)
        .method(HttpRequestMethod::Get)
        .body(None)
    .build();
    let response = make_http_request(request).await?;

    serde_json::from_slice(&response.body).map_err(|e| Error::Deserialization {
        message: format!("Could not deserialize payload: {e:?}"),
    })
}

async fn fetch_closest_user(
    config: &CatnipConfig,
    target: GeoLocation,
) -> Result<Option<(f64, User)>> {
    let users = fetch_users(config).await?;
    Ok(closest_user(target, &users).clone())
}
