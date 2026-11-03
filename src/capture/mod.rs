pub mod grab;
pub mod region;

use image::DynamicImage;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaptureError {
    #[error(transparent)]
    Region(#[from] region::RegionError),
    #[error(transparent)]
    Grab(#[from] grab::GrabError),
}

/// Captures a user-selected screen region into an in-memory DynamicImage.
///
/// Returns `Ok(Some(image))` on success, or `Ok(None)` if the user cancelled region selection.
pub fn capture_screen() -> Result<Option<DynamicImage>, CaptureError> {
    let geom = match region::select_region()? {
        Some(g) => g,
        None => return Ok(None),
    };

    let img = grab::grab_geometry(&geom)?;
    Ok(Some(img))
}
