use std::ops::RangeInclusive;

use hypervisor::arch::x86::MsrEntry;
use hypervisor::arch::x86::msr_index::{
    MSR_CORE_PERF_FIXED_CTR0, MSR_CORE_PERF_GLOBAL_CTRL, MSR_F15H_PERF_CTL, MSR_F15H_PERF_CTR,
    MSR_IA32_BNDCFGS, MSR_IA32_DS_AREA, MSR_IA32_PEBS_ENABLE, MSR_IA32_PERF_CAPABILITIES,
    MSR_IA32_PERFCTR0, MSR_IA32_RTIT_ADDR0_A, MSR_IA32_RTIT_ADDR3_B, MSR_IA32_RTIT_CR3_MATCH,
    MSR_IA32_RTIT_CTL, MSR_IA32_RTIT_OUTPUT_BASE, MSR_IA32_RTIT_OUTPUT_MASK, MSR_IA32_UCODE_REV,
    MSR_IA32_VMX_BASIC, MSR_IA32_VMX_CR0_FIXED0, MSR_IA32_VMX_CR4_FIXED0,
    MSR_IA32_VMX_EPT_VPID_CAP, MSR_IA32_VMX_MISC, MSR_IA32_VMX_PROCBASED_CTLS2,
    MSR_IA32_VMX_TRUE_ENTRY_CTLS, MSR_IA32_VMX_TRUE_EXIT_CTLS, MSR_IA32_VMX_TRUE_PINBASED_CTLS,
    MSR_IA32_VMX_TRUE_PROCBASED_CTLS, MSR_IA32_VMX_VMCS_ENUM, MSR_IA32_VMX_VMFUNC, MSR_K7_EVNTSEL0,
    MSR_K7_PERFCTR3, MSR_P6_EVNTSEL0,
};
use hypervisor::{CpuVendor, Hypervisor};

use super::ProfilePolicy;
use crate::x86_64::cpu_profile::msr_adjustments::{
    FeatureMsrAdjustment, MsrProfileData, RegisterAddress,
};

type ValueDefinition = super::ValueDefinition<u64>;

