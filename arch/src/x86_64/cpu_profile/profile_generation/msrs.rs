use hypervisor::CpuVendor;
use hypervisor::arch::x86::msr_index::MSR_IA32_UCODE_REV;

use super::ProfilePolicy;

type ValueDefinition = super::ValueDefinition<u64>;

const MSR_AMD64_DE_CFG: u32 = 0xc0011029;

pub static FEATURE_MSR_DEFINITIONS: &[(u32, &[ValueDefinition])] = &[
    (
        MSR_AMD64_DE_CFG,
        &[ValueDefinition {
            name: "lfence_serialize",
            bits_range: 1..=1,
            policy: ProfilePolicy::Inherit,
            vendors: &[CpuVendor::AMD],
        }],
    ),
    (
        MSR_IA32_UCODE_REV,
        &[ValueDefinition {
            name: "patch_sign_id",
            bits_range: 0..=31,
            policy: ProfilePolicy::Inherit,
            vendors: &[CpuVendor::AMD],
        }],
    ),
    (
        MSR_IA32_UCODE_REV,
        &[ValueDefinition {
            name: "patch_sign_id",
            bits_range: 32..=63,
            policy: ProfilePolicy::Inherit,
            vendors: &[CpuVendor::Intel],
        }],
    ),
];
