use hypervisor::CpuVendor;

use super::{ProfilePolicy, ValueDefinition};

const MSR_AMD64_DE_CFG: u32 = 0xc0011029;

pub static FEATURE_MSR_DEFINITIONS: &[(u32, &[ValueDefinition])] = &[(
    MSR_AMD64_DE_CFG,
    &[ValueDefinition {
        name: "lfence_serialize",
        bits_range: 1..=1,
        policy: ProfilePolicy::Inherit,
        vendors: &[CpuVendor::AMD],
    }],
)];
