use loco_rs::prelude::*;

use crate::models::_entities::readings;

/// Render a list view of readings.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<readings::Model>) -> Result<Response> {
    format::render().view(v, "readings/list.html", data!({"items": items}))
}

/// Render a single readings view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &readings::Model) -> Result<Response> {
    format::render().view(v, "readings/show.html", data!({"item": item}))
}

/// Render a readings create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "readings/create.html", data!({}))
}

/// Render a readings edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &readings::Model) -> Result<Response> {
    format::render().view(v, "readings/edit.html", data!({"item": item}))
}