const MSR_AMD64_DE_CFG: u32 = 0xc0011029;
const MSR_IA32_ARCH_CAPABILITIES: u32 = 0x10a;
const MSR_IA32_VMX_EXIT_CTLS2: u32 = 0x493;

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
    (
        MSR_IA32_VMX_TRUE_EXIT_CTLS,
        &[
            ValueDefinition {
                name: "allowed_zero_0_1",
                bits_range: 0..=1,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_save_debug_controls",
                bits_range: 2..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_3_8",
                bits_range: 3..=8,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_host_address_space_size",
                bits_range: 9..=9,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_10_11",
                bits_range: 10..=11,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_ia32_perf_global_ctrl",
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
                name: "allowed_zero_acknowledge_interrupt_o_exit",
                bits_range: 15..=15,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_16_17",
                bits_range: 16..=17,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_save_ia32_pat",
                bits_range: 18..=18,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_ia32_pat",
                bits_range: 19..=19,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_save_ia32_efer",
                bits_range: 20..=20,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_ia32_efer",
                bits_range: 21..=21,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_save_vmx_preemption_timer_value",
                bits_range: 22..=22,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_clear_ia32_bndcfgs",
                bits_range: 23..=23,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_conceal_vmx_from_pt",
                bits_range: 24..=24,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_clear_ia32_rtit_ctl",
                bits_range: 25..=25,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_clear_ia32_lbr_ctl",
                bits_range: 26..=26,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_clear_uinv",
                bits_range: 27..=27,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_cet_state",
                bits_range: 28..=28,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_pkrs",
                bits_range: 29..=29,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_save_ia32_perf_global_ctl",
                bits_range: 30..=30,
                policy: ProfilePolicy::Static(0),
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
                name: "allowed_one_save_debug_controls",
                bits_range: 34..=34,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_3_8",
                bits_range: 35..=40,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_host_address_space_size",
                bits_range: 41..=41,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_10_11",
                bits_range: 42..=43,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_ia32_perf_global_ctrl",
                bits_range: 44..=44,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_13_14",
                bits_range: 45..=46,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_acknowledge_interrupt_o_exit",
                bits_range: 47..=47,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_16_17",
                bits_range: 48..=49,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_save_ia32_pat",
                bits_range: 50..=50,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_ia32_pat",
                bits_range: 51..=51,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_save_ia32_efer",
                bits_range: 52..=52,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_ia32_efer",
                bits_range: 53..=53,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_save_vmx_preemption_timer_value",
                bits_range: 54..=54,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_clear_ia32_bndcfgs",
                bits_range: 55..=55,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_conceal_vmx_from_pt",
                bits_range: 56..=56,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_clear_ia32_rtit_ctl",
                bits_range: 57..=57,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_clear_ia32_lbr_ctl",
                bits_range: 58..=58,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_clear_uinv",
                bits_range: 59..=59,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_cet_state",
                bits_range: 60..=60,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_pkrs",
                bits_range: 61..=61,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_save_ia32_perf_global_ctl",
                bits_range: 62..=62,
                policy: ProfilePolicy::Static(0),
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
    (
        MSR_IA32_VMX_TRUE_ENTRY_CTLS,
        &[
            ValueDefinition {
                name: "allowed_zero_0_1",
                bits_range: 0..=1,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_debug_controls",
                bits_range: 2..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_3_8",
                bits_range: 3..=8,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_ia_32e_mode_gues",
                bits_range: 9..=9,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_entry_to_smm",
                bits_range: 10..=10,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_deactivate_dual__monitor_treatment",
                bits_range: 11..=11,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_12",
                bits_range: 12..=12,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_ia32_perf_global_ctrl",
                bits_range: 13..=13,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_ia32_pat",
                bits_range: 14..=14,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_ia32_efer",
                bits_range: 15..=15,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_ia32_bndcfgs",
                bits_range: 16..=16,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_conceal_vmx_from_pt",
                bits_range: 17..=17,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_ia32_rtit_ctl",
                bits_range: 18..=18,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_uinv",
                bits_range: 19..=19,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_cet_state",
                bits_range: 20..=20,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_guest_ia32_lbr_ctl",
                bits_range: 21..=21,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_load_pkrs",
                bits_range: 22..=22,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_23_24",
                bits_range: 23..=24,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_allow_seam_guest_telemetry",
                bits_range: 25..=25,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_26_31",
                bits_range: 26..=31,
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
                name: "allowed_one_load_debug_controls",
                bits_range: 34..=34,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_3_8",
                bits_range: 35..=40,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_ia_32e_mode_gues",
                bits_range: 41..=41,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_entry_to_smm",
                bits_range: 42..=42,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_deactivate_dual__monitor_treatment",
                bits_range: 43..=43,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_12",
                bits_range: 44..=44,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_ia32_perf_global_ctrl",
                bits_range: 45..=45,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_ia32_pat",
                bits_range: 46..=46,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_ia32_efer",
                bits_range: 47..=47,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_ia32_bndcfgs",
                bits_range: 48..=48,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_conceal_vmx_from_pt",
                bits_range: 49..=49,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_ia32_rtit_ctl",
                bits_range: 50..=50,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_uinv",
                bits_range: 51..=51,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_cet_state",
                bits_range: 52..=52,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_guest_ia32_lbr_ctl",
                bits_range: 53..=53,
                policy: ProfilePolicy::Static(0),
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_load_pkrs",
                bits_range: 54..=54,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_23_24",
                bits_range: 55..=56,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_allow_seam_guest_telemetry",
                bits_range: 57..=57,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_26_31",
                bits_range: 58..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    (
        MSR_IA32_VMX_MISC,
        &[
            ValueDefinition {
                name: "VMX_PREEMPTION_TSC_REL",
                bits_range: (0..=4),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "IA32_EFER.LMA_STORE",
                bits_range: (5..=5),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "HLT_STATE",
                bits_range: (6..=6),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "SHUTDOWN_STATE",
                bits_range: (7..=7),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "WAIT_FOR_SIPI__STATE",
                bits_range: (8..=8),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "VMX_NUM_CR3",
                bits_range: (16..=24),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "MAX_MSR_STORE_LISTS",
                bits_range: (25..=27),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "VM_WRITE_EXIT_FIELDS",
                bits_range: (29..=29),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "VM_ENTRY_INJECTION",
                bits_range: (30..=30),
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "MSEG_REV_ID",
                bits_range: 32..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    (
        MSR_IA32_VMX_CR0_FIXED0,
        &[
            ValueDefinition {
                name: "cr0.pe",
                bits_range: 0..=0,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.mp",
                bits_range: 1..=1,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.em",
                bits_range: 2..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.ts",
                bits_range: 3..=3,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.et",
                bits_range: 4..=4,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.ne",
                bits_range: 5..=5,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ia32_vmx_cr0_fixed1_reserved_6_15",
                bits_range: 6..=15,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.wp",
                bits_range: 16..=16,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ia32_vmx_cr0_fixed1_reserved_17_17",
                bits_range: 17..=17,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.am",
                bits_range: 18..=18,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ia32_vmx_cr0_fixed1_reserved_19_28",
                bits_range: 19..=28,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.nw",
                bits_range: 29..=29,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.cd",
                bits_range: 30..=30,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr0.pg",
                bits_range: 31..=31,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ia32_vmx_cr0_fixed1_reserved_32_63",
                bits_range: 32..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    (
        MSR_IA32_VMX_CR4_FIXED0,
        &[
            ValueDefinition {
                name: "cr4.vme",
                bits_range: 0..=0,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.pvi",
                bits_range: 1..=1,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.tsd",
                bits_range: 2..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.de",
                bits_range: 3..=3,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.pse",
                bits_range: 4..=4,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.pae",
                bits_range: 5..=5,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.mce",
                bits_range: 6..=6,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.pge",
                bits_range: 7..=7,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.osfxsr",
                bits_range: 9..=9,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.osxmmexcpt",
                bits_range: 10..=10,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.umip",
                bits_range: 11..=11,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.la57",
                bits_range: 12..=12,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.vmxe",
                bits_range: 13..=13,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.reserved_15",
                bits_range: 15..=15,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.fsgsbase",
                bits_range: 16..=16,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.pcide",
                bits_range: 17..=17,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.osxsave",
                bits_range: 18..=18,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.smep",
                bits_range: 20..=20,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.smap",
                bits_range: 21..=21,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.pke",
                bits_range: 22..=22,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.cet",
                bits_range: 23..=23,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.reserved_26",
                bits_range: 26..=26,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.lass",
                bits_range: 27..=27,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "cr4.lam_sup",
                bits_range: 28..=28,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ia32_vmx_cr4_fixed0",
                bits_range: 29..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    (
        MSR_IA32_VMX_VMCS_ENUM,
        &[ValueDefinition {
            name: "max_index",
            bits_range: 1..=9,
            policy: ProfilePolicy::Inherit,
            vendors: &[CpuVendor::Intel],
        }],
    ),
    (
        MSR_IA32_VMX_PROCBASED_CTLS2,
        &[
            ValueDefinition {
                name: "allowed_zero_virtualize_apic_accesses",
                bits_range: 0..=0,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_ept",
                bits_range: 1..=1,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_descriptor_table_exiting",
                bits_range: 2..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_rdtscp",
                bits_range: 3..=3,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_virtualize_x2apic_mode",
                bits_range: 4..=4,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_vpid",
                bits_range: 5..=5,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_wbinvd_exiting",
                bits_range: 6..=6,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_unrestricted_guest",
                bits_range: 7..=7,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_apic_register_virtualization",
                bits_range: 8..=8,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_virtual_interrupt_delivery",
                bits_range: 9..=9,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_pause_loop_exiting",
                bits_range: 10..=10,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_rdrand_exiting",
                bits_range: 11..=11,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_invpcid",
                bits_range: 12..=12,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_vm_functions",
                bits_range: 13..=13,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_vmcs_shadowing",
                bits_range: 14..=14,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_encls_exiting",
                bits_range: 15..=15,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_rdseed_exiting",
                bits_range: 16..=16,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_pml",
                bits_range: 17..=17,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_ept_violation_#ve",
                bits_range: 18..=18,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_conceal_vmx_from_pt",
                bits_range: 19..=19,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_xsaves/xrstors",
                bits_range: 20..=20,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_pasid_translation",
                bits_range: 21..=21,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_mode_based_execute_control_for_ept",
                bits_range: 22..=22,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_sub_page_write_permissions_for_ept",
                bits_range: 23..=23,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_intel_pt_uses_guest_physical_addresses",
                bits_range: 24..=24,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_use_tsc_scaling",
                bits_range: 25..=25,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_user_wait_and_pause",
                bits_range: 26..=26,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_enable_pconfig",
                bits_range: 27..=27,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_28_29",
                bits_range: 28..=29,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_vmm_bus_lock_detection",
                bits_range: 30..=30,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_zero_instruction_timeou",
                bits_range: 31..=31,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_virtualize_apic_accesses",
                bits_range: 32..=32,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_ept",
                bits_range: 33..=33,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_descriptor_table_exiting",
                bits_range: 34..=34,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_rdtscp",
                bits_range: 35..=35,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_virtualize_x2apic_mode",
                bits_range: 36..=36,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_vpid",
                bits_range: 37..=37,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_wbinvd_exiting",
                bits_range: 38..=38,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_unrestricted_guest",
                bits_range: 39..=39,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_apic_register_virtualization",
                bits_range: 40..=40,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_virtual_interrupt_delivery",
                bits_range: 41..=41,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_pause_loop_exiting",
                bits_range: 42..=42,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_rdrand_exiting",
                bits_range: 43..=43,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_invpcid",
                bits_range: 44..=44,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_vm_functions",
                bits_range: 45..=45,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_vmcs_shadowing",
                bits_range: 46..=46,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_encls_exiting",
                bits_range: 47..=47,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_rdseed_exiting",
                bits_range: 48..=48,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_pml",
                bits_range: 49..=49,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_ept_violation_#ve",
                bits_range: 50..=50,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_conceal_vmx_from_pt",
                bits_range: 51..=51,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_xsaves/xrstors",
                bits_range: 52..=52,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_pasid_translation",
                bits_range: 53..=53,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_mode_based_execute_control_for_ept",
                bits_range: 54..=54,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_sub_page_write_permissions_for_ept",
                bits_range: 55..=55,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_use_tsc_scaling",
                bits_range: 57..=57,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_enable_user_wait_and_pause",
                bits_range: 58..=58,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_28_29",
                bits_range: 60..=61,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_vmm_bus_lock_detection",
                bits_range: 62..=62,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_instruction_timeout",
                bits_range: 63..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    (
        MSR_IA32_VMX_EPT_VPID_CAP,
        &[
            ValueDefinition {
                name: "ept_execute_only",
                bits_range: 0..=0,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "page_walk_length_4",
                bits_range: 6..=6,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "page_walk_length_5",
                bits_range: 7..=7,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ept_mem_type_uc",
                bits_range: 8..=8,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ept_mem_type_wb",
                bits_range: 14..=14,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ept_pde_2m",
                bits_range: 16..=16,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "ept_pdpte_1g",
                bits_range: 17..=17,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "invept",
                bits_range: 20..=20,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "flags_ept",
                bits_range: 21..=21,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "vm_exit_violations_info",
                bits_range: 22..=22,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "shadow_stack_ctl",
                bits_range: 23..=23,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "single_context_invept",
                bits_range: 25..=25,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "all_context_invept",
                bits_range: 26..=26,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "invvpid",
                bits_range: 32..=32,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "individual_address_invvpid",
                bits_range: 40..=40,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "single_context_invvpid",
                bits_range: 41..=41,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "all_context_invvpid",
                bits_range: 42..=42,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "single_context_retaining_globals_invvpid",
                bits_range: 43..=43,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "max_hlat_prefix",
                bits_range: 48..=53,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    (
        MSR_IA32_VMX_VMFUNC,
        &[
            ValueDefinition {
                name: "allowed_one_eptp_switching",
                bits_range: 0..=0,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_1_63",
                bits_range: 1..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
    (
        MSR_IA32_VMX_EXIT_CTLS2,
        &[
            ValueDefinition {
                name: "allowed_one_0_2",
                bits_range: 0..=2,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_prematurely_busy_shadow_stack",
                bits_range: 3..=3,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
            ValueDefinition {
                name: "allowed_one_4_63",
                bits_range: 4..=63,
                policy: ProfilePolicy::Inherit,
                vendors: &[CpuVendor::Intel],
            },
        ],
    ),
];

const MSR_IA32_TSX_CTRL: u32 = 0x122;
const MSR_IA32_PEBS_DATA_CFG: u32 = 0x3f2;
const MAX_KVM_INTEL_PERFCTR: u32 = MSR_IA32_PERFCTR0 + 7;
const MAX_KVM_INTEL_EVENTSEL: u32 = MSR_P6_EVNTSEL0 + 7;
const MSR_F15H_PERF_CTR5: u32 = MSR_F15H_PERF_CTR + 10;

static NON_REQUIRED_MSRS_INTEL: &[RangeInclusive<u32>] = &[
    // Deprecated features not advertised via CPUID
    MSR_IA32_TSX_CTRL..=MSR_IA32_TSX_CTRL,
    MSR_IA32_BNDCFGS..=MSR_IA32_BNDCFGS,
    // Performance/PMU related MSRs
    MSR_IA32_PERFCTR0..=MAX_KVM_INTEL_PERFCTR,
    MSR_P6_EVNTSEL0..=MAX_KVM_INTEL_EVENTSEL,
    MSR_CORE_PERF_FIXED_CTR0..=MSR_CORE_PERF_GLOBAL_CTRL,
    MSR_IA32_PEBS_ENABLE..=MSR_IA32_PEBS_DATA_CFG,
    MSR_IA32_DS_AREA..=MSR_IA32_DS_AREA,
    // Related to Intel Processor trace (not advertised via CPUID)
    MSR_IA32_RTIT_CTL..=MSR_IA32_RTIT_CR3_MATCH,
    MSR_IA32_RTIT_OUTPUT_BASE..=MSR_IA32_RTIT_OUTPUT_MASK,
    MSR_IA32_RTIT_ADDR0_A..=MSR_IA32_RTIT_ADDR3_B,
];

static NON_REQUIRED_MSRS_COMMON: &[RangeInclusive<u32>] =
    &[MSR_IA32_PERF_CAPABILITIES..=MSR_IA32_PERF_CAPABILITIES];

static NON_REQUIRED_MSRS_AMD: &[RangeInclusive<u32>] = &[
    // Performance/PMU related MSRs
    MSR_K7_EVNTSEL0..=MSR_K7_PERFCTR3,
    MSR_F15H_PERF_CTL..=MSR_F15H_PERF_CTR5,
];

pub(super) fn generate_msrs_profile_data_with(
    feature_msrs: &[MsrEntry],
    supported_msr_indices: &[u32],
    vendor: CpuVendor,
) -> MsrProfileData {
    let mut adjustments = Vec::new();
    let mut required_msrs = Vec::new();

    for (msr_index, value_defs) in FEATURE_MSR_DEFINITIONS {
        let Some(feature_msr_value) = feature_msrs
            .iter()
            .find_map(|entry| (entry.index == *msr_index).then_some(entry.data))
        else {
            continue;
        };

        required_msrs.push(*msr_index);

        let mask = ValueDefinition::mask(value_defs, vendor).unwrap_or(0);
        let replacements = ValueDefinition::replacements(value_defs, feature_msr_value, vendor);

        adjustments.push((
            RegisterAddress(*msr_index),
            FeatureMsrAdjustment { replacements, mask },
        ));
    }

    let non_required_msrs: Vec<&RangeInclusive<u32>> = NON_REQUIRED_MSRS_COMMON
        .into_iter()
        .chain(match vendor {
            CpuVendor::Intel => NON_REQUIRED_MSRS_INTEL,
            CpuVendor::AMD => NON_REQUIRED_MSRS_AMD,
            _ => &[],
        })
        .collect();

    for msr_index in supported_msr_indices {
        if non_required_msrs
            .iter()
            .any(|range| range.contains(msr_index))
        {
            continue;
        }
        required_msrs.push(*msr_index);
    }

    required_msrs.sort();
    required_msrs.dedup();

    let required_msrs: Vec<RegisterAddress> =
        required_msrs.into_iter().map(RegisterAddress).collect();

    MsrProfileData {
        adjustments,
        required_msrs,
    }
}
