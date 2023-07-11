// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;

impl<N: Network> FromBytes for VectorType<N> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        // Read the element type.
        let element_type = ElementType::read_le(&mut reader)?;
        // Return the vector type.
        Ok(Self::new(element_type))
    }
}

impl<N: Network> ToBytes for VectorType<N> {
    fn write_le<W: Write>(&self, mut writer: W) -> IoResult<()> {
        // Write the element type.
        self.element_type.write_le(&mut writer)
    }
}
