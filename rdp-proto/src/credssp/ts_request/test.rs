use lazy_static::lazy_static;

use super::*;
use crate::{
    nego::NegotiationRequestFlags,
    sspi::{AuthIdentity, Credentials},
};

const NTLM_CLIENT_NONCE: [u8; 32] = [
    0x22, 0x10, 0x12, 0xad, 0x12, 0x5c, 0x7a, 0x15, 0xfe, 0xb6, 0x4b, 0x1f, 0xcb, 0x94, 0x83, 0x3a, 0xc5, 0x6f, 0x66,
    0x4c, 0xf3, 0xbc, 0xe7, 0x54, 0x8a, 0x5d, 0x9e, 0x05, 0x0a, 0x46, 0x91, 0xdb,
];

const NTLM_1_PHASE_TS_REQUEST: [u8; 93] = [
    0x30, 0x5b, 0xa0, 0x03, 0x02, 0x01, 0x06, 0xa1, 0x30, 0x30, 0x2e, 0x30, 0x2c, 0xa0, 0x2a, 0x04, 0x28, 0x4e, 0x54,
    0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x01, 0x00, 0x00, 0x00, 0xb7, 0x82, 0x08, 0xe2, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0xb1, 0x1d, 0x00, 0x00, 0x00, 0x0f,
    0xa5, 0x22, 0x04, 0x20, 0x22, 0x10, 0x12, 0xad, 0x12, 0x5c, 0x7a, 0x15, 0xfe, 0xb6, 0x4b, 0x1f, 0xcb, 0x94, 0x83,
    0x3a, 0xc5, 0x6f, 0x66, 0x4c, 0xf3, 0xbc, 0xe7, 0x54, 0x8a, 0x5d, 0x9e, 0x05, 0x0a, 0x46, 0x91, 0xdb,
];
const NTLM_1_PHASE_NEGO_TOKEN: [u8; 40] = [
    0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x01, 0x00, 0x00, 0x00, 0xb7, 0x82, 0x08, 0xe2, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0xb1, 0x1d, 0x00, 0x00,
    0x00, 0x0f,
];

const NTLM_2_PHASE_TS_REQUEST: [u8; 187] = [
    0x30, 0x81, 0xb8, 0xa0, 0x03, 0x02, 0x01, 0x06, 0xa1, 0x81, 0x8c, 0x30, 0x81, 0x89, 0x30, 0x81, 0x86, 0xa0, 0x81,
    0x83, 0x04, 0x81, 0x80, 0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x08,
    0x00, 0x38, 0x00, 0x00, 0x00, 0xb7, 0x82, 0x88, 0xe2, 0xc8, 0x6f, 0x7d, 0x24, 0xd5, 0xb0, 0x65, 0x65, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x00, 0x00, 0x06, 0x01, 0xb1, 0x1d, 0x00,
    0x00, 0x00, 0x0f, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x02, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00,
    0x50, 0x00, 0x43, 0x00, 0x01, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x04, 0x00, 0x08,
    0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x03, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00,
    0x43, 0x00, 0x07, 0x00, 0x08, 0x00, 0x80, 0x26, 0x00, 0xe1, 0x4a, 0xcf, 0xd4, 0x01, 0x00, 0x00, 0x00, 0x00, 0xa5,
    0x22, 0x04, 0x20, 0x22, 0x10, 0x12, 0xad, 0x12, 0x5c, 0x7a, 0x15, 0xfe, 0xb6, 0x4b, 0x1f, 0xcb, 0x94, 0x83, 0x3a,
    0xc5, 0x6f, 0x66, 0x4c, 0xf3, 0xbc, 0xe7, 0x54, 0x8a, 0x5d, 0x9e, 0x05, 0x0a, 0x46, 0x91, 0xdb,
];
const NTLM_2_PHASE_NEGO_TOKEN: [u8; 128] = [
    0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x38, 0x00, 0x00,
    0x00, 0xb7, 0x82, 0x88, 0xe2, 0xc8, 0x6f, 0x7d, 0x24, 0xd5, 0xb0, 0x65, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x40, 0x00, 0x40, 0x00, 0x40, 0x00, 0x00, 0x00, 0x06, 0x01, 0xb1, 0x1d, 0x00, 0x00, 0x00, 0x0f, 0x56,
    0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x02, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00,
    0x01, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x04, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47,
    0x00, 0x50, 0x00, 0x43, 0x00, 0x03, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x07, 0x00,
    0x08, 0x00, 0x80, 0x26, 0x00, 0xe1, 0x4a, 0xcf, 0xd4, 0x01, 0x00, 0x00, 0x00, 0x00,
];

