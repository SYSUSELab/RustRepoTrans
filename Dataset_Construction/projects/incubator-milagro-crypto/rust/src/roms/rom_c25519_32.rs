/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use super::big::NLEN;
use crate::arch::Chunk;
use crate::types::{CurvePairingType, CurveType, ModType, SexticTwist, SignOfX};

// Base Bits= 29
// Curve25519 Modulus
pub const MODULUS: [Chunk; NLEN] = [
    0x1FFFFFED, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF, 0x1FFFFFFF,
    0x7FFFFF,
];
pub const R2MODP: [Chunk; NLEN] = [0x169000, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const MCONST: Chunk = 0x13;

// c25519 Curve
pub const CURVE_COF_I: isize = 8;
pub const CURVE_A: isize = 486662;
pub const CURVE_B_I: isize = 0;
pub const CURVE_COF: [Chunk; NLEN] = [0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_B: [Chunk; NLEN] = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x1CF5D3ED, 0x9318D2, 0x1DE73596, 0x1DF3BD45, 0x14D, 0x0, 0x0, 0x0, 0x100000,
];
pub const CURVE_GX: [Chunk; NLEN] = [0x9, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
pub const CURVE_GY: [Chunk; NLEN] = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

pub const MODBYTES: usize = 32;
pub const BASEBITS: usize = 29;

pub const MODBITS: usize = 255;
pub const MOD8: usize = 5;
pub const MODTYPE: ModType = ModType::PseudoMersenne;
pub const SH: usize = 6;

pub const CURVETYPE: CurveType = CurveType::Montgomery;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::Not;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::Not;
pub const ATE_BITS: usize = 0;
pub const SIGN_OF_X: SignOfX = SignOfX::Not;
pub const HASH_TYPE: usize = 32;
pub const AESKEY: usize = 16;
