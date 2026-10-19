use std::ops::RangeInclusive;

use hypervisor::CpuVendor;

use crate::x86_64::CpuidReg;
use crate::x86_64::cpu_profile::cpuid_adjustments::CpuidParameters;
// pub mod amd_definitions;
// pub mod common_definitions;
// pub mod intel_definitions;

/// Describes a policy for how a given range of bits within a CPUID output register should be
/// recorded when the CPU profile is generated.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProfilePolicy {
    /// Write the value extracted from the hardware and the hypervisor into the CPU profile.
    Inherit,
    /// The CPU profile should not alter the value upon applying the CPU profile at runtime.
    Passthrough,
    /// Write the given fixed value into the CPU profile.
    Static(u32),
}

/// A description of a range of bits within a CPUID output register.
#[derive(Clone, Debug)]
pub struct ValueDefinition {
    /// A short name describing the value (e.g. "avx2")
    pub name: &'static str,
    // The range of bits that back the value.
    pub bits_range: RangeInclusive<u8>,
    /// The policy corresponding to this value when building CPU profiles.
    pub policy: ProfilePolicy,
    // The CPU vendors that have documented this value.
    pub vendors: &'static [CpuVendor],
}

pub const AMD_AND_INTEL: &'static [CpuVendor] = &[CpuVendor::AMD, CpuVendor::Intel];

pub fn cpuid_definitions() -> Vec<(CpuidParameters, &'static [ValueDefinition])> {
    [
        (
            CpuidParameters {
                leaf: 0,
                sub_leaf: 0..=0,
                register: CpuidReg::EAX,
            },
            &[ValueDefinition {
                name: "max_std_leaf",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0,
                sub_leaf: 0..=0,
                register: CpuidReg::EBX,
            },
            &[ValueDefinition {
                name: "cpu_vendorid_0",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0,
                sub_leaf: 0..=0,
                register: CpuidReg::ECX,
            },
            &[ValueDefinition {
                name: "cpu_vendorid_2",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0,
                sub_leaf: 0..=0,
                register: CpuidReg::EDX,
            },
            &[ValueDefinition {
                name: "cpu_vendorid_1",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0x1,
                sub_leaf: 0..=0,
                register: CpuidReg::EAX,
            },
            &[
                ValueDefinition {
                    name: "stepping",
                    bits_range: 0..=3,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "base_model",
                    bits_range: 4..=7,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "base_family_id",
                    bits_range: 8..=11,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "cpu_type",
                    bits_range: 12..=13,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "ext_model",
                    bits_range: 16..=19,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "ext_family",
                    bits_range: 20..=27,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x1,
                sub_leaf: 0..=0,
                register: CpuidReg::EBX,
            },
            &[
                ValueDefinition {
                    name: "brand_id",
                    bits_range: 0..=7,
                    policy: ProfilePolicy::Static(0),
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "clflush_size",
                    bits_range: 8..=15,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // This is always set by cloud hypervisor
                ValueDefinition {
                    name: "n_logical_cpu",
                    bits_range: 16..=23,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
                // This is always set by cloud hypervisor
                ValueDefinition {
                    name: "local_apic_id",
                    bits_range: 24..=31,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
            ][..],
        ),
    ]
    .to_vec()
}
