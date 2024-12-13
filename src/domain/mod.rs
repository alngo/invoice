//! # Domain
//!
//! The domain of an invoice generator for a hotel.
//!
//! Core Domain Concept:
//!
//! ## Invoice:
//! It is the core entity and represent the bill for hotel services
//! It contains:
//! - Id: Unique identifier.
//! - Date: Date of creations.
//! - Guest: Associated Guest informations.
//! - Services: List of services billed (rooms, breakfasts, lunch, dinner, etc...)
//! - Total Amount: Calculated total cost
//! - Payment Status: Indicates whether the invoice is paid or unpaid
//!
//! ## Guest:
//! It represent a hotel guest and contains:
//! - Id: Unique identifier
//! - Name: Guest's name
//! - Contact information:
//!   - Address
//!   - Phone
//!   - Email
//!
//! ## Services:
//! It represent a single service on the invoice
//! - Description
//! - Quantity
//! - Price
//! - Payment Status
//!
mod shared;
