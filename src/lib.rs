//! # gavaconnect
//!
//! A comprehensive Rust SDK for the **Kenya Revenue Authority (KRA) API**.
//!
//! Covers PIN management, tax checker services, customs declarations,
//! return filing, PRN generation, TCC applications, excise licensing,
//! and the full eTIMS OSCU integrator — all behind ergonomic feature flags.
//!
//! ## Quick Start
//!
//! ```no_run
//! use gavaconnect::GavaConnectClient;
//!
//! #[tokio::main]
//! async fn main() -> gavaconnect::Result<()> {
//!     let client = GavaConnectClient::sandbox("CLIENT_ID", "CLIENT_SECRET");
//!     client.authenticate().await?;
//!
//!     let station = client.check_station("A123456789Z").await?;
//!     println!("{:#?}", station);
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! | Feature   | Modules included                                    |
//! |-----------|-----------------------------------------------------|
//! | `full`    | Everything (default)                                |
//! | `auth`    | OAuth2 client-credentials token generation          |
//! | `pin`     | PIN checker (by ID, by PIN) and individual PIN gen  |
//! | `checker` | Station, obligations, exemptions, invoice, TCC      |
//! | `customs` | Customs declarations, import certificates           |
//! | `filing`  | Nil returns, TOT return filing                      |
//! | `prn`     | Withholding tax PRN generation (IT, Rental, VAT)    |
//! | `tcc`     | Tax Compliance Certificate application              |
//! | `excise`  | Excise license checker (by PIN, by number)          |
//! | `etims`   | eTIMS OSCU integrator (branch, data, sales, stock)  |

pub mod client;
pub mod error;

#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "pin")]
pub mod pin;

#[cfg(feature = "checker")]
pub mod checker;

#[cfg(feature = "customs")]
pub mod customs;

#[cfg(feature = "filing")]
pub mod filing;

#[cfg(feature = "prn")]
pub mod prn;

#[cfg(feature = "tcc")]
pub mod tcc;

#[cfg(feature = "excise")]
pub mod excise;

#[cfg(feature = "etims")]
pub mod etims;

// Re-export the primary types at the crate root for convenience.
pub use client::{Environment, GavaConnectClient};
pub use error::{GavaError, Result};
