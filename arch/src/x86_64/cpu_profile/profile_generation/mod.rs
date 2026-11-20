pub mod cpuid;
pub mod msrs;

use std::ops::{BitAnd, BitOr, RangeInclusive, Shl};

use hypervisor::{
    CpuVendor, Hypervisor, HypervisorError,
    arch::x86::{CpuIdEntry, MsrEntry},
};
use thiserror::Error;

use crate::x86_64::cpu_profile::CpuProfile;

pub const AMD_AND_INTEL: &[CpuVendor] = &[CpuVendor::AMD, CpuVendor::Intel];

/// Describes a policy for how a given range of bits within a CPUID output register or
/// feature MSR should be recorded when the CPU profile is generated.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProfilePolicy<T: Copy> {
    /// Write the value extracted from the hardware and the hypervisor into the CPU profile.
    Inherit,
    /// The CPU profile should not alter the value upon applying the CPU profile at runtime.
    Passthrough,
    /// Write the given fixed value into the CPU profile.
    Static(T),
}

/// A description of a range of bits within a CPUID output register or feature MSR.
#[derive(Clone, Debug)]
pub struct ValueDefinition<T: Copy> {
    /// A short name describing the value (e.g. "avx2")
    pub name: &'static str,
    // The range of bits that back the value.
    pub bits_range: RangeInclusive<u8>,
    /// The policy corresponding to this value when building CPU profiles.
    pub policy: ProfilePolicy<T>,
    // The CPU vendors that have documented this value.
    pub vendors: &'static [CpuVendor],
}

impl<T: Copy + PartialEq + Eq> ValueDefinition<T> {
    fn mask(definitions: &[Self], vendor: CpuVendor) -> Option<T>
    where
        T: Shl<u8, Output = T>,
        T: BitOr<Output = T>,
        T: From<u8>,
    {
        definitions
            .iter()
            .filter_map(|def| {
                (def.policy == ProfilePolicy::Passthrough && def.vendors.contains(&vendor))
                    .then_some(&def.bits_range)
                    .map(bit_range_mask)
            })
            .reduce(|acc, next| acc | next)
    }

    fn replacements(definitions: &[Self], value: T, vendor: CpuVendor) -> T
    where
        T: Shl<u8, Output = T>,
        T: BitOr<Output = T>,
        T: From<u8>,
        T: BitAnd<Output = T>,
    {
        definitions
            .iter()
            .filter_map(|def| {
                def.vendors.contains(&vendor).then_some(match def.policy {
                    ProfilePolicy::Inherit => value & bit_range_mask::<T>(&def.bits_range),
                    ProfilePolicy::Static(overwrite_value) => {
                        overwrite_value << *def.bits_range.start()
                    }
                    ProfilePolicy::Passthrough => T::from(0_u8),
                })
            })
            .fold(T::from(0_u8), |acc, partial_value| acc | partial_value)
    }
}

fn bit_range_mask<T>(range: &RangeInclusive<u8>) -> T
where
    T: Shl<u8, Output = T>,
    T: BitOr<Output = T>,
    T: From<u8>,
{
    (range.clone()).fold(T::from(0_u8), |acc, next| acc | ((T::from(1_u8)) << next))
}

fn generate_cpu_profile_with(
    supported_cpuid: Vec<CpuIdEntry>,
    supported_feature_msrs: &[MsrEntry],
    supported_msr_index_list: &[u32],
    vendor: CpuVendor,
) -> CpuProfile {
    CpuProfile {
        cpuid: cpuid::generate_cpuid_profile_data_with(supported_cpuid, vendor),
        msr: msrs::generate_msrs_profile_data_with(
            supported_feature_msrs,
            supported_msr_index_list,
            vendor,
        ),
    }
}

#[derive(Error, Debug)]
#[error("Failed to generate CPU Profile")]
pub struct CpuProfileGenerationError(#[source] pub HypervisorError);

/// Get as many of the supported CPUID entries from the hypervisor as possible.
fn supported_cpuid(hypervisor: &dyn Hypervisor) -> Result<Vec<CpuIdEntry>, HypervisorError> {
    // Try enabling AMX before querying the hypervisor for CPUID.
    hypervisor.enable_amx_state_components().or_else(|e| {
        if let HypervisorError::CouldNotEnableAmxStateComponents(amx_err) = &e
            && !matches!(
                amx_err,
                hypervisor::arch::x86::AmxGuestSupportError::AmxGuestTileRequest { .. }
            )
        {
            // AMX is not supported on the host so we do not need to enable it
            Ok(())
        } else {
            Err(e)
        }
    })?;
    hypervisor.get_supported_cpuid()
}

pub fn generate_cpu_profile(
    hypervisor: &dyn Hypervisor,
) -> Result<CpuProfile, CpuProfileGenerationError> {
    let cpuid = supported_cpuid(hypervisor).map_err(CpuProfileGenerationError)?;
    let feature_msrs = hypervisor
        .get_feature_msrs()
        .map_err(CpuProfileGenerationError)?;
    let msr_index_list = hypervisor
        .get_msr_index_list()
        .map_err(CpuProfileGenerationError)?;
    let vendor = hypervisor.get_cpu_vendor();

    Ok(generate_cpu_profile_with(
        cpuid,
        &feature_msrs,
        &msr_index_list,
        vendor,
    ))
}

#[cfg(test)]
mod unit_tests {
    #[test]
    fn check_profile_generation() {
        todo!()
    }
}
