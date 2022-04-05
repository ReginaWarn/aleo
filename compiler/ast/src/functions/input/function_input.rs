// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Identifier, Node, Type};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// A function parameter.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionInputVariable {
    /// The name the parameter is accessible as in the function's body.
    pub identifier: Identifier,
    /// Is it a const parameter?
    pub const_: bool,
    /// Is it a public parameter?
    pub public: bool,
    /// Is it a mutable parameter?
    pub mutable: bool,
    /// What's the parameter's type?
    pub type_: Type,
    /// The parameters span from any annotations to its type.
    pub span: Span,
}

impl FunctionInputVariable {
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.public {
            write!(f, "public ")?;
        } else {
            write!(f, "private ")?;
        }
        if self.const_ {
            write!(f, "const ")?;
        }
        if self.mutable {
            write!(f, "mut ")?;
        }
        write!(f, "{}: ", self.identifier)?;
        write!(f, "{}", self.type_)
    }
}

impl fmt::Display for FunctionInputVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for FunctionInputVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl Node for FunctionInputVariable {
    fn span(&self) -> &Span {
        &self.span
    }

    fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}