const NTLM_3_PHASE_TS_REQUEST: [u8; 429] = [
    0x30, 0x82, 0x01, 0xa9, 0xa0, 0x03, 0x02, 0x01, 0x06, 0xa1, 0x82, 0x01, 0x48, 0x30, 0x82, 0x01, 0x44, 0x30, 0x82,
    0x01, 0x40, 0xa0, 0x82, 0x01, 0x3c, 0x04, 0x82, 0x01, 0x38, 0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x03,
    0x00, 0x00, 0x00, 0x18, 0x00, 0x18, 0x00, 0x5a, 0x00, 0x00, 0x00, 0xb6, 0x00, 0xb6, 0x00, 0x72, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x58, 0x00, 0x00, 0x00, 0x02, 0x00, 0x02, 0x00, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x5a, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x28, 0x01, 0x00, 0x00, 0x35, 0x82, 0x88, 0xe2, 0x06, 0x01,
    0xb1, 0x1d, 0x00, 0x00, 0x00, 0x0f, 0x85, 0x61, 0x57, 0xd0, 0xe4, 0x87, 0x1c, 0x1a, 0x75, 0xc8, 0x71, 0x22, 0x58,
    0x13, 0xaa, 0x22, 0x61, 0x00, 0x4f, 0x16, 0x46, 0xf0, 0x76, 0x0b, 0x22, 0x06, 0x36, 0xba, 0x46, 0x5f, 0x56, 0x79,
    0x8b, 0x89, 0x99, 0x79, 0x66, 0x9f, 0x64, 0xc9, 0x23, 0x2b, 0xda, 0x0a, 0xa5, 0xec, 0xf1, 0x1b, 0xf8, 0xb3, 0x8d,
    0xf9, 0x23, 0xbd, 0xda, 0x3b, 0x48, 0xbc, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x26, 0x00, 0xe1,
    0x4a, 0xcf, 0xd4, 0x01, 0x99, 0x79, 0x66, 0x9f, 0x64, 0xc9, 0x23, 0x2b, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x08,
    0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x01, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00,
    0x43, 0x00, 0x04, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x03, 0x00, 0x08, 0x00, 0x56,
    0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x07, 0x00, 0x08, 0x00, 0x80, 0x26, 0x00, 0xe1, 0x4a, 0xcf, 0xd4, 0x01,
    0x06, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x1e, 0x00, 0x54, 0x00, 0x45, 0x00, 0x52, 0x00,
    0x4d, 0x00, 0x53, 0x00, 0x52, 0x00, 0x56, 0x00, 0x2f, 0x00, 0x30, 0x00, 0x2e, 0x00, 0x30, 0x00, 0x2e, 0x00, 0x30,
    0x00, 0x2e, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x7f, 0x2b, 0x04, 0x65, 0x22, 0x3a, 0xc5, 0x25, 0x9d, 0x8f, 0x29, 0x30, 0x68, 0xce, 0x6a, 0x9d, 0xa3,
    0x32, 0x04, 0x30, 0x01, 0x00, 0x00, 0x00, 0xca, 0x18, 0x71, 0xef, 0x99, 0xe5, 0x16, 0x21, 0x00, 0x00, 0x00, 0x00,
    0x23, 0x1f, 0xb3, 0xbe, 0xb8, 0x05, 0x19, 0x0c, 0xe1, 0xa2, 0x9d, 0x30, 0x23, 0xd0, 0xd4, 0x21, 0x33, 0xcb, 0x94,
    0x2c, 0x5b, 0xa4, 0x66, 0x77, 0x84, 0x2e, 0x05, 0xd9, 0x4b, 0x46, 0x98, 0xac, 0xa5, 0x22, 0x04, 0x20, 0x22, 0x10,
    0x12, 0xad, 0x12, 0x5c, 0x7a, 0x15, 0xfe, 0xb6, 0x4b, 0x1f, 0xcb, 0x94, 0x83, 0x3a, 0xc5, 0x6f, 0x66, 0x4c, 0xf3,
    0xbc, 0xe7, 0x54, 0x8a, 0x5d, 0x9e, 0x05, 0x0a, 0x46, 0x91, 0xdb,
];
const NTLM_3_PHASE_NEGO_TOKEN: [u8; 312] = [
    0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x03, 0x00, 0x00, 0x00, 0x18, 0x00, 0x18, 0x00, 0x5a, 0x00, 0x00,
    0x00, 0xb6, 0x00, 0xb6, 0x00, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x00, 0x00, 0x00, 0x02, 0x00,
    0x02, 0x00, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x28,
    0x01, 0x00, 0x00, 0x35, 0x82, 0x88, 0xe2, 0x06, 0x01, 0xb1, 0x1d, 0x00, 0x00, 0x00, 0x0f, 0x85, 0x61, 0x57, 0xd0,
    0xe4, 0x87, 0x1c, 0x1a, 0x75, 0xc8, 0x71, 0x22, 0x58, 0x13, 0xaa, 0x22, 0x61, 0x00, 0x4f, 0x16, 0x46, 0xf0, 0x76,
    0x0b, 0x22, 0x06, 0x36, 0xba, 0x46, 0x5f, 0x56, 0x79, 0x8b, 0x89, 0x99, 0x79, 0x66, 0x9f, 0x64, 0xc9, 0x23, 0x2b,
    0xda, 0x0a, 0xa5, 0xec, 0xf1, 0x1b, 0xf8, 0xb3, 0x8d, 0xf9, 0x23, 0xbd, 0xda, 0x3b, 0x48, 0xbc, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x26, 0x00, 0xe1, 0x4a, 0xcf, 0xd4, 0x01, 0x99, 0x79, 0x66, 0x9f, 0x64, 0xc9,
    0x23, 0x2b, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x01,
    0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x04, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00,
    0x50, 0x00, 0x43, 0x00, 0x03, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x07, 0x00, 0x08,
    0x00, 0x80, 0x26, 0x00, 0xe1, 0x4a, 0xcf, 0xd4, 0x01, 0x06, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0a, 0x00,
    0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
    0x00, 0x1e, 0x00, 0x54, 0x00, 0x45, 0x00, 0x52, 0x00, 0x4d, 0x00, 0x53, 0x00, 0x52, 0x00, 0x56, 0x00, 0x2f, 0x00,
    0x30, 0x00, 0x2e, 0x00, 0x30, 0x00, 0x2e, 0x00, 0x30, 0x00, 0x2e, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x2b, 0x04, 0x65, 0x22, 0x3a, 0xc5, 0x25,
    0x9d, 0x8f, 0x29, 0x30, 0x68, 0xce, 0x6a, 0x9d,
];
const NTLM_3_PHASE_PUB_KEY_AUTH: [u8; 48] = [
    0x01, 0x00, 0x00, 0x00, 0xca, 0x18, 0x71, 0xef, 0x99, 0xe5, 0x16, 0x21, 0x00, 0x00, 0x00, 0x00, 0x23, 0x1f, 0xb3,
    0xbe, 0xb8, 0x05, 0x19, 0x0c, 0xe1, 0xa2, 0x9d, 0x30, 0x23, 0xd0, 0xd4, 0x21, 0x33, 0xcb, 0x94, 0x2c, 0x5b, 0xa4,
    0x66, 0x77, 0x84, 0x2e, 0x05, 0xd9, 0x4b, 0x46, 0x98, 0xac,
];

