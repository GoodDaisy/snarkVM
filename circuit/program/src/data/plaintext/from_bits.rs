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

impl<A: Aleo> FromBits for Plaintext<A> {
    type Boolean = Boolean<A>;

    /// Initializes a new plaintext from a list of little-endian bits *without* trailing zeros.
    fn from_bits_le(bits_le: &[Boolean<A>]) -> Self {
        let mut bits = bits_le.iter().cloned();

        // Helper function to get the next n bits as a slice.
        let mut next_bits = |n: usize| -> Vec<Boolean<A>> {
            let bits: Vec<_> = bits.by_ref().take(n).collect();
            if bits.len() < n {
                return A::halt("Insufficient bits");
            }
            bits
        };

        let variant = next_bits(2).iter().map(|b| b.eject_value()).collect::<Vec<_>>();

        // Literal
        if variant == [false, false] {
            let literal_variant = U8::from_bits_le(&next_bits(8));
            let literal_size = U16::from_bits_le(&next_bits(16)).eject_value();
            let literal = Literal::from_bits_le(&literal_variant, &next_bits(*literal_size as usize));

            // Cache the plaintext bits, and return the literal.
            Self::Literal(literal, OnceCell::with_value(bits_le.to_vec()))
        }
        // Struct
        else if variant == [false, true] {
            let num_members = U8::from_bits_le(&next_bits(8)).eject_value();

            let mut members = IndexMap::with_capacity(*num_members as usize);
            for _ in 0..*num_members {
                let identifier_size = U8::from_bits_le(&next_bits(8)).eject_value();
                let identifier = Identifier::from_bits_le(&next_bits(*identifier_size as usize));

                let member_size = U16::from_bits_le(&next_bits(16)).eject_value();
                let value = Plaintext::from_bits_le(&next_bits(*member_size as usize));

                members.insert(identifier, value);
            }

            // Cache the plaintext bits, and return the struct.
            Self::Struct(members, OnceCell::with_value(bits_le.to_vec()))
        }
        // Unknown variant.
        else {
            A::halt("Unknown plaintext variant.")
        }
    }

    /// Initializes a new plaintext from a list of big-endian bits *without* trailing zeros.
    fn from_bits_be(bits_be: &[Boolean<A>]) -> Self {
        let mut bits = bits_be.iter().cloned();

        // Helper function to get the next n bits as a slice.
        let mut next_bits = |n: usize| -> Vec<Boolean<A>> {
            let bits: Vec<_> = bits.by_ref().take(n).collect();
            if bits.len() < n {
                return A::halt("Insufficient bits.");
            }
            bits
        };

        let variant = next_bits(2).iter().map(|b| b.eject_value()).collect::<Vec<_>>();

        // Literal
        if variant == [false, false] {
            let literal_variant = U8::from_bits_be(&next_bits(8));
            let literal_size = U16::from_bits_be(&next_bits(16)).eject_value();
            let literal = Literal::from_bits_be(&literal_variant, &next_bits(*literal_size as usize));

            // Cache the plaintext bits, and return the literal.
            Self::Literal(literal, OnceCell::with_value(bits_be.to_vec()))
        }
        // Struct
        else if variant == [false, true] {
            let num_members = U8::from_bits_be(&next_bits(8)).eject_value();

            let mut members = IndexMap::with_capacity(*num_members as usize);
            for _ in 0..*num_members {
                let identifier_size = U8::from_bits_be(&next_bits(8)).eject_value();
                let identifier = Identifier::from_bits_be(&next_bits(*identifier_size as usize));

                let member_size = U16::from_bits_be(&next_bits(16)).eject_value();
                let value = Plaintext::from_bits_be(&next_bits(*member_size as usize));

                members.insert(identifier, value);
            }

            // Cache the plaintext bits, and return the struct.
            Self::Struct(members, OnceCell::with_value(bits_be.to_vec()))
        }
        // Unknown variant.
        else {
            A::halt("Unknown plaintext variant.")
        }
    }
}
