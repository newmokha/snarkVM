// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

mod arithmetic;
mod one;
mod parse;
mod zero;

use snarkvm_console_network::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Field<N: Network> {
    /// The underlying field element.
    field: N::Field,
    /// The input mode for the field element.
    mode: Mode,
}

impl<N: Network> FieldTrait for Field<N> {}

impl<N: Network> Field<N> {
    /// Initializes a new field with the given mode.
    pub const fn new(mode: Mode, field: N::Field) -> Self {
        Self { field, mode }
    }

    /// Returns the mode of the field element.
    pub const fn mode(&self) -> Mode {
        self.mode
    }
}

impl<N: Network> TypeName for Field<N> {
    /// Returns the type name as a string.
    #[inline]
    fn type_name() -> &'static str {
        "field"
    }
}

impl<N: Network> Deref for Field<N> {
    type Target = N::Field;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.field
    }
}
