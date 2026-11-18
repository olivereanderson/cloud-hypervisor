use hypervisor::CpuVendor;
use hypervisor::arch::x86::msr_index::{
    MSR_IA32_UCODE_REV, MSR_IA32_VMX_BASIC, MSR_IA32_VMX_TRUE_PINBASED_CTLS,
};

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
    (
        MSR_IA32_VMX_TRUE_PINBASED_CTLS,
        &[
            ValueDefinition {
                name: "allowed_zero_external_interrupt_exiting",
                bits_range: 0..=0,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_1_2",
                bits_range: 1..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_nmi_exiting",
                bits_range: 3..=3,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_4",
                bits_range: 4..=4,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_virtual_nmis",
                bits_range: 5..=5,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_activate_vmx_preemption_timer",
                bits_range: 6..=6,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_process_posted_interrupts",
                bits_range: 7..=7,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero",
                bits_range: 8..=31,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_external_interrupt_exiting",
                bits_range: 32..=32,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_1_2",
                bits_range: 33..=34,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_nmi_exiting",
                bits_range: 35..=35,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_4",
                bits_range: 36..=36,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_virtual_nmis",
                bits_range: 37..=37,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_activate_vmx__preemption_timer",
                bits_range: 38..=38,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_process_posted_interrupts",
                bits_range: 39..=39,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one",
                bits_range: 40..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    /*




































































































































































































































































    (
        MSR_IA32_VMX_TRUE_PROCBASED_CTLS,
        &[
            ValueDefinition {
                name: "allowed_zero_0_1",
                bits_range: 0..=1,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_interrupt_window_exiting",
                bits_range: 2..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_use_tsc_offsetting",
                bits_range: 3..=3,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_4_6",
                bits_range: 4..=6,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_hlt_exiting",
                bits_range: 7..=7,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_8",
                bits_range: 8..=8,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_invlpg_exiting",
                bits_range: 9..=9,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_mwait_exiting",
                bits_range: 10..=10,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_rdpmc_exiting",
                bits_range: 11..=11,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_rdtsc_exiting",
                bits_range: 12..=12,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_13_14",
                bits_range: 13..=14,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_cr3_load_exiting",
                bits_range: 15..=15,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_cr3_store_exiting",
                bits_range: 16..=16,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_activate_tertiary_controls",
                bits_range: 17..=17,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_18",
                bits_range: 18..=18,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_cr8_load_exiting",
                bits_range: 19..=19,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_cr8_store_exiting",
                bits_range: 20..=20,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_use_tpr_shadow",
                bits_range: 21..=21,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_nmi_window_exiting",
                bits_range: 22..=22,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_mov_dr_exiting",
                bits_range: 23..=23,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_unconditional_i/o_exiting",
                bits_range: 24..=24,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_use_i/o_bitmaps",
                bits_range: 25..=25,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_26",
                bits_range: 26..=26,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_monitor_trap_flag",
                bits_range: 27..=27,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_use_msr_bitmaps",
                bits_range: 28..=28,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_monitor_exiting",
                bits_range: 29..=29,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_pause_exiting",
                bits_range: 30..=30,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_activate_secondary_controls",
                bits_range: 31..=31,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_0_1",
                bits_range: 32..=33,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_interrupt_window_exiting",
                bits_range: 34..=34,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_use_tsc_offsetting",
                bits_range: 35..=35,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_4_6",
                bits_range: 36..=38,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_hlt_exiting",
                bits_range: 39..=39,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_8",
                bits_range: 40..=40,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_invlpg_exiting",
                bits_range: 41..=41,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_mwait_exiting",
                bits_range: 42..=42,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_rdpmc_exiting",
                bits_range: 43..=43,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_rdtsc_exiting",
                bits_range: 44..=44,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_13_14",
                bits_range: 45..=46,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_cr3_load_exiting",
                bits_range: 47..=47,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_cr3_store_exiting",
                bits_range: 48..=48,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_activate_tertiary_controls",
                bits_range: 49..=49,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_18",
                bits_range: 50..=50,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_cr8_load_exiting",
                bits_range: 51..=51,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_cr8_store_exiting",
                bits_range: 52..=52,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_use_tpr_shadow",
                bits_range: 53..=53,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_nmi_window_exiting",
                bits_range: 54..=54,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_mov_dr_exiting",
                bits_range: 55..=55,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_unconditional_i/o_exiting",
                bits_range: 56..=56,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_use_i/o_bitmaps",
                bits_range: 57..=57,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_26",
                bits_range: 58..=58,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_monitor_trap_flag",
                bits_range: 59..=59,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_use_msr_bitmaps",
                bits_range: 60..=60,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_monitor_exiting",
                bits_range: 61..=61,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_pause_exiting",
                bits_range: 62..=62,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_activate_secondary_controls",
                bits_range: 63..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    */
];