const NTLM_4_PHASE_TS_REQUEST: [u8; 95] = [
    0x30, 0x5d, 0xa0, 0x03, 0x02, 0x01, 0x06, 0xa3, 0x32, 0x04, 0x30, 0x01, 0x00, 0x00, 0x00, 0xce, 0x7a, 0x5a, 0xf4,
    0xf7, 0xe2, 0x57, 0x1b, 0x00, 0x00, 0x00, 0x00, 0xda, 0xcc, 0x16, 0x8f, 0xb2, 0x34, 0x45, 0xed, 0x02, 0x67, 0x31,
    0xc1, 0x63, 0x90, 0xa8, 0x5c, 0x6b, 0x01, 0xbc, 0x1f, 0x8d, 0x93, 0x5e, 0x34, 0xde, 0x66, 0xb9, 0xb3, 0xd7, 0xf5,
    0xa1, 0xab, 0xa5, 0x22, 0x04, 0x20, 0x22, 0x10, 0x12, 0xad, 0x12, 0x5c, 0x7a, 0x15, 0xfe, 0xb6, 0x4b, 0x1f, 0xcb,
    0x94, 0x83, 0x3a, 0xc5, 0x6f, 0x66, 0x4c, 0xf3, 0xbc, 0xe7, 0x54, 0x8a, 0x5d, 0x9e, 0x05, 0x0a, 0x46, 0x91, 0xdb,
];
const NTLM_4_PHASE_PUB_KEY_AUTH: [u8; 48] = [
    0x01, 0x00, 0x00, 0x00, 0xce, 0x7a, 0x5a, 0xf4, 0xf7, 0xe2, 0x57, 0x1b, 0x00, 0x00, 0x00, 0x00, 0xda, 0xcc, 0x16,
    0x8f, 0xb2, 0x34, 0x45, 0xed, 0x02, 0x67, 0x31, 0xc1, 0x63, 0x90, 0xa8, 0x5c, 0x6b, 0x01, 0xbc, 0x1f, 0x8d, 0x93,
    0x5e, 0x34, 0xde, 0x66, 0xb9, 0xb3, 0xd7, 0xf5, 0xa1, 0xab,
];

const NTLM_5_PHASE_TS_REQUEST: [u8; 92] = [
    0x30, 0x5a, 0xa0, 0x03, 0x02, 0x01, 0x06, 0xa2, 0x2f, 0x04, 0x2d, 0x01, 0x00, 0x00, 0x00, 0xe0, 0xba, 0xe3, 0xba,
    0xc2, 0x72, 0x54, 0xe8, 0x01, 0x00, 0x00, 0x00, 0x6e, 0x03, 0x7c, 0x4c, 0x9d, 0x02, 0x4e, 0x5e, 0xeb, 0xa1, 0x9f,
    0xab, 0x60, 0x32, 0x97, 0xcd, 0x95, 0xcf, 0xfe, 0x8f, 0x44, 0x70, 0x79, 0x6e, 0x6c, 0xcd, 0xea, 0x74, 0x6a, 0xa5,
    0x22, 0x04, 0x20, 0x22, 0x10, 0x12, 0xad, 0x12, 0x5c, 0x7a, 0x15, 0xfe, 0xb6, 0x4b, 0x1f, 0xcb, 0x94, 0x83, 0x3a,
    0xc5, 0x6f, 0x66, 0x4c, 0xf3, 0xbc, 0xe7, 0x54, 0x8a, 0x5d, 0x9e, 0x05, 0x0a, 0x46, 0x91, 0xdb,
];
const NTLM_5_PHASE_AUTH_INFO: [u8; 45] = [
    0x01, 0x00, 0x00, 0x00, 0xe0, 0xba, 0xe3, 0xba, 0xc2, 0x72, 0x54, 0xe8, 0x01, 0x00, 0x00, 0x00, 0x6e, 0x03, 0x7c,
    0x4c, 0x9d, 0x02, 0x4e, 0x5e, 0xeb, 0xa1, 0x9f, 0xab, 0x60, 0x32, 0x97, 0xcd, 0x95, 0xcf, 0xfe, 0x8f, 0x44, 0x70,
    0x79, 0x6e, 0x6c, 0xcd, 0xea, 0x74, 0x6a,
];

