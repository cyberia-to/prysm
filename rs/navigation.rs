//! Navigation state carries public references; selecting a destination grants no authority.
use bevy::prelude::*;
pub use neuron_model::{Destination, NetworkRef, SubjectRef, navigation::Route};
#[derive(Resource, Default, Debug, Clone)]
pub struct ActiveDestination(pub Option<Route>);
#[derive(Debug, Clone)]
pub struct DestinationItem {
    pub route: Route,
    pub icon: String,
    pub label: String,
}
#[derive(Resource, Default, Debug, Clone)]
pub struct DestinationList(pub Vec<DestinationItem>);
/// Compatibility for the published built-in page, not a runtime-origin resolver.
pub fn parse_uri(text: &str) -> Result<Route, neuron_model::IdentityError> {
    // The old short cyb:// page routes have a finite view alias table.
    if let Some(name) = text.strip_prefix("cyb://") {
        let view = match name {
            "body" | "machine" => Some("body"),
            "brain" | "graph" | "mir" => Some("brain"),
            "log" | "com" | "terminal" => Some("log"),
            "robot" | "cell" | "landing" => Some("robot"),
            "sigma" | "money" => Some("sigma"),
            "models" | "mind" => Some("models"),
            "vault" | "secrets" => Some("vault"),
            "memory" | "files" => Some("memory"),
            "oracle" | "blocks" | "explorer" => Some("oracle"),
            _ => None,
        };
        if let Some(view) = view {
            return Route::view(view);
        }
    }
    Route::parse(text, |id| {
        (id == "landing").then(|| Route::view("robot").expect("static view"))
    })
}
