use crate::x86_64::CpuidReg;
use crate::x86_64::cpu_profile::cpuid_adjustments::CpuidParameters;
use crate::x86_64::cpu_profile::profile_generation::cpuid::{ProfilePolicy, ValueDefinition};

pub fn commond_cpuid_definitions() -> Vec<(CpuidParameters, &'static [ValueDefinition])> {
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
                },
                ValueDefinition {
                    name: "base_model",
                    bits_range: 4..=7,
                    policy: ProfilePolicy::Inherit,
                },
                ValueDefinition {
                    name: "base_family_id",
                    bits_range: 8..=11,
                    policy: ProfilePolicy::Inherit,
                },
                ValueDefinition {
                    name: "cpu_type",
                    bits_range: 12..=13,
                    policy: ProfilePolicy::Inherit,
                },
                ValueDefinition {
                    name: "ext_model",
                    bits_range: 16..=19,
                    policy: ProfilePolicy::Inherit,
                },
                ValueDefinition {
                    name: "ext_family",
                    bits_range: 20..=27,
                    policy: ProfilePolicy::Inherit,
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
                },
                ValueDefinition {
                    name: "clflush_size",
                    bits_range: 8..=15,
                    policy: ProfilePolicy::Inherit,
                },
                // This is always set by cloud hypervisor
                ValueDefinition {
                    name: "n_logical_cpu",
                    bits_range: 16..=23,
                    policy: ProfilePolicy::Passthrough,
                },
                // This is always set by cloud hypervisor
                ValueDefinition {
                    name: "local_apic_id",
                    bits_range: 24..=31,
                    policy: ProfilePolicy::Passthrough,
                },
            ][..],
        ),
    ]
    .to_vec()
}