const NTLM_WITHOUT_CLIENT_NONCE: [u8; 262] = [
    0x30, 0x82, 0x1, 0x2, 0xa0, 0x3, 0x2, 0x1, 0x6, 0xa1, 0x81, 0xfa, 0x30, 0x81, 0xf7, 0x30, 0x81, 0xf4, 0xa0, 0x81,
    0xf1, 0x4, 0x81, 0xee, 0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x0, 0x2, 0x0, 0x0, 0x0, 0x1e, 0x0, 0x1e, 0x0,
    0x38, 0x0, 0x0, 0x0, 0x35, 0x82, 0x8a, 0xe2, 0x6e, 0x80, 0x3b, 0xb7, 0xf8, 0xb9, 0xf6, 0x5c, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x98, 0x0, 0x98, 0x0, 0x56, 0x0, 0x0, 0x0, 0xa, 0x0, 0xab, 0x3f, 0x0, 0x0, 0x0, 0xf, 0x44, 0x0,
    0x45, 0x0, 0x53, 0x0, 0x4b, 0x0, 0x54, 0x0, 0x4f, 0x0, 0x50, 0x0, 0x2d, 0x0, 0x4b, 0x0, 0x4d, 0x0, 0x44, 0x0, 0x54,
    0x0, 0x34, 0x0, 0x32, 0x0, 0x30, 0x0, 0x2, 0x0, 0x1e, 0x0, 0x44, 0x0, 0x45, 0x0, 0x53, 0x0, 0x4b, 0x0, 0x54, 0x0,
    0x4f, 0x0, 0x50, 0x0, 0x2d, 0x0, 0x4b, 0x0, 0x4d, 0x0, 0x44, 0x0, 0x54, 0x0, 0x34, 0x0, 0x32, 0x0, 0x30, 0x0, 0x1,
    0x0, 0x1e, 0x0, 0x44, 0x0, 0x45, 0x0, 0x53, 0x0, 0x4b, 0x0, 0x54, 0x0, 0x4f, 0x0, 0x50, 0x0, 0x2d, 0x0, 0x4b, 0x0,
    0x4d, 0x0, 0x44, 0x0, 0x54, 0x0, 0x34, 0x0, 0x32, 0x0, 0x30, 0x0, 0x4, 0x0, 0x1e, 0x0, 0x44, 0x0, 0x45, 0x0, 0x53,
    0x0, 0x4b, 0x0, 0x54, 0x0, 0x4f, 0x0, 0x50, 0x0, 0x2d, 0x0, 0x4b, 0x0, 0x4d, 0x0, 0x44, 0x0, 0x54, 0x0, 0x34, 0x0,
    0x32, 0x0, 0x30, 0x0, 0x3, 0x0, 0x1e, 0x0, 0x44, 0x0, 0x45, 0x0, 0x53, 0x0, 0x4b, 0x0, 0x54, 0x0, 0x4f, 0x0, 0x50,
    0x0, 0x2d, 0x0, 0x4b, 0x0, 0x4d, 0x0, 0x44, 0x0, 0x54, 0x0, 0x34, 0x0, 0x32, 0x0, 0x30, 0x0, 0x7, 0x0, 0x8, 0x0,
    0x75, 0xc6, 0x84, 0x56, 0xd7, 0xd8, 0xd4, 0x1, 0x0, 0x0, 0x0, 0x0,
];

const NTLM_WITH_ERROR_CODE_TS_REQUEST: [u8; 103] = [
    0x30, 0x65, 0xa0, 0x3, 0x2, 0x1, 0x6, 0xa3, 0x32, 0x4, 0x30, 0x1, 0x0, 0x0, 0x0, 0xd8, 0xb3, 0xc, 0x7e, 0x37, 0x51,
    0x21, 0x84, 0x0, 0x0, 0x0, 0x0, 0x6c, 0x47, 0xc8, 0xe3, 0x2, 0x59, 0x92, 0x19, 0xd1, 0x99, 0xdf, 0xbd, 0xf, 0xf5,
    0x71, 0x61, 0x3a, 0xdf, 0x74, 0x3, 0xef, 0xfc, 0xa5, 0x23, 0x16, 0x23, 0x37, 0x55, 0xff, 0x91, 0x20, 0xd8, 0xa4,
    0x6, 0x2, 0x4, 0xc0, 0x7, 0x0, 0xea, 0xa5, 0x22, 0x4, 0x20, 0xf8, 0x8d, 0xd4, 0xb7, 0x55, 0x9a, 0x94, 0xe5, 0xe8,
    0x7f, 0x2b, 0x2f, 0x94, 0xc2, 0x38, 0x58, 0xa9, 0x84, 0xfc, 0x7, 0x69, 0x7d, 0xf8, 0x57, 0x76, 0xc9, 0x57, 0xa3,
    0x15, 0x12, 0xeb, 0x32,
];

const NTLM_WITH_ERROR_CODE_PUB_KEY_AUTH: [u8; 48] = [
    0x1, 0x0, 0x0, 0x0, 0xd8, 0xb3, 0xc, 0x7e, 0x37, 0x51, 0x21, 0x84, 0x0, 0x0, 0x0, 0x0, 0x6c, 0x47, 0xc8, 0xe3, 0x2,
    0x59, 0x92, 0x19, 0xd1, 0x99, 0xdf, 0xbd, 0xf, 0xf5, 0x71, 0x61, 0x3a, 0xdf, 0x74, 0x3, 0xef, 0xfc, 0xa5, 0x23,
    0x16, 0x23, 0x37, 0x55, 0xff, 0x91, 0x20, 0xd8,
];

const NTLM_WITH_ERROR_CODE_ERROR_CODE: u32 = 0xC007_00EA;

const NTLM_WITH_ERROR_CODE_CLIENT_NONCE: [u8; 32] = [
    0xf8, 0x8d, 0xd4, 0xb7, 0x55, 0x9a, 0x94, 0xe5, 0xe8, 0x7f, 0x2b, 0x2f, 0x94, 0xc2, 0x38, 0x58, 0xa9, 0x84, 0xfc,
    0x7, 0x69, 0x7d, 0xf8, 0x57, 0x76, 0xc9, 0x57, 0xa3, 0x15, 0x12, 0xeb, 0x32,
];

const TS_CREDENTIALS_ONE_SYMBOL_USERNAME_AND_PASSWORD: [u8; 29] = [
    0x30, 0x1b, 0xa0, 0x03, 0x02, 0x01, 0x01, 0xa1, 0x14, 0x04, 0x12, 0x30, 0x10, 0xa0, 0x02, 0x04, 0x00, 0xa1, 0x04,
    0x04, 0x02, 0x61, 0x00, 0xa2, 0x04, 0x04, 0x02, 0x31, 0x00,
];

