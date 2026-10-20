use std::ops::RangeInclusive;

use hypervisor::CpuVendor;

use crate::x86_64::CpuidReg;
use crate::x86_64::Error::CpuProfileCpuidIncompatibility;
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
        (
            CpuidParameters {
                leaf: 0x4,
                sub_leaf: 0..=u32::MAX,
                register: CpuidReg::EAX,
            },
            &[
                ValueDefinition {
                    name: "cache_type",
                    bits_range: 0..=4,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "cache_level",
                    bits_range: 5..=7,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "cache_self_init",
                    bits_range: 8..=8,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "fully_associative",
                    bits_range: 9..=9,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                // This is always set by cloud hypervisor
                ValueDefinition {
                    name: "num_threads_sharing",
                    bits_range: 14..=25,
                    policy: ProfilePolicy::Passthrough,
                    vendors: &[CpuVendor::Intel],
                },
                // This is always set by cloud hypervisor
                ValueDefinition {
                    name: "num_cores_on_die",
                    bits_range: 26..=31,
                    policy: ProfilePolicy::Passthrough,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x4,
                sub_leaf: 0..=u32::MAX,
                register: CpuidReg::EBX,
            },
            &[
                ValueDefinition {
                    name: "cache_linesize",
                    bits_range: 0..=11,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "cache_npartitions",
                    bits_range: 12..=21,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "cache_nways",
                    bits_range: 22..=31,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x4,
                sub_leaf: 0..=u32::MAX,
                register: CpuidReg::ECX,
            },
            &[ValueDefinition {
                name: "cache_nsets",
                bits_range: 0..=30,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0x4,
                sub_leaf: 0..=u32::MAX,
                register: CpuidReg::EDX,
            },
            &[
                ValueDefinition {
                    name: "wbinvd_rll_no_guarantee",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "ll_inclusive",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "complex_indexing",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x6,
                sub_leaf: 0..=0,
                register: CpuidReg::EAX,
            },
            &[ValueDefinition {
                name: "arat",
                bits_range: 2..=2,
                // The timer is emulated by KVM and thus always always-running :)
                policy: ProfilePolicy::Static(1),
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0x7,
                sub_leaf: RangeInclusive::new(0, 0),
                register: CpuidReg::EAX,
            },
            &[ValueDefinition {
                name: "leaf7_n_subleaves",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0x7,
                sub_leaf: 0..=0,
                register: CpuidReg::EBX,
            },
            &[
                ValueDefinition {
                    name: "fsgsbase",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "tsc_adjust",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // SGX is deprecated so we disable it unconditionally for all CPU profiles
                ValueDefinition {
                    name: "sgx",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "bmi1",
                    bits_range: 3..=3,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // TSX related which is riddled with CVEs.
                ValueDefinition {
                    name: "hle",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx2",
                    bits_range: 5..=5,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                //The KVM docs recommend always setting this (https://docs.kernel.org/virt/kvm/x86/errata.html#kvm-get-supported-cpuid-issues).
                ValueDefinition {
                    name: "fdp_excptn_only",
                    bits_range: 6..=6,
                    policy: ProfilePolicy::Static(1),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "smep",
                    bits_range: 7..=7,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "bmi2",
                    bits_range: 8..=8,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "erms",
                    bits_range: 9..=9,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "invpcid",
                    bits_range: 10..=10,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // This is TSX related which is riddled with CVEs.
                ValueDefinition {
                    name: "rtm",
                    bits_range: 11..=11,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                // The KVM docs recommend always setting this (https://docs.kernel.org/virt/kvm/x86/errata.html#kvm-get-supported-cpuid-issues).
                ValueDefinition {
                    name: "zero_fcs_fds",
                    bits_range: 13..=13,
                    policy: ProfilePolicy::Static(1),
                    vendors: &[CpuVendor::Intel],
                },
                // This has been deprecated
                ValueDefinition {
                    name: "mpx",
                    bits_range: 14..=14,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx512f",
                    bits_range: 16..=16,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx512dq",
                    bits_range: 17..=17,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "rdseed",
                    bits_range: 18..=18,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "adx",
                    bits_range: 19..=19,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "smap",
                    bits_range: 20..=20,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx512ifma",
                    bits_range: 21..=21,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "clflushopt",
                    bits_range: 23..=23,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "clwb",
                    bits_range: 24..=24,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx512pf",
                    bits_range: 26..=26,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx512er",
                    bits_range: 27..=27,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx512cd",
                    bits_range: 28..=28,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "sha",
                    bits_range: 29..=29,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx512bw",
                    bits_range: 30..=30,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx512vl",
                    bits_range: 31..=31,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x7,
                sub_leaf: 0..=0,
                register: CpuidReg::ECX,
            },
            &[
                ValueDefinition {
                    name: "avx512vbmi",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "umip",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "pku",
                    bits_range: 3..=3,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // NOTE: This field is mutable in principle and can be changed by the OS
                ValueDefinition {
                    name: "ospke",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
                // We do not want guests to mess with power adjustments. QEMU also has this policy.
                ValueDefinition {
                    name: "waitpkg",
                    bits_range: 5..=5,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx512_vbmi2",
                    bits_range: 6..=6,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "cet_ss",
                    bits_range: 7..=7,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "gfni",
                    bits_range: 8..=8,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "vaes",
                    bits_range: 9..=9,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "vpclmulqdq",
                    bits_range: 10..=10,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx512_vnni",
                    bits_range: 11..=11,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx512_bitalg",
                    bits_range: 12..=12,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // TDX related.
                ValueDefinition {
                    name: "tme_en",
                    bits_range: 13..=13,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx512_vpopcntdq",
                    bits_range: 14..=14,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "la57",
                    bits_range: 16..=16,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // TSX related which is deprecated
                ValueDefinition {
                    name: "mawau_val_lm",
                    bits_range: 17..=21,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "rdpid",
                    bits_range: 22..=22,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "bus_lock_detect",
                    bits_range: 24..=24,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "cldemote",
                    bits_range: 25..=25,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "movdiri",
                    bits_range: 27..=27,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "movdir64b",
                    bits_range: 28..=28,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                // Currently not exposed by KVM
                ValueDefinition {
                    name: "enqcmd",
                    bits_range: 29..=29,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                // SGX support is deprecated so we disable it unconditionally for CPU profiles
                ValueDefinition {
                    name: "sgx_lc",
                    bits_range: 30..=30,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
                // Currently not supported by KVM
                ValueDefinition {
                    name: "pks",
                    bits_range: 31..=31,
                    policy: ProfilePolicy::Static(0),
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x7,
                sub_leaf: 0..=0,
                register: CpuidReg::EDX,
            },
            &[
                ValueDefinition {
                    name: "fsrm",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx512_vp2intersect",
                    bits_range: 8..=8,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "srdbs_ctrl",
                    bits_range: 9..=9,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "md_clear",
                    bits_range: 10..=10,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "serialize",
                    bits_range: 14..=14,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "hybrid_cpu",
                    bits_range: 15..=15,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "cet_ibt",
                    bits_range: 20..=20,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "amx_bf16",
                    bits_range: 22..=22,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx512_fp16",
                    bits_range: 23..=23,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "amx_tile",
                    bits_range: 24..=24,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "amx_int8",
                    bits_range: 25..=25,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "spec_ctrl",
                    bits_range: 26..=26,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "intel_stibp",
                    bits_range: 27..=27,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "flush_l1d",
                    bits_range: 28..=28,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "arch_capabilities",
                    bits_range: 29..=29,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "spec_ctrl_ssbd",
                    bits_range: 31..=31,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x7,
                sub_leaf: 1..=1,
                register: CpuidReg::EAX,
            },
            &[
                ValueDefinition {
                    name: "sha512",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "sm3",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "sm4",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx_vnni",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx512_bf16",
                    bits_range: 5..=5,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "lass",
                    bits_range: 6..=6,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "cmpccxadd",
                    bits_range: 7..=7,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "fzrm",
                    bits_range: 10..=10,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "fsrs",
                    bits_range: 11..=11,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "fsrc",
                    bits_range: 12..=12,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "fred",
                    bits_range: 17..=17,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "lkgs",
                    bits_range: 18..=18,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "wrmsrns",
                    bits_range: 19..=19,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "amx_fp16",
                    bits_range: 21..=21,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx_ifma",
                    bits_range: 23..=23,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "lam",
                    bits_range: 26..=26,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "rd_wr_msrlist",
                    bits_range: 27..=27,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "invd_disable_post_bios_done",
                    bits_range: 30..=30,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "movrs",
                    bits_range: 31..=31,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x7,
                sub_leaf: 1..=1,
                register: CpuidReg::ECX,
            },
            &[ValueDefinition {
                name: "msr_imm",
                bits_range: 5..=5,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0x7,
                sub_leaf: 1..=1,
                register: CpuidReg::EDX,
            },
            &[
                ValueDefinition {
                    name: "avx_vnni_int8",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "avx_ne_convert",
                    bits_range: 5..=5,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "amx_complex",
                    bits_range: 8..=8,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx_vnni_int16",
                    bits_range: 10..=10,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "prefetchit_0_1",
                    bits_range: 14..=14,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "avx10",
                    bits_range: 19..=19,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "apx_f",
                    bits_range: 21..=21,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0x7,
                sub_leaf: 2..=2,
                register: CpuidReg::EDX,
            },
            &[
                // MSR related
                ValueDefinition {
                    name: "intel_psfd",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                // MSR related
                ValueDefinition {
                    name: "ipred_ctrl",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                // MSR related
                ValueDefinition {
                    name: "rrsba_ctrl",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                // MSR related
                ValueDefinition {
                    name: "ddp_ctrl",
                    bits_range: 3..=3,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                // MSR related
                ValueDefinition {
                    name: "bhi_ctrl",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                // MSR related
                ValueDefinition {
                    name: "mcdt_no",
                    bits_range: 5..=5,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                // MSR related
                ValueDefinition {
                    name: "uclock_disable",
                    bits_range: 6..=6,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        // Leaf 0xb must be set by CHV itself (and do all necessary checks)
        (
            CpuidParameters {
                leaf: 0xb,
                sub_leaf: 0..=u32::MAX,
                register: CpuidReg::EAX,
            },
            &[ValueDefinition {
                name: "x2apic_id_shift",
                bits_range: 0..=4,
                policy: ProfilePolicy::Passthrough,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xb,
                sub_leaf: 0..=u32::MAX,
                register: CpuidReg::EBX,
            },
            &[ValueDefinition {
                name: "domain_lcpus_count",
                bits_range: 0..=15,
                policy: ProfilePolicy::Passthrough,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xb,
                sub_leaf: 0..=u32::MAX,
                register: CpuidReg::ECX,
            },
            &[
                ValueDefinition {
                    name: "domain_nr",
                    bits_range: 0..=7,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "domain_type",
                    bits_range: 8..=15,
                    policy: ProfilePolicy::Passthrough,
                    vendors: AMD_AND_INTEL,
                },
            ][..],
        ),
        // Set by VMM/user provided config
        (
            CpuidParameters {
                leaf: 0xb,
                sub_leaf: 0..=u32::MAX,
                register: CpuidReg::EDX,
            },
            &[ValueDefinition {
                name: "x2apic_id",
                bits_range: 0..=31,
                policy: ProfilePolicy::Passthrough,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 0..=0,
                register: CpuidReg::EAX,
            },
            &[
                ValueDefinition {
                    name: "xcr0_x87",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xcr0_sse",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xcr0_avx",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xcr0_avx512_opmask",
                    bits_range: 5..=5,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xcr0_avx512_zmm_hi256",
                    bits_range: 6..=6,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xcr0_avx512_hi16_zmm",
                    bits_range: 7..=7,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xcr0_pkru",
                    bits_range: 9..=9,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xcr0_tileconfig",
                    bits_range: 17..=17,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "xcr0_tiledata",
                    bits_range: 18..=18,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "xcr0_apx",
                    bits_range: 19..=19,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 0..=0,
                register: CpuidReg::EBX,
            },
            // This is mutable and must be passthrough
            &[ValueDefinition {
                name: "xsave_sz_xcr0_enabled",
                bits_range: 0..=31,
                policy: ProfilePolicy::Passthrough,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 0..=0,
                register: CpuidReg::ECX,
            },
            &[ValueDefinition {
                name: "xsave_sz_max",
                bits_range: 0..=31,
                policy: ProfilePolicy::Passthrough,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 1..=1,
                register: CpuidReg::EAX,
            },
            &[
                ValueDefinition {
                    name: "xsaveopt",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xsavec",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xgetbv",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xsaves",
                    bits_range: 3..=3,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xfd",
                    bits_range: 4..=4,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 1..=1,
                register: CpuidReg::EBX,
            },
            &[ValueDefinition {
                name: "xsave_sz_xcr0_xmms_enabled",
                bits_range: 0..=31,
                policy: ProfilePolicy::Passthrough,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 1..=1,
                register: CpuidReg::ECX,
            },
            &[
                ValueDefinition {
                    name: "xss_cet_u",
                    bits_range: 11..=11,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
                ValueDefinition {
                    name: "xss_cet_p",
                    bits_range: 12..=12,
                    policy: ProfilePolicy::Inherit,
                    vendors: AMD_AND_INTEL,
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 2..=2,
                register: CpuidReg::EAX,
            },
            &[ValueDefinition {
                name: "xsave_sz",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 2..=2,
                register: CpuidReg::EBX,
            },
            &[ValueDefinition {
                name: "xsave_offset",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 2..=2,
                register: CpuidReg::ECX,
            },
            &[
                ValueDefinition {
                    name: "is_xss_bit",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "compacted_xsave_64byte_aligned",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "xfd_faulting",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 5..=5,
                register: CpuidReg::EAX,
            },
            &[ValueDefinition {
                name: "xsave_sz",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 5..=5,
                register: CpuidReg::EBX,
            },
            &[ValueDefinition {
                name: "xsave_offset",
                bits_range: 0..=31,
                policy: ProfilePolicy::Inherit,
                vendors: AMD_AND_INTEL,
            }][..],
        ),
        (
            CpuidParameters {
                leaf: 0xd,
                sub_leaf: 5..=5,
                register: CpuidReg::ECX,
            },
            &[
                ValueDefinition {
                    name: "is_xss_bit",
                    bits_range: 0..=0,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "compacted_xsave_64byte_aligned",
                    bits_range: 1..=1,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
                ValueDefinition {
                    name: "xfd_faulting",
                    bits_range: 2..=2,
                    policy: ProfilePolicy::Inherit,
                    vendors: &[CpuVendor::Intel],
                },
            ][..],
        ),
    ]
    .to_vec()
}
