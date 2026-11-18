use hypervisor::CpuVendor;
use hypervisor::arch::x86::msr_index::{MSR_IA32_UCODE_REV, MSR_IA32_VMX_BASIC};

use super::{ProfilePolicy, ValueDefinition};

const MSR_AMD64_DE_CFG: u32 = 0xc0011029;
const MSR_IA32_ARCH_CAPABILITIES: u32 = 0x10a;

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
    (
        MSR_IA32_ARCH_CAPABILITIES,
        &[
            ValueDefinition {
                name: "rdcl_no",
                bits_range: 0..=0,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ibrs_all",
                bits_range: 1..=1,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "rsba",
                bits_range: 2..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "skip_l1dfl_vmentry",
                bits_range: 3..=3,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ssb_no",
                bits_range: 4..=4,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "mds_no",
                bits_range: 5..=5,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "if_pschange_mc_no",
                bits_range: 6..=6,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "taa_no",
                bits_range: 8..=8,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "sbdr_ssdp_no",
                bits_range: 13..=13,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "fbsdp_no",
                bits_range: 14..=14,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "psdp_no",
                bits_range: 15..=15,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "fb_clear",
                bits_range: 17..=17,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "rrsba",
                bits_range: 19..=19,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "bhi_no",
                bits_range: 20..=20,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "pbrsb_no",
                bits_range: 24..=24,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "gds_ctrl",
                bits_range: 25..=25,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "gds_no",
                bits_range: 26..=26,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "rfds_no",
                bits_range: 27..=27,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "rfds_clear",
                bits_range: 28..=28,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "pbopt_support",
                bits_range: 32..=32,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "its_no",
                bits_range: 62..=62,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    (
        MSR_IA32_VMX_BASIC,
        &[
            ValueDefinition {
                name: "vmcs_rev_id",
                bits_range: (0..=31),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "region_size",
                bits_range: (32..=44),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "mem_type",
                bits_range: (50..=53),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "vm_exit_info_ins_outs",
                bits_range: (54..=54),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "vmx_ctrls_default_mut",
                bits_range: (55..=55),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "vm_entry_hardware_exceptions",
                bits_range: (56..=56),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
];