const TS_CREDENTIALS_STRONG_USERNAME_AND_PASSWORD: [u8; 337] = [
    0x30, 0x82, 0x01, 0x4d, 0xa0, 0x03, 0x02, 0x01, 0x01, 0xa1, 0x82, 0x01, 0x44, 0x04, 0x82, 0x01, 0x40, 0x30, 0x82,
    0x01, 0x3c, 0xa0, 0x02, 0x04, 0x00, 0xa1, 0x7e, 0x04, 0x7c, 0x51, 0x00, 0x57, 0x00, 0x45, 0x00, 0x52, 0x00, 0x54,
    0x00, 0x59, 0x00, 0x55, 0x00, 0x49, 0x00, 0x4f, 0x00, 0x50, 0x00, 0x41, 0x00, 0x53, 0x00, 0x44, 0x00, 0x46, 0x00,
    0x47, 0x00, 0x48, 0x00, 0x4a, 0x00, 0x4b, 0x00, 0x4c, 0x00, 0x5a, 0x00, 0x58, 0x00, 0x43, 0x00, 0x56, 0x00, 0x42,
    0x00, 0x4e, 0x00, 0x4d, 0x00, 0x71, 0x00, 0x77, 0x00, 0x65, 0x00, 0x72, 0x00, 0x74, 0x00, 0x79, 0x00, 0x75, 0x00,
    0x69, 0x00, 0x6f, 0x00, 0x70, 0x00, 0x61, 0x00, 0x73, 0x00, 0x64, 0x00, 0x66, 0x00, 0x67, 0x00, 0x68, 0x00, 0x6a,
    0x00, 0x6b, 0x00, 0x6c, 0x00, 0x7a, 0x00, 0x78, 0x00, 0x63, 0x00, 0x76, 0x00, 0x62, 0x00, 0x6e, 0x00, 0x6d, 0x00,
    0x31, 0x00, 0x32, 0x00, 0x33, 0x00, 0x34, 0x00, 0x35, 0x00, 0x36, 0x00, 0x37, 0x00, 0x38, 0x00, 0x39, 0x00, 0x30,
    0x00, 0xa2, 0x81, 0xb5, 0x04, 0x81, 0xb2, 0x40, 0x00, 0x23, 0x00, 0x24, 0x00, 0x25, 0x00, 0x5e, 0x00, 0x26, 0x00,
    0x2a, 0x00, 0x28, 0x00, 0x29, 0x00, 0x5f, 0x00, 0x2b, 0x00, 0x31, 0x00, 0x32, 0x00, 0x33, 0x00, 0x34, 0x00, 0x35,
    0x00, 0x36, 0x00, 0x37, 0x00, 0x38, 0x00, 0x39, 0x00, 0x30, 0x00, 0x2d, 0x00, 0x3d, 0x00, 0x51, 0x00, 0x57, 0x00,
    0x45, 0x00, 0x52, 0x00, 0x54, 0x00, 0x59, 0x00, 0x55, 0x00, 0x49, 0x00, 0x4f, 0x00, 0x50, 0x00, 0x7b, 0x00, 0x7d,
    0x00, 0x71, 0x00, 0x77, 0x00, 0x65, 0x00, 0x72, 0x00, 0x74, 0x00, 0x79, 0x00, 0x75, 0x00, 0x69, 0x00, 0x6f, 0x00,
    0x70, 0x00, 0x5b, 0x00, 0x5d, 0x00, 0x61, 0x00, 0x73, 0x00, 0x64, 0x00, 0x66, 0x00, 0x67, 0x00, 0x68, 0x00, 0x6a,
    0x00, 0x6b, 0x00, 0x6c, 0x00, 0x3b, 0x00, 0x41, 0x00, 0x53, 0x00, 0x44, 0x00, 0x46, 0x00, 0x47, 0x00, 0x48, 0x00,
    0x4a, 0x00, 0x4b, 0x00, 0x4c, 0x00, 0x3a, 0x00, 0x5c, 0x00, 0x22, 0x00, 0x7c, 0x00, 0x7a, 0x00, 0x78, 0x00, 0x63,
    0x00, 0x76, 0x00, 0x62, 0x00, 0x6e, 0x00, 0x6d, 0x00, 0x2c, 0x00, 0x2e, 0x00, 0x5a, 0x00, 0x58, 0x00, 0x43, 0x00,
    0x56, 0x00, 0x42, 0x00, 0x4e, 0x00, 0x4d, 0x00, 0x3c, 0x00, 0x3e, 0x00, 0x3f, 0x00,
];

const TS_CREDENTIALS_SIMPLE_WITH_USERNAME_AND_DOMAIN_AND_PASSWORD: [u8; 69] = [
    0x30, 0x43, 0xa0, 0x03, 0x02, 0x01, 0x01, 0xa1, 0x3c, 0x04, 0x3a, 0x30, 0x38, 0xa0, 0x0e, 0x04, 0x0c, 0x44, 0x00,
    0x6f, 0x00, 0x6d, 0x00, 0x61, 0x00, 0x69, 0x00, 0x6e, 0x00, 0xa1, 0x12, 0x04, 0x10, 0x55, 0x00, 0x73, 0x00, 0x65,
    0x00, 0x72, 0x00, 0x6e, 0x00, 0x61, 0x00, 0x6d, 0x00, 0x65, 0x00, 0xa2, 0x12, 0x04, 0x10, 0x50, 0x00, 0x61, 0x00,
    0x73, 0x00, 0x73, 0x00, 0x77, 0x00, 0x6f, 0x00, 0x72, 0x00, 0x64, 0x00,
];

const TS_CREDENTIALS_WITH_RESTRICTED_ADMIN_MODE_REQUIRED: [u8; 25] = [
    0x30, 0x17, 0xa0, 0x03, 0x02, 0x01, 0x01, 0xa1, 0x10, 0x04, 0x0e, 0x30, 0x0c, 0xa0, 0x02, 0x04, 0x00, 0xa1, 0x02,
    0x04, 0x00, 0xa2, 0x02, 0x04, 0x00,
];

