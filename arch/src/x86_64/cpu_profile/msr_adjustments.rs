// Copyright © 2026 Cyberus Technology GmbH
//
// SPDX-License-Identifier: Apache-2.0
//

use hypervisor::arch::x86::MsrEntry;
use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::x86_64::Error;
use crate::x86_64::helpers::{
    deserialize_u32_hex, deserialize_u64_hex, serialize_u32_hex, serialize_u64_hex,
};

/// The register address of an MSR
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RegisterAddress(
    #[serde(
        serialize_with = "serialize_u32_hex",
        deserialize_with = "deserialize_u32_hex"
    )]
    pub u32,
);

/// Used to adjust the value of a Feature MSR.
///
/// Instances of this struct typically adjust MSR values according to the
/// following formula: `msr_value = (self.mask & msr_value) | self.replacements`.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FeatureMsrAdjustment {
    /// Packs values to be placed into the given feature MSR value.
    #[serde(
        serialize_with = "serialize_u64_hex",
        deserialize_with = "deserialize_u64_hex"
    )]
    pub replacements: u64,

    /// Used to zero out the area `replacements` occupy. This mask is not necessarily !replacements, as replacements
    /// may pack values of different types that occupy varying ranges of bits.
    ///
    /// Bit ranges within a feature MSR value that are **not** supposed to be replaced/overwritten should be set in
    /// this mask.
    #[serde(
        serialize_with = "serialize_u64_hex",
        deserialize_with = "deserialize_u64_hex"
    )]
    pub mask: u64,
}

impl FeatureMsrAdjustment {
    /// Adjusts the given `feature_msrs` according to `adjustments`.
    ///
    /// An error is returned if there exists an MSR register address in
    /// `adjustments` without a matching entry in `feature_msrs`.
    ///
    /// Entries in `feature_msrs` whose indices don't match any entry in `adjustments` are tolerated:
    /// CPU profiles overwrite CPUID feature bits and/or the CPU model/family values that should be
    /// checked by the guest before attempting to access MSRs, thus correct guest software should
    /// never access any MSR not known to the CPU profile.
    pub fn adjust_to(
        adjustments: &[(RegisterAddress, FeatureMsrAdjustment)],
        feature_msrs: &[MsrEntry],
    ) -> Result<Vec<MsrEntry>, Error> {
        let mut missing_msr = false;
        let mut output_feature_msrs = Vec::with_capacity(feature_msrs.len());
        for (reg_address, adjustment) in adjustments {
            let Some(entry) = feature_msrs
                .iter()
                .find(|entry| entry.index == reg_address.0)
            else {
                missing_msr = true;
                error!(
                    "Did not find feature based MSR entry for MSR {:#x}",
                    reg_address.0
                );
                continue;
            };
            // Adjust the entry and push it to outputs
            {
                let mut entry = *entry;
                let data = entry.data;
                entry.data = (adjustment.mask & data) | adjustment.replacements;

                debug!(
                    "prepared adjusted MSR feature: register address:={:#x} value:={:#x}, previous value:={data:#x}",
                    entry.index, entry.data
                );
                output_feature_msrs.push(entry);
            }
        }
        if missing_msr {
            Err(Error::CpuProfileMissingMsr)
        } else {
            Ok(output_feature_msrs)
        }
    }
}

/// Data describing MSR adjustments related to a CPU profile.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MsrProfileData {
    /// Describes feature MSR adjustments necessary to become compatible with
    /// the desired target.
    pub adjustments: Vec<(RegisterAddress, FeatureMsrAdjustment)>,
}

#[cfg(test)]
mod unit_tests {
    use hypervisor::arch::x86::MsrEntry;
    use proptest::prelude::*;

    use super::FeatureMsrAdjustment;
    use crate::x86_64::cpu_profile::msr_adjustments::RegisterAddress;

