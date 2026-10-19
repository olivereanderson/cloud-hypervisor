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
        (
            CpuidParameters {
                leaf: 0x1,
                sub_leaf: 0..=0,
                register: CpuidReg::ECX,
            },
            &[
                ValueDefinition {
                    name: "sse3",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "pclmulqdq",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "dtes64",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "monitor",
                    bits_range: 3..=3,
                    policy: ProfilePolicy::Static(0),
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "ds_cpl",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                // Must exist on the host for the VM to boot.
                // Cloud Hypervisor decides whether to set this at
                // runtime depending on the user provided config.
                ValueDefinition {
                    name: "vmx",
                    bits_range: 5..=5,
                    policy: ProfilePolicy::Passthrough,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "smx",
                    bits_range: 6..=6,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "est",
                    bits_range: 7..=7,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "tm2",
                    bits_range: 8..=8,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "ssse3",
                    bits_range: 9..=9,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "cnxt_id",
                    bits_range: 10..=10,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "sdbg",
                    bits_range: 11..=11,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "fma",
                    bits_range: 12..=12,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "cmpxchg16b",
                    bits_range: 13..=13,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xtpr",
                    bits_range: 14..=14,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "pdcm",
                    bits_range: 15..=15,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "pcid",
                    bits_range: 17..=17,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "dca",
                    bits_range: 18..=18,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "sse4_1",
                    bits_range: 19..=19,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "sse4_2",
                    bits_range: 20..=20,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // Set by Cloud hypervisor
                ValueDefinition {
                    name: "x2apic",
                    bits_range: 21..=21,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "movbe",
                    bits_range: 22..=22,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "popcnt",
                    bits_range: 23..=23,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // Set by Cloud hypervisor
                ValueDefinition {
                    name: "tsc_deadline_timer",
                    bits_range: 24..=24,
                    policy: ProfilePolicy::Passthrough,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "aes",
                    bits_range: 25..=25,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xsave",
                    bits_range: 26..=26,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "osxsave",
                    bits_range: 27..=27,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx",
                    bits_range: 28..=28,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "f16c",
                    bits_range: 29..=29,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "rdrand",
                    bits_range: 30..=30,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // Set by Cloud Hypervisor
                ValueDefinition {
                    name: "guest_status",
                    bits_range: 31..=31,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x1,
                sub_leaf: 0..=0,
                register: CpuidReg::EDX,
            },
            &[
                ValueDefinition {
                    name: "fpu",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "vme",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "de",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "pse",
                    bits_range: 3..=3,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "tsc",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "msr",
                    bits_range: 5..=5,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "pae",
                    bits_range: 6..=6,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "mce",
                    bits_range: 7..=7,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "cx8",
                    bits_range: 8..=8,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // Mutable value reflecting IA32_APIC_BASE[11]
                ValueDefinition {
                    name: "apic",
                    bits_range: 9..=9,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "sep",
                    bits_range: 11..=11,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "mtrr",
                    bits_range: 12..=12,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "pge",
                    bits_range: 13..=13,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "mca",
                    bits_range: 14..=14,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "cmov",
                    bits_range: 15..=15,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "pat",
                    bits_range: 16..=16,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "pse36",
                    bits_range: 17..=17,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "psn",
                    bits_range: 18..=18,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "clfsh",
                    bits_range: 19..=19,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "ds",
                    bits_range: 21..=21,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "acpi",
                    bits_range: 22..=22,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "mmx",
                    bits_range: 23..=23,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "fxsr",
                    bits_range: 24..=24,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "sse",
                    bits_range: 25..=25,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "sse2",
                    bits_range: 26..=26,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "self-snoop",
                    bits_range: 27..=27,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                // Set by Cloud Hypervisor
                ValueDefinition {
                    name: "htt",
                    bits_range: 28..=28,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "tm",
                    bits_range: 29..=29,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "pbe",
                    bits_range: 31..=31,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x2,
                sub_leaf: 0..=0,
                register: CpuidReg::EAX,
            },
            &[ValueDefinition {
                name: "iteration_count",
                bits_range: 0..=7,
                policy: ProfilePolicy::Static(1),
                vendors: &[CpuVendor::Intel],
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0x2,
                sub_leaf: 0..=0,
                register: CpuidReg::EDX,
            },
            &[ValueDefinition {
                name: "descriptor",
                bits_range: 0..=7,
                policy: ProfilePolicy::Static(0xff),
                vendors: &[CpuVendor::Intel],
            }][..],
        ),
    ]
    .to_vec()
}