lazy_static! {
    static ref AUTH_IDENTITY_ONE_SYMBOL_USER_AND_PASSWORD: AuthIdentity =
        Credentials::new(String::from("a"), String::from("1"), None).into();
    static ref AUTH_IDENTITY_STRONG_USERNAME_AND_PASSWORD: AuthIdentity = Credentials::new(
        String::from("QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm1234567890"),
        String::from("@#$%^&*()_+1234567890-=QWERTYUIOP{}qwertyuiop[]asdfghjkl;ASDFGHJKL:\\\"|zxcvbnm,.ZXCVBNM<>?"),
        None
    )
    .into();
    static ref AUTH_IDENTITY_SIMPLE_WITH_USERNAME_AND_DOMAIN_AND_PASSWORD: AuthIdentity = Credentials::new(
        String::from("Username"),
        String::from("Password"),
        Some(String::from("Domain"))
    )
    .into();
    static ref AUTH_IDENTITY_WITH_RESTRICTED_ADMIN_MODE_REQUIRED: AuthIdentity =
        Credentials::new(String::from(""), String::from(""), Some(String::from(""))).into();
}

#[test]
fn ntlm_decode_first_phase_with_nego_token_and_client_nonce() {
    let buffer = NTLM_1_PHASE_TS_REQUEST;

    let expected_version = NLA_VERSION;
    let expected_nego_token = NTLM_1_PHASE_NEGO_TOKEN;
    let expected_client_nonce = NTLM_CLIENT_NONCE;

    let ts_request = TsRequest::from_buffer(&buffer).unwrap();

    assert_eq!(expected_version, ts_request.peer_version.unwrap());
    assert_eq!(ts_request.nego_tokens.unwrap().as_slice(), expected_nego_token.as_ref());
    assert_eq!(ts_request.client_nonce.unwrap(), expected_client_nonce);
    assert!(ts_request.auth_info.is_none());
    assert!(ts_request.pub_key_auth.is_none());
    assert!(ts_request.error_code.is_none());
}

#[test]
fn ntlm_decode_second_phase_with_nego_token_and_client_nonce() {
    let buffer = NTLM_2_PHASE_TS_REQUEST;

    let expected_version = NLA_VERSION;
    let expected_nego_token = NTLM_2_PHASE_NEGO_TOKEN;
    let expected_client_nonce = NTLM_CLIENT_NONCE;

    let ts_request = TsRequest::from_buffer(&buffer).unwrap();

    assert_eq!(expected_version, ts_request.peer_version.unwrap());
    assert_eq!(ts_request.nego_tokens.unwrap().as_slice(), expected_nego_token.as_ref());
    assert_eq!(ts_request.client_nonce.unwrap(), expected_client_nonce);
    assert!(ts_request.auth_info.is_none());
    assert!(ts_request.pub_key_auth.is_none());
    assert!(ts_request.error_code.is_none());
}

#[test]
fn ntlm_decode_third_phase_with_nego_token_and_pub_key_auth_and_client_nonce() {
    let buffer = NTLM_3_PHASE_TS_REQUEST;

    let expected_version = NLA_VERSION;

    let expected_nego_token = NTLM_3_PHASE_NEGO_TOKEN;
    let expected_pub_key_auth = NTLM_3_PHASE_PUB_KEY_AUTH;
    let expected_client_nonce = NTLM_CLIENT_NONCE;

    let ts_request = TsRequest::from_buffer(&buffer).unwrap();

    assert_eq!(expected_version, ts_request.peer_version.unwrap());
    assert_eq!(ts_request.nego_tokens.unwrap().as_slice(), expected_nego_token.as_ref());
    assert_eq!(
        ts_request.pub_key_auth.unwrap().as_slice(),
        expected_pub_key_auth.as_ref()
    );
    assert_eq!(ts_request.client_nonce.unwrap(), expected_client_nonce);
    assert!(ts_request.auth_info.is_none());
    assert!(ts_request.error_code.is_none());
}

#[test]
fn ntlm_decode_fourth_phase_with_pub_key_auth_and_client_nonce() {
    let buffer = NTLM_4_PHASE_TS_REQUEST;

    let expected_version = NLA_VERSION;
    let expected_pub_key_auth = NTLM_4_PHASE_PUB_KEY_AUTH;
    let expected_client_nonce = NTLM_CLIENT_NONCE;

    let ts_request = TsRequest::from_buffer(&buffer).unwrap();

    assert_eq!(expected_version, ts_request.peer_version.unwrap());
    assert_eq!(
        ts_request.pub_key_auth.unwrap().as_slice(),
        expected_pub_key_auth.as_ref()
    );
    assert_eq!(ts_request.client_nonce.unwrap(), expected_client_nonce);
    assert!(ts_request.nego_tokens.is_none());
    assert!(ts_request.auth_info.is_none());
    assert!(ts_request.error_code.is_none());
}

#[test]
fn ntlm_decode_fiveth_phase_with_auth_info_and_client_nonce() {
    let buffer = NTLM_5_PHASE_TS_REQUEST;

    let expected_version = NLA_VERSION;
    let expected_auth_info = NTLM_5_PHASE_AUTH_INFO;
    let expected_client_nonce = NTLM_CLIENT_NONCE;

    let ts_request = TsRequest::from_buffer(&buffer).unwrap();

    assert_eq!(expected_version, ts_request.peer_version.unwrap());
    assert_eq!(ts_request.auth_info.unwrap().as_slice(), expected_auth_info.as_ref());
    assert_eq!(ts_request.client_nonce.unwrap(), expected_client_nonce);
    assert!(ts_request.nego_tokens.is_none());
    assert!(ts_request.pub_key_auth.is_none());
    assert!(ts_request.error_code.is_none());
}

