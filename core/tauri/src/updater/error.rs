// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use thiserror::Error;

/// All errors that can occur while running the updater.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
  /// IO Errors.
  #[error("`{0}`")]
  Io(#[from] std::io::Error),
  /// Semver Errors.
  #[error("Unable to compare version: {0}")]
  Semver(#[from] semver::Error),
  /// JSON (Serde) Errors.
  #[error("JSON error: {0}")]
  SerdeJson(#[from] serde_json::Error),
  /// Minisign is used for signature validation.
  #[error("Verify signature error: {0}")]
  Minisign(#[from] minisign_verify::Error),
  /// Error with Minisign base64 decoding.
  #[error("Signature decoding error: {0}")]
  Base64(#[from] base64::DecodeError),
  /// UTF8 Errors in signature.
  #[error("The signature {0} could not be decoded, please check if it is a valid base64 string. The signature must be the contents of the `.sig` file generated by the Tauri bundler, as a string.")]
  SignatureUtf8(String),
  /// Tauri utils, mainly extract and file move.
  #[error("Tauri API error: {0}")]
  TauriApi(#[from] crate::api::Error),
  /// Network error.
  #[error("Network error: {0}")]
  Network(String),
  /// Could not fetch a valid response from the server.
  #[error("Could not fetch a valid release JSON from the remote")]
  ReleaseNotFound,
  /// Error building updater.
  #[error("Unable to prepare the updater: {0}")]
  Builder(String),
  /// Error building updater.
  #[error("Unable to extract the new version: {0}")]
  Extract(String),
  /// Updater cannot be executed on this Linux package. Currently the updater is enabled only on AppImages.
  #[error("Cannot run updater on this Linux package. Currently only an AppImage can be updated.")]
  UnsupportedLinuxPackage,
  /// Operating system is not supported.
  #[error("unsupported OS, expected one of `linux`, `darwin` or `windows`.")]
  UnsupportedOs,
  /// Unsupported app architecture.
  #[error(
    "Unsupported application architecture, expected one of `x86`, `x86_64`, `arm` or `aarch64`."
  )]
  UnsupportedArch,
  /// Invalid updater binary format
  #[error("invalid updater binary format")]
  InvalidUpdaterFormat,
  /// The platform was not found on the updater JSON response.
  #[error("the platform `{0}` was not found on the response `platforms` object")]
  TargetNotFound(String),
  /// Triggered when there is NO error and the two versions are equals.
  /// On client side, it's important to catch this error.
  #[error("No updates available")]
  UpToDate,
  /// The updater responded with an invalid signature type.
  #[error("the updater response field `{0}` type is invalid, expected {1} but found {2}")]
  InvalidResponseType(&'static str, &'static str, serde_json::Value),
  /// HTTP error.
  #[error(transparent)]
  Http(#[from] http::Error),
  /// HTTP invalid header value error.
  #[error(transparent)]
  InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
  /// Temp dir is not on same mount mount. This prevents our updater to rename the AppImage to a temp file.
  #[cfg(target_os = "linux")]
  #[error("temp directory is not on the same mount point as the AppImage")]
  TempDirNotOnSameMountPoint,
}

pub type Result<T = ()> = std::result::Result<T, Error>;