    // These tests will not necessarily use realistic feature MSR addresses or values,
    // they only focus on adjustment logic.
    proptest! {
    #[test]
    fn failure_on_missing_feature_msrs(
        host_feature_msrs in prop::collection::hash_set(any::<(u32,u64)>(), 20),
        feature_msr_removal_idx in  any::<usize>(),
    ) {
        // Let our adjustments just retain whatever we have got in this test
        let adjustments: Vec<(RegisterAddress, FeatureMsrAdjustment)> = host_feature_msrs
            .iter()
            .map(|(idx, _)| {
                (
                    RegisterAddress(*idx),
                    FeatureMsrAdjustment {
                        replacements: 0,
                        mask: u64::MAX,
                    },
                )
            })
            .collect();


        let mut host_feature_msrs: Vec<MsrEntry> = host_feature_msrs
            .into_iter()
            .map(|(idx, data)| MsrEntry {
                index: idx,
                data,
            })
            .collect();


        // Reality check that the current host_feature_msrs are accepted by the artificial profile
        let required_updates = FeatureMsrAdjustment::adjust_to(
            &adjustments,
            &host_feature_msrs,
        )
        .unwrap();

        // Small additional reality check that the required feature MSRs stay the same
        let random_idx = feature_msr_removal_idx % host_feature_msrs.len();
        let random_feature_msr = required_updates[random_idx];
        assert!(
            host_feature_msrs
                .iter()
                .any(|entry| entry.index == random_feature_msr.index
                    && entry.data == random_feature_msr.data)
        );

        // Now we try again, but with one of the host's feature MSRs removed this should not work
        let _= host_feature_msrs.remove(random_idx);
        let _ = FeatureMsrAdjustment::adjust_to(
            &adjustments,
            &host_feature_msrs,
        )
        .unwrap_err();

        }
    }

    // Check that adjustments satisfy our expectations.
    //
    // We only focus on two feature MSRs in this test. Realistically
    // there would be a few more.
    #[test]
    fn required_msr_updates_expected_adjustments() {
        const IA32_BIOS_SIGN_ID: u32 = 0x8b;
        const IA32_VMX_MISC: u32 = 0x485;

        let host_feature_msrs = [
            MsrEntry {
                index: IA32_BIOS_SIGN_ID,
                data: 1 << 42,
            },
            // Extracted from KVM on a Granite Rapids CPU
            MsrEntry {
                index: IA32_VMX_MISC,
                data: 0x20000165,
            },
        ];

        // Example where the profile decides to retain the patch signature ID
        // thus keeping whatever the host has. Zeroing the value out would also
        // make sense since guests should not rely on this MSR anyway.
        let bios_sign_id_msr_adjustment = (
            RegisterAddress(IA32_BIOS_SIGN_ID),
            FeatureMsrAdjustment {
                replacements: 0,
                mask: 0xffffffff00000000,
            },
        );

        // We zero out IA32_VMX_MISC[8] (wait-for-SIPI), but otherwise set exactly
        // what we extracted from a Sapphire Rapids machine
        let vmx_misc_msr_adjustment = (
            RegisterAddress(IA32_VMX_MISC),
            FeatureMsrAdjustment {
                replacements: 0x20000065,
                mask: 0,
            },
        );

        let adjustments = vec![bios_sign_id_msr_adjustment, vmx_misc_msr_adjustment.clone()];

        let required_updates =
            FeatureMsrAdjustment::adjust_to(&adjustments, &host_feature_msrs).unwrap();

        assert_eq!(required_updates.len(), host_feature_msrs.len());

        for feat_msr in required_updates {
            match feat_msr.index {
                IA32_BIOS_SIGN_ID => {
                    // The original value should be retained
                    assert_eq!(feat_msr.data, host_feature_msrs[0].data);
                }
                IA32_VMX_MISC => {
                    // In this case the value should match the replacement
                    assert_eq!(feat_msr.data, vmx_misc_msr_adjustment.1.replacements);
                }
                _ => unreachable!(),
            }
        }
    }
}