#[test]
fn decode_with_wrong_len() {
    let mut buffer = NTLM_1_PHASE_TS_REQUEST;
    buffer[1] += 1;

    assert!(TsRequest::from_buffer(&buffer).is_err());
}

#[test]
fn decode_with_error_code() {
    let buffer = NTLM_WITH_ERROR_CODE_TS_REQUEST;
    let expected_error = NTLM_WITH_ERROR_CODE_ERROR_CODE;

    let ts_request = TsRequest::from_buffer(&buffer).unwrap();

    assert_eq!(expected_error, ts_request.error_code.unwrap());
    assert!(ts_request.check_error().is_err());
}

#[test]
fn decode_without_client_nonce() {
    let buffer = NTLM_WITHOUT_CLIENT_NONCE;

    let ts_request = TsRequest::from_buffer(&buffer).unwrap();

    assert!(ts_request.client_nonce.is_none());
}

#[test]
fn ntlm_encode_first_phase_with_nego_token_and_client_nonce() {
    let expected_buffer = NTLM_1_PHASE_TS_REQUEST;

    let ts_request = TsRequest {
        nego_tokens: Some(NTLM_1_PHASE_NEGO_TOKEN.to_vec()),
        auth_info: None,
        pub_key_auth: None,
        error_code: None,
        client_nonce: Some(NTLM_CLIENT_NONCE),
        peer_version: Some(NLA_VERSION),
    };

    let ts_request_len = ts_request.buffer_len();
    assert_eq!(ts_request_len as usize, expected_buffer.len());

    let mut buffer = Vec::with_capacity(ts_request_len as usize);
    ts_request.encode_ts_request(&mut buffer).unwrap();

    assert_eq!(buffer.as_slice(), expected_buffer.as_ref());
}

#[test]
fn ntlm_encode_second_phase_with_nego_token_and_client_nonce() {
    let expected_buffer = NTLM_2_PHASE_TS_REQUEST;

    let ts_request = TsRequest {
        nego_tokens: Some(NTLM_2_PHASE_NEGO_TOKEN.to_vec()),
        auth_info: None,
        pub_key_auth: None,
        error_code: None,
        client_nonce: Some(NTLM_CLIENT_NONCE),
        peer_version: Some(NLA_VERSION),
    };

    let ts_request_len = ts_request.buffer_len();
    assert_eq!(ts_request_len as usize, expected_buffer.len());

    let mut buffer = Vec::with_capacity(ts_request_len as usize);
    ts_request.encode_ts_request(&mut buffer).unwrap();

    assert_eq!(buffer.as_slice(), expected_buffer.as_ref());
}

#[test]
fn ntlm_encode_third_phase_with_nego_token_and_pub_key_auth_and_client_nonce() {
    let expected_buffer = NTLM_3_PHASE_TS_REQUEST;

    let ts_request = TsRequest {
        nego_tokens: Some(NTLM_3_PHASE_NEGO_TOKEN.to_vec()),
        auth_info: None,
        pub_key_auth: Some(NTLM_3_PHASE_PUB_KEY_AUTH.to_vec()),
        error_code: None,
        client_nonce: Some(NTLM_CLIENT_NONCE),
        peer_version: Some(NLA_VERSION),
    };

    let ts_request_len = ts_request.buffer_len();
    assert_eq!(ts_request_len as usize, expected_buffer.len());

    let mut buffer = Vec::with_capacity(ts_request_len as usize);
    ts_request.encode_ts_request(&mut buffer).unwrap();

    assert_eq!(buffer.as_slice(), expected_buffer.as_ref());
}

#[test]
fn ntlm_encode_fourth_phase_with_pub_key_auth_and_client_nonce() {
    let expected_buffer = NTLM_4_PHASE_TS_REQUEST;

    let ts_request = TsRequest {
        nego_tokens: None,
        auth_info: None,
        pub_key_auth: Some(NTLM_4_PHASE_PUB_KEY_AUTH.to_vec()),
        error_code: None,
        client_nonce: Some(NTLM_CLIENT_NONCE),
        peer_version: Some(NLA_VERSION),
    };

    let ts_request_len = ts_request.buffer_len();
    assert_eq!(ts_request_len as usize, expected_buffer.len());

    let mut buffer = Vec::with_capacity(ts_request_len as usize);
    ts_request.encode_ts_request(&mut buffer).unwrap();

    assert_eq!(buffer.as_slice(), expected_buffer.as_ref());
}

#[test]
fn ntlm_encode_fiveth_phase_with_auth_info_and_client_nonce() {
    let expected_buffer = NTLM_5_PHASE_TS_REQUEST;

    let ts_request = TsRequest {
        nego_tokens: None,
        auth_info: Some(NTLM_5_PHASE_AUTH_INFO.to_vec()),
        pub_key_auth: None,
        error_code: None,
        client_nonce: Some(NTLM_CLIENT_NONCE),
        peer_version: Some(NLA_VERSION),
    };

    let ts_request_len = ts_request.buffer_len();
    assert_eq!(ts_request_len as usize, expected_buffer.len());

    let mut buffer = Vec::with_capacity(ts_request_len as usize);
    ts_request.encode_ts_request(&mut buffer).unwrap();

    assert_eq!(buffer.as_slice(), expected_buffer.as_ref());
}

