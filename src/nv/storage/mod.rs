// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

/// This module conatins code that deaals with non volatile storage
/// in the TPM.
///
pub use index::{NvIndexAttributes, NvIndexType};
pub use public::{NvPublic, NvPublicBuilder};

mod index;
mod public;
