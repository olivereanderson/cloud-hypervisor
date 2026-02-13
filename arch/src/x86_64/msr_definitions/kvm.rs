// Copyright © 2025 Cyberus Technology GmbH
//
// SPDX-License-Identifier: Apache-2.0
//

//! This module lists KVM defined MSRS. It is currently only used when generating CPU profiles
//! (hence feature gated), but may possibly be extended and utilized for better debug logs in
//! the future.
use super::RegisterAddress;

impl RegisterAddress {
    const MSR_KVM_WALL_CLOCK_NEW: Self = Self(0x4b564d00);
    const MSR_KVM_SYSTEM_TIME_NEW: Self = Self(0x4b564d01);
    const MSR_KVM_WALL_CLOCK: Self = Self(0x11);
    const MSR_KVM_SYSTEM_TIME: Self = Self(0x12);
    const MSR_KVM_ASYNC_PF_EN: Self = Self(0x4b564d02);
    const MSR_KVM_STEAL_TIME: Self = Self(0x4b564d03);
    const MSR_KVM_EOI_EN: Self = Self(0x4b564d04);
    const MSR_KVM_POLL_CONTROL: Self = Self(0x4b564d05);
    const MSR_KVM_ASYNC_PF_INT: Self = Self(0x4b564d06);
    const MSR_KVM_ASYNC_PF_ACK: Self = Self(0x4b564d07);
    const MSR_KVM_MIGRATION_CONTROL: Self = Self(0x4b564d08);
}

/// KVM defined MSRS that CPU profiles may inclide in their permitted MSR definitions.
///
/// This list is (currently) only utilized when generating CPU profiles.
pub(in crate::x86_64) const PROFILE_PERMITTED_KVM_MSRS: [RegisterAddress; 9] = [
    RegisterAddress::MSR_KVM_WALL_CLOCK_NEW,
    RegisterAddress::MSR_KVM_SYSTEM_TIME_NEW,
    RegisterAddress::MSR_KVM_ASYNC_PF_EN,
    RegisterAddress::MSR_KVM_STEAL_TIME,
    RegisterAddress::MSR_KVM_EOI_EN,
    RegisterAddress::MSR_KVM_POLL_CONTROL,
    RegisterAddress::MSR_KVM_ASYNC_PF_INT,
    RegisterAddress::MSR_KVM_ASYNC_PF_ACK,
    RegisterAddress::MSR_KVM_MIGRATION_CONTROL,
];

/// KVM defined MSRS that CPU profiles should not include in their permitted MSR definitions.
///
/// This list helps us detect new MSRs that the profile generation tool may not be
/// aware of, but is not logically necessary for anything beyond that.
pub(in crate::x86_64) const PROFILE_UNPERMITTED_MSRS: [RegisterAddress; 2] = [
    RegisterAddress::MSR_KVM_WALL_CLOCK,
    RegisterAddress::MSR_KVM_SYSTEM_TIME,
];
