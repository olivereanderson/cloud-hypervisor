use crate::x86_64::CpuidReg;
use crate::x86_64::cpu_profile::cpuid_adjustments::CpuidParameters;
use crate::x86_64::cpu_profile::profile_generation::cpuid::{ProfilePolicy, ValueDefinition};

pub fn intel_cpuid_definitions() -> Vec<(CpuidParameters, &'static [ValueDefinition])> {
    [(
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
            },
            ValueDefinition {
                name: "pclmulqdq",
                bits_range: 1..=1,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "dtes64",
                bits_range: 2..=2,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "monitor",
                bits_range: 3..=3,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "ds_cpl",
                bits_range: 4..=4,
                policy: ProfilePolicy::Static(0),
            },
            // Must exist on the host for the VM to boot.
            // Cloud Hypervisor decides whether to set this at
            // runtime depending on the user provided config.
            ValueDefinition {
                name: "vmx",
                bits_range: 5..=5,
                policy: ProfilePolicy::Passthrough,
            },
            ValueDefinition {
                name: "smx",
                bits_range: 6..=6,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "est",
                bits_range: 7..=7,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "tm2",
                bits_range: 8..=8,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "ssse3",
                bits_range: 9..=9,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "cnxt_id",
                bits_range: 10..=10,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "sdbg",
                bits_range: 11..=11,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "fma",
                bits_range: 12..=12,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "cmpxchg16b",
                bits_range: 13..=13,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "xtpr",
                bits_range: 14..=14,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "pdcm",
                bits_range: 15..=15,
                policy: ProfilePolicy::Static(0),
            },
            ValueDefinition {
                name: "pcid",
                bits_range: 17..=17,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "dca",
                bits_range: 18..=18,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "sse4_1",
                bits_range: 19..=19,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "sse4_2",
                bits_range: 20..=20,
                policy: ProfilePolicy::Inherit,
            },
            // Set by Cloud hypervisor
            ValueDefinition {
                name: "x2apic",
                bits_range: 21..=21,
                policy: ProfilePolicy::Passthrough,
            },
            ValueDefinition {
                name: "movbe",
                bits_range: 22..=22,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "popcnt",
                bits_range: 23..=23,
                policy: ProfilePolicy::Inherit,
            },
            // Set by Cloud hypervisor
            ValueDefinition {
                name: "tsc_deadline_timer",
                bits_range: 24..=24,
                policy: ProfilePolicy::Passthrough,
            },
            ValueDefinition {
                name: "aes",
                bits_range: 25..=25,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "xsave",
                bits_range: 26..=26,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "osxsave",
                bits_range: 27..=27,
                policy: ProfilePolicy::Passthrough,
            },
            ValueDefinition {
                name: "avx",
                bits_range: 28..=28,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "f16c",
                bits_range: 29..=29,
                policy: ProfilePolicy::Inherit,
            },
            ValueDefinition {
                name: "rdrand",
                bits_range: 30..=30,
                policy: ProfilePolicy::Inherit,
            },
            // Set by Cloud Hypervisor
            ValueDefinition {
                name: "guest_status",
                bits_range: 31..=31,
                policy: ProfilePolicy::Passthrough,
            },
        ][..],
    )]
    .to_vec()
}