#[test]
fn ntlm_encode_with_error_code() {
    let expected_buffer = NTLM_WITH_ERROR_CODE_TS_REQUEST;

    let ts_request = TsRequest {
        nego_tokens: None,
        auth_info: None,
        pub_key_auth: Some(NTLM_WITH_ERROR_CODE_PUB_KEY_AUTH.to_vec()),
        error_code: Some(NTLM_WITH_ERROR_CODE_ERROR_CODE),
        client_nonce: Some(NTLM_WITH_ERROR_CODE_CLIENT_NONCE),
        peer_version: Some(NLA_VERSION),
    };

    let ts_request_len = ts_request.buffer_len();
    assert_eq!(ts_request_len as usize, expected_buffer.len());

    let mut buffer = Vec::with_capacity(ts_request_len as usize);
    ts_request.encode_ts_request(&mut buffer).unwrap();

    assert_eq!(buffer.as_slice(), expected_buffer.as_ref());
}

#[test]
fn decode_ts_credentials_with_one_symbol_user_and_password() {
    let buffer = TS_CREDENTIALS_ONE_SYMBOL_USERNAME_AND_PASSWORD;

    let identity = read_ts_credentials(buffer.as_ref()).unwrap();

    assert_eq!(*AUTH_IDENTITY_ONE_SYMBOL_USER_AND_PASSWORD, identity);
}

#[test]
fn encode_ts_credentials_with_one_symbol_user_and_password() {
    let expected_buffer = TS_CREDENTIALS_ONE_SYMBOL_USERNAME_AND_PASSWORD;

    let identity = &AUTH_IDENTITY_ONE_SYMBOL_USER_AND_PASSWORD;
    let nego_flags = NegotiationRequestFlags::default();
    let buffer = write_ts_credentials(&identity, nego_flags).unwrap();

    assert_eq!(expected_buffer.as_ref(), buffer.as_slice());
}

#[test]
fn decode_ts_credentials_with_strong_user_and_password() {
    let buffer = TS_CREDENTIALS_STRONG_USERNAME_AND_PASSWORD;

    let identity = read_ts_credentials(buffer.as_ref()).unwrap();

    assert_eq!(*AUTH_IDENTITY_STRONG_USERNAME_AND_PASSWORD, identity);
}

#[test]
fn encode_ts_credentials_with_strong_user_and_password() {
    let expected_buffer = TS_CREDENTIALS_STRONG_USERNAME_AND_PASSWORD;

    let identity = &AUTH_IDENTITY_STRONG_USERNAME_AND_PASSWORD;
    let nego_flags = NegotiationRequestFlags::default();
    let buffer = write_ts_credentials(&identity, nego_flags).unwrap();

    assert_eq!(expected_buffer.as_ref(), buffer.as_slice());
}

#[test]
fn decode_ts_credentials_with_simple_username_and_domain_and_password() {
    let buffer = TS_CREDENTIALS_SIMPLE_WITH_USERNAME_AND_DOMAIN_AND_PASSWORD;

    let identity = read_ts_credentials(buffer.as_ref()).unwrap();

    assert_eq!(*AUTH_IDENTITY_SIMPLE_WITH_USERNAME_AND_DOMAIN_AND_PASSWORD, identity);
}

#[test]
fn encode_ts_credentials_with_simple_username_and_domain_and_password() {
    let expected_buffer = TS_CREDENTIALS_SIMPLE_WITH_USERNAME_AND_DOMAIN_AND_PASSWORD;

    let identity = &AUTH_IDENTITY_SIMPLE_WITH_USERNAME_AND_DOMAIN_AND_PASSWORD;
    let nego_flags = NegotiationRequestFlags::default();
    let buffer = write_ts_credentials(&identity, nego_flags).unwrap();

    assert_eq!(expected_buffer.as_ref(), buffer.as_slice());
}

#[test]
fn decode_ts_credentials_with_restricted_admin_mode_required() {
    let buffer = TS_CREDENTIALS_WITH_RESTRICTED_ADMIN_MODE_REQUIRED;

    let identity = read_ts_credentials(buffer.as_ref()).unwrap();

    assert_eq!(*AUTH_IDENTITY_WITH_RESTRICTED_ADMIN_MODE_REQUIRED, identity);
}

#[test]
fn encode_ts_credentials_with_restricted_admin_mode_required() {
    let expected_buffer = TS_CREDENTIALS_WITH_RESTRICTED_ADMIN_MODE_REQUIRED;

    let identity = &AUTH_IDENTITY_WITH_RESTRICTED_ADMIN_MODE_REQUIRED;
    let nego_flags = NegotiationRequestFlags::RESTRICTED_ADMIN_MODE_REQUIED;
    let buffer = write_ts_credentials(&identity, nego_flags).unwrap();

    assert_eq!(expected_buffer.as_ref(), buffer.as_slice());
}

#[test]
fn decode_ts_request_fails_on_incomplete_buffer() {
    let mut buffer = NTLM_3_PHASE_TS_REQUEST.to_vec();
    buffer.split_off(buffer.len() - 1);

    assert!(TsRequest::from_buffer(buffer.as_ref()).is_err());
}

#[test]
fn decode_ts_credentials_fails_on_incomplete_buffer() {
    let mut buffer = TS_CREDENTIALS_WITH_RESTRICTED_ADMIN_MODE_REQUIRED.to_vec();
    buffer.split_off(buffer.len() - 1);

    assert!(read_ts_credentials(buffer.as_slice()).is_err());
}

#[test]
fn buffer_len_correct_returns_len() {
    let buffer = NTLM_3_PHASE_TS_REQUEST;
    let ts_request = TsRequest::from_buffer(buffer.as_ref()).unwrap();

    assert_eq!(buffer.len() as u16, ts_request.buffer_len());
}

#[test]
fn buffer_len_correct_returns_len_with_garbage() {
    let garbage_len = 10;
    let garbage = vec![0x00; garbage_len];

    let mut buffer = NTLM_3_PHASE_TS_REQUEST.to_vec();
    buffer.extend_from_slice(&garbage);

    let ts_request = TsRequest::from_buffer(buffer.as_ref()).unwrap();

    assert_eq!((buffer.len() - garbage_len) as u16, ts_request.buffer_len());
}
