mod api_doc;
#[cfg(feature = "api-doc")]
mod scalar_handler;

pub use api_doc::ApiDoc;
#[cfg(feature = "api-doc")]
pub use scalar_handler::scalar;