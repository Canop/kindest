//! A typid::Id is strongly typed to avoid misuse of Rust APIs, especially
//! when functions ask for several ids of different types.
//! The typid::Id also prevents the misuse of any string based API, such
//! as Rest or GraphQL, by prefixing the internally used ids with a class
//! prefix.
//!
//! ```
//! use typid::*;
//!
//! // The structs we want to define Id types for are just annotated. The
//! // Identifiable trait is derived.
//!
//! #[derive(Debug, Typid)]
//! #[typid(class="Cust")]
//! pub struct Customer {
//!     // many fields
//! }
//!
//! #[derive(Debug, Typid)]
//! #[typid(class="Cont")]
//! pub struct Contract {
//!     // many fields
//! }
//!
//! // Let's start from an id in the database (we use the string representantion
//! // but typid natively decodes from postgres' Uuid into Id)
//! let customer_db_id = "371c35ec-34d9-4315-ab31-7ea8889a419a";
//!
//! // Now, use it to get our Rust instance of Id:
//! let customer_id: Id<Customer> = Id::from_db_id(customer_db_id).unwrap();
//!
//! // If we communicate (via serde, Display, or directly), we
//! // use the public id
//! let customer_public_id = customer_id.public_id();
//! assert_eq!(&customer_public_id, "Cust_371c35ec-34d9-4315-ab31-7ea8889a419a");
//!
//! // When reading an id withtout prefix, from the db, there was
//! // no type check. It's (almost) OK because we carefully wrote our
//! // queries. But we need a type check when we read from a public id.
//! // Let's try to read our public id as a contract id:
//! let contract_id: Result<Id<Contract>, IdError> = customer_public_id.parse();
//! assert!(contract_id.is_err());
//!
//! // And let's check it's OK as a customer id:
//! let customer_id: Result<Id<Customer>, IdError> = Id::from_public_id(&customer_public_id);
//! assert!(customer_id.is_ok());
//! assert_eq!(customer_id.unwrap().db_id(), "371c35ec-34d9-4315-ab31-7ea8889a419a");
//!
//! // The public id is parsed and checked in a case insensitive way
//! assert_eq!(customer_id, "cust_371c35ec-34d9-4315-ab31-7ea8889a419a".parse());
//! assert_eq!(customer_id, "CUST_371C35EC-34D9-4315-AB31-7EA8889A419A".parse());
//!
//! ```

mod error;
mod id;
mod id_class;
mod ided;
mod identifiable;
mod postgres;

#[cfg(feature = "serde")]
mod id_enum;
#[cfg(feature = "jsonschema")]
mod jsonschema;
mod openapi;
#[cfg(feature = "serde")]
mod serde_serialize;

pub use {
    error::*, id::*, id_class::*, id_enum::*, ided::*, identifiable::*, openapi::*, postgres::*,
    typid_derive::*,
};

#[cfg(feature = "serde")]
pub use crate::serde_serialize::*;

#[cfg(feature = "jsonschema")]
pub use crate::jsonschema::*;
