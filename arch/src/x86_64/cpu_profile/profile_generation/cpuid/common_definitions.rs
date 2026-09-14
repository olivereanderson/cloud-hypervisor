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
    ]
    .to_vec()
}
