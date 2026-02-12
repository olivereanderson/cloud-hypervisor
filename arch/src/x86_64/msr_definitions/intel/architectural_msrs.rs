use crate::x86_64::CpuidReg;
use crate::x86_64::cpuid_definitions::{
    Parameters, ProfilePolicy as CpuidProfilePolicy, intel::INTEL_CPUID_DEFINITIONS,
};

/// Compile time check that the given `BIT` in the CPUID output register specified by `params` is not
/// declared to be overwritten by `0` for non-host CPU profiles.
const fn assert_not_denied_cpuid_feature<const BIT: u8>(params: Parameters) {
    if let Some(defs) = INTEL_CPUID_DEFINITIONS.get(params)
        && let Some(def) = defs.find_bit::<BIT>()
    {
        assert!(!matches!(def.policy, CpuidProfilePolicy::Static(0)));
    } else {
        panic!("Unable to lookup CPUID value definition with the given parameters and feature bit");
    };
}

mod permitted_architectural_msrs {

    use crate::x86_64::msr_definitions::intel::architectural_msrs::assert_not_denied_cpuid_feature;

    use super::{CpuidProfilePolicy, CpuidReg, INTEL_CPUID_DEFINITIONS, Parameters};

    // TODO: Not sure if we need to permit this
    const IA32_P5_MC_ADDR: u32 = 0x0;
    // TODO: Not sure if we need to permit this
    const IA32_P5_MC_TYPE: u32 = 0x1;

    const IA32_TIME_STAMP_COUNTER: u32 = 0x10;

    const IA32_APIC_BASE: u32 = 0x1b;

    /// (R/O)
    const IA32_BARRIER: u32 = 0x2f;
    const _IA32_BARRIER_CPUID_CHECK: () = const {
        assert_not_denied_cpuid_feature::<27>(Parameters {
            leaf: 0x7,
            sub_leaf: 0..=0,
            register: CpuidReg::EAX,
        });
    };

    /// Control Features (R/W)
    const IA32_FEATURE_CONTROL: u32 = 0x3a;

    /// Per Logical Processor TSC Adjust (R/Write to clear)
    const IA32_TSC_ADJUST: u32 = 0x3b;
    const _IA32_TSC_ADJUST_CPUID_CHECK: () = assert_not_denied_cpuid_feature::<1>(Parameters {
        leaf: 0x7,
        sub_leaf: 0..=0,
        register: CpuidReg::EBX,
    });

    /// Speculation Control (R/W).
    const IA32_SPEC_CTRL: u32 = 0x48;
    const _IA32_SPECT_CTRL_CPUID_CHECK: () = assert_not_denied_cpuid_feature::<26>(Parameters {
        leaf: 0x7,
        sub_leaf: 0..=0,
        register: CpuidReg::EDX,
    });

    /// Prediction Command (WO)
    const IA32_PRED_CMD: u32 = 0x49;
    const _IA32_PRED_CMD_CPUID_CHECK: () = assert_not_denied_cpuid_feature::<26>(Parameters {
        leaf: 0x7,
        sub_leaf: 0..=0,
        register: CpuidReg::EDX,
    });

    // TODO: Does it really make sense to permit this MSR?
    const IA32_SMM_MONITOR_CTL: u32 = 0x9b;
    const _IA32_SMM_MONITOR_CTL_CPUID_CHECK: () =
        assert_not_denied_cpuid_feature::<5>(Parameters {
            leaf: 0x1,
            sub_leaf: 0..=0,
            register: CpuidReg::ECX,
        });

    /// xAPIC Disable Status (R/O)
    const IA32_XAPIC_DISABLE_STATUS: u32 = 0xbd;
    const _IA32_XAPIC_DISABLE_STATUS_CPUID_CHECK: () =
        assert_not_denied_cpuid_feature::<29>(Parameters {
            leaf: 0x7,
            sub_leaf: 0..=0,
            register: CpuidReg::EDX,
        });
    // TODO: Also assert that IA32_ARCH_CAPABILITIES[21] is also not hard-coded to prevent
    // this MSR

    // TODO: Do we really want to permit this MSR?
    const IA32_UMWAIT_CONTROL: u32 = 0xe1;

    /// MTRR Capability (R/O)
    const IA32_MTRRCAP: u32 = 0xfe;

    /// Flush Command (WO)
    const IA32_FLUSH_CMD: u32 = 0x10b;

    // TODO: Should probably use inherit policy here
    const _IA32_FLUSH_CMD_CPUID_CHECK: () = assert_not_denied_cpuid_feature::<28>(Parameters {
        leaf: 0x7,
        sub_leaf: 0..=0,
        register: CpuidReg::EDX,
    });

    /// SYSENTER_CS_MSR (R/W)
    const IA32_SYSENTER_CS: u32 = 0x174;

    /// SYSENTER_ESP_MSR (R/W)
    const IA32_SYSENTER_ESP: u32 = 0x175;

    /// SYSENTER_ESP_MSR (R/W)
    const IA32_SYSENTER_EIP: u32 = 0x176;

    /// Overclocking Status (R/O)
    const IA32_OVERCLOCKING_STATUS: u32 = 0x195;
    // TODO: Also check consistency with IA32_ARCH_CAPABILITIES[23]

    const IA32_CLOCK_MODULATION: u32 = 0x19a;
    const _IA32_CLOCK_MODULATION_CPUID_CHECK: () =
        assert_not_denied_cpuid_feature::<22>(Parameters {
            leaf: 0x1,
            sub_leaf: 0..=0,
            register: CpuidReg::EDX,
        });

    // TODO: Should we perhaps not permit IA32_MISC_ENABLE?
    const IA32_MISC_ENABLE: u32 = 0x1a0;
    /// A list of permitted Intel IA32 MSRs that are not considered MSR-based feature indices
    /// by KVM.
    ///
    /// The MSRs listed here can be studied further in Table 2.2 in Section 2.1 of the Intel SDM
    /// Vol. 4 from October 2025
    pub(in crate::x86_64) const PERMITTED_IA32_MSRS: [u32; 155] = [
        IA32_P5_MC_ADDR,
        IA32_P5_MC_TYPE,
        IA32_TIME_STAMP_COUNTER,
        IA32_APIC_BASE,
        IA32_BARRIER,
        IA32_FEATURE_CONTROL,
        IA32_TSC_ADJUST,
        IA32_SPEC_CTRL,
        IA32_PRED_CMD,
        0x82,
        0x83,
        0x84,
        0x85,
        0x86,
        IA32_SMM_MONITOR_CTL,
        IA32_XAPIC_DISABLE_STATUS,
        IA32_UMWAIT_CONTROL,
        IA32_MTRRCAP,
        IA32_FLUSH_CMD,
        IA32_SYSENTER_CS,
        IA32_SYSENTER_ESP,
        IA32_SYSENTER_EIP,
        IA32_OVERCLOCKING_STATUS,
        IA32_CLOCK_MODULATION,
        IA32_MISC_ENABLE,
        0x1c4,
        0x1c5,
        0x1f2,
        0x1f3,
        0x1f8,
        0x1f9,
        0x1fa,
        0x200,
        0x201,
        0x202,
        0x203,
        0x204,
        0x205,
        0x206,
        0x207,
        0x208,
        0x209,
        0x20a,
        0x20b,
        0x20c,
        0x20d,
        0x20e,
        0x20f,
        0x210,
        0x211,
        0x212,
        0x213,
        0x250,
        0x258,
        0x259,
        0x268,
        0x269,
        0x26a,
        0x26b,
        0x26c,
        0x26d,
        0x26e,
        0x26f,
        0x277,
        0x2ff,
        0x345,
        0x480,
        0x481,
        0x482,
        0x483,
        0x484,
        0x485,
        0x486,
        0x487,
        0x488,
        0x489,
        0x48a,
        0x48b,
        0x48c,
        0x48d,
        0x48e,
        0x48f,
        0x490,
        0x491,
        0x492,
        0x493,
        0x6a0,
        0x6a2,
        0x6a4,
        0x6a5,
        0x6a6,
        0x6a7,
        0x6a8,
        0x6e0,
        0x775,
        0x7a5,
        0x802,
        0x803,
        0x808,
        0x80a,
        0x80b,
        0x80d,
        0x80f,
        0x810,
        0x811,
        0x812,
        0x813,
        0x814,
        0x815,
        0x816,
        0x817,
        0x818,
        0x819,
        0x81a,
        0x81b,
        0x81c,
        0x81d,
        0x81e,
        0x81f,
        0x820,
        0x821,
        0x822,
        0x823,
        0x824,
        0x825,
        0x826,
        0x827,
        0x828,
        0x82f,
        0x830,
        0x832,
        0x833,
        0x834,
        0x835,
        0x836,
        0x837,
        0x838,
        0x839,
        0x83e,
        0x83f,
        0xc88,
        0xc89,
        0xd93,
        0xda0,
        0x1406,
        0x1b01,
        0xc0000080,
        0xc0000081,
        0xc0000082,
        0xc0000083,
        0xc0000084,
        0xc0000100,
        0xc0000101,
        0xc0000102,
        0xc0000103,
    ];
}

mod forbidden_architectural_msrs {
    // TODO: Not sure about IA32_P5_MC_ADDR & IA32_P5_MC_TYPE

    // TODO: IA32_MCU_OPT_CTRL should be not permitted (ensure this via IA32_ARCH_CAPABILITIES)

    const IA32_MONITOR_FILTER_SIZE: (u32, u32) = (0x6, 0x6);
    // TODO: Not sure about this one
    const IA32_PLATFORM_ID: (u32, u32) = (0x17, 0x17);

    /// Only available is CPUID 0x7.0x1.EBX[0] = 1, but this is always 0 for non-host CPU profiles
    const IA32_PPIN_CTL: (u32, u32) = (0x4e, 0x4e);

    /// Only available is CPUID 0x7.0x1.EBX[0] = 1, but this is always 0 for non-host CPU profiles
    const IA32_PPIN: (u32, u32) = (0x4f, 0x4f);

    /// Used for microcode updates. Should not be available for guests.
    const IA32_BIOS_UPDT_TRIG: (u32, u32) = (0x79, 0x79);

    /// Currently only related to Secure enclaves/Keylocker which is not available for non-host CPU profiles
    const IA32_FEATURE_ACTIVATION: (u32, u32) = (0x7a, 0x7a);

    /// Related to microcode updates
    const IA32_MCU_ENUMERATION: (u32, u32) = (0x7b, 0x7b);

    const IA32_MCU_STATUS: (u32, u32) = (0x7c, 0x7c);

    /// Related to total memory encryption
    const IA32_MKTME_KEYID_PARTITIONING: (u32, u32) = (0x87, 0x87);

    // TODO: Not sure what to do about IA32_BIOS_SIGN_ID (note that it is also a MSR-based feature according to KVM)

    const IA32_SGXLEPUBKEYHASH0: (u32, u32) = (0x8c, 0x8c);

    const IA32_SGXLEPUBKEYHASH1: (u32, u32) = (0x8d, 0x8d);

    const IA32_SGXLEPUBKEYHASH2: (u32, u32) = (0x8e, 0x8e);

    const IA32_SGXLEPUBKEYHASH3: (u32, u32) = (0x8f, 0x8f);

    const IA32_SGXLEPUBKEYHASH4: (u32, u32) = (0x90, 0x90);

    const IA32_SGXLEPUBKEYHASH5: (u32, u32) = (0x91, 0x91);

    // TODO: Check this
    const IA32_SMBASE: (u32, u32) = (0x9e, 0x9e);

    const IA32_MISC_PACKAGE_CTLS: (u32, u32) = (0xbc, 0xbc);

    // TODO: IA32_X2APIC_DISABLE_STATUS

    const IA32_PMC0: (u32, u32) = (0xc1, 0xc1);
    const IA32_PMC1: (u32, u32) = (0xc2, 0xc2);
    const IA32_PMC2: (u32, u32) = (0xc3, 0xc3);
    const IA32_PMC3: (u32, u32) = (0xc4, 0xc4);
    const IA32_PMC4: (u32, u32) = (0xc5, 0xc5);
    const IA32_PMC5: (u32, u32) = (0xc6, 0xc6);
    const IA32_PMC6: (u32, u32) = (0xc7, 0xc7);
    const IA32_PMC7: (u32, u32) = (0xc8, 0xc8);
    const IA32_PMC8: (u32, u32) = (0xc9, 0xc9);
    const IA32_PMC9: (u32, u32) = (0xca, 0xca);

    const IA32_CORE_CAPABILITIES: (u32, u32) = (0xcf, 0xcf);

    // TODO: IA32_UMWAIT_CONTROL

    // Disabled by CPUID for non-host CPU profiles
    const IA32_MPERF: (u32, u32) = (0xe7, 0xe7);

    const IA32_APERF: (u32, u32) = (0xe8, 0xe8);

    const IA32_TSX_FORCE_ABORT: (u32, u32) = (0x10f, 0x10f);

    // Disabled via static IA32_ARCH_CAPABILITIES bit for non-host CPU profiles
    const IA32_TSX_CTRL: (u32, u32) = (0x122, 0x122);

    // NOTE: IA32_MCU_OPT_CTRL must necessarily be available, due to
    // what we set in CPUID for some CPU profiles (inherit policy)

    // TODO: Don't know about IA32_SYSENTER_CS, IA32_SYSENTER_ESP,
    // IA32_SYSENTER_EIP
    //

    // TODO: Not sure if we can/should deny this MSR, but
    // it doesn't really make sense to have it available in
    // a virtualized environment
    //
    // If we keep it denied we should document that
    // even for 06_01H one cannot rely on the existence of this MSR
    const IA32_MCG_CAP: (u32, u32) = (0x179, 0x179);

    // TODO: Also not sure if we may deny this MSR
    const IA32_MCG_STATUS: (u32, u32) = (0x17a, 0x17a);

    // TODO: Can we deny this?
    const IA32_MCG_CTL: (u32, u32) = (0x17b, 0x17b);

    // TODO: 0x180- 0x185 is reserved, we should not list these MSRS at all

    /// Disabled via CPUID for all non-host CPU profiles
    const IA32_PERFEVTSEL0: (u32, u32) = (0x186, 0x186);
    const IA32_PERFEVTSEL1: (u32, u32) = (0x187, 0x187);
    const IA32_PERFEVTSEL2: (u32, u32) = (0x188, 0x188);
    const IA32_PERFEVTSEL3: (u32, u32) = (0x189, 0x189);
    const IA32_PERFEVTSEL4: (u32, u32) = (0x18a, 0x18a);
    const IA32_PERFEVTSEL5: (u32, u32) = (0x18b, 0x18b);
    const IA32_PERFEVTSEL6: (u32, u32) = (0x18c, 0x18c);
    const IA32_PERFEVTSEL7: (u32, u32) = (0x18d, 0x18d);
    const IA32_PERFEVTSEL8: (u32, u32) = (0x18e, 0x18e);
    const IA32_PERFEVTSEL9: (u32, u32) = (0x18f, 0x18f);

    // TODO: 0x18a - 0x194 is reserved and should not be included in any list

    // TODO: 0x196, 197 is reserved and should not be included in any list
    //

    const IA32_PERF_STATUS: (u32, u32) = (0x198, 0x198);

    const IA32_PERF_CTL: (u32, u32) = (0x199, 0x199);

    // Disabled via CPUID for non-host profiles
    const IA32_THERM_INTERRUPT: (u32, u32) = (0x19b, 0x19b);

    // Disabled via CPUID for non-host profiles
    const IA32_THERM_STATUS: (u32, u32) = (0x19c, 0x19c);

    // TODO: Consider disabling IA32_MISC_ENABLE

    // Disabled via CPUID for non-host profiles
    const IA32_ENERGY_PERF_BIAS: (u32, u32) = (0x1b0, 0x1b0);

    // Disabled via CPUID for non-host profiles
    const IA32_PACKAGE_THERM_STATUS: (u32, u32) = (0x1b1, 0x1b1);

    // Disabled via CPUID for non-host profiles
    const IA32_PACKAGE_THERM_INTERRUPT: (u32, u32) = (0x1b2, 0x1b2);

    const IA32_DEBUGCTL: (u32, u32) = (0x1d9, 0x1d9);

    const IA32_LER_FROM_IP: (u32, u32) = (0x1dd, 0x1dd);

    const IA32_LER_TO_IP: (u32, u32) = (0x1de, 0x1de);

    const IA32_LER_INFO: (u32, u32) = (0x1e0, 0x1e0);

    // TODO: Not sure about IA32_SMRR_PHYSBASE & IA32_SMRR_PHYSMASK

    const IA32_MC_I_CTL2: (u32, u32) = (0x280, 0x29f);

    // Disabled via CPUID for non-host profiles
    const IA32_INTEGRITY_STATUS: (u32, u32) = (0x2dc, 0x2dc);

    const IA32_FIXED_CTRI: (u32, u32) = (0x309, 0x30f);

    // IA32_PERF_CAPABILITIES is an MSR-based feature thus not listed here

    // Disabled via CPUID for non-host profiles
    const IA32_FIXED_CTR_CTRL: (u32, u32) = (0x38d, 0x38d);

    // Disabled via CPUID for non-host profiles
    const IA32_PERF_GLOBAL_STATUS: (u32, u32) = (0x38e, 0x38e);

    // Disabled via CPUID for non-host profiles
    const IA32_PERF_GLOBAL_CTRL: (u32, u32) = (0x38f, 0x38f);

    // Disabled via CPUID for non-host profiles
    const IA32_PERF_GLOBAL_STATUS_RESET: (u32, u32) = (0x390, 0x390);

    // Disabled via CPUID for non-host profiles
    const IA32_PERF_GLOBAL_STATUS_SET: (u32, u32) = (0x391, 0x391);

    // Disabled via CPUID for non-host profiles
    const IA32_PERF_GLOBAL_INUSE: (u32, u32) = (0x392, 0x392);

    // TODO: Not sure about this one, but seems to be related to performance monitoring which
    // should be disabled for non-host CPU profiles.
    const IA32_PEBS_ENABLE: (u32, u32) = (0x3f1, 0x3f1);

    const IA32_MC0_CTL: (u32, u32) = (0x400, 0x400);
    const IA32_MC0_STATUS: (u32, u32) = (0x401, 0x401);
    const IA32_MC0_ADDR: (u32, u32) = (0x402, 0x402);
    const IA32_MC0_MISC: (u32, u32) = (0x403, 0x403);
    const IA32_MC1_CTL: (u32, u32) = (0x404, 0x404);
    const IA32_MC1_STATUS: (u32, u32) = (0x405, 0x405);
    const IA32_MC1_ADDR: (u32, u32) = (0x406, 0x406);

    const IA32_MC1_MISC: (u32, u32) = (0x407, 0x407);
    const IA32_MC2_CTL: (u32, u32) = (0x408, 0x408);
    const IA32_MC2_STATUS: (u32, u32) = (0x409, 0x409);
    const IA32_MC2_ADDR: (u32, u32) = (0x40a, 0x40a);
    const IA32_MC2_MISC: (u32, u32) = (0x40b, 0x40b);
    const IA32_MC3_CTL: (u32, u32) = (0x40c, 0x40c);
    const IA32_MC3_STATUS: (u32, u32) = (0x40d, 0x40d);
    const IA32_MC3_ADDR1: (u32, u32) = (0x40e, 0x40e);
    const IA32_MC3_MISC: (u32, u32) = (0x40f, 0x40f);
    const IA32_MC4_CTL: (u32, u32) = (0x410, 0x410);
    const IA32_MC4_STATUS: (u32, u32) = (0x411, 0x411);
    const IA32_MC4_ADDR: (u32, u32) = (0x412, 0x412);
    const IA32_MC4_MISC: (u32, u32) = (0x413, 0x413);
    const IA32_MC5_CTL: (u32, u32) = (0x414, 0x414);
    const IA32_MC5_STATUS: (u32, u32) = (0x415, 0x415);
    const IA32_MC5_ADDR: (u32, u32) = (0x416, 0x416);
    const IA32_MC5_MISC: (u32, u32) = (0x417, 0x417);
    const IA32_MC6_CTL: (u32, u32) = (0x418, 0x418);

    const IA32_MC6_STATUS: (u32, u32) = (0x419, 0x419);
    const IA32_MC6_ADDR1: (u32, u32) = (0x41a, 0x41a);
    const IA32_MC6_MISC: (u32, u32) = (0x41b, 0x41b);
    const IA32_MC7_CTL: (u32, u32) = (0x41c, 0x41c);
    const IA32_MC7_STATUS: (u32, u32) = (0x41d, 0x41d);
    const IA32_MC7_ADDR: (u32, u32) = (0x41e, 0x41e);
    const IA32_MC7_MISC: (u32, u32) = (0x41f, 0x41f);
    const IA32_MC8_CTL: (u32, u32) = (0x420, 0x420);
    const IA32_MC8_STATUS: (u32, u32) = (0x421, 0x421);
    const IA32_MC8_ADDR: (u32, u32) = (0x422, 0x422);
    const IA32_MC8_MISC: (u32, u32) = (0x423, 0x423);
    const IA32_MC9_CTL: (u32, u32) = (0x424, 0x424);
    const IA32_MC9_STATUS: (u32, u32) = (0x425, 0x425);
    const IA32_MC9_ADDR: (u32, u32) = (0x426, 0x426);
    const IA32_MC9_MISC: (u32, u32) = (0x427, 0x427);
    const IA32_MC10_CTL: (u32, u32) = (0x428, 0x428);
    const IA32_MC10_STATUS: (u32, u32) = (0x429, 0x429);
    const IA32_MC10_ADDR: (u32, u32) = (0x42a, 0x42a);
    const IA32_MC10_MISC: (u32, u32) = (0x42b, 0x42b);

    const IA32_MC11_CTL: (u32, u32) = (0x42c, 0x42c);
    const IA32_MC11_STATUS: (u32, u32) = (0x42d, 0x42d);
    const IA32_MC11_ADDR: (u32, u32) = (0x42e, 0x42e);
    const IA32_MC11_MISC: (u32, u32) = (0x42f, 0x42f);
    const IA32_MC12_CTL: (u32, u32) = (0x430, 0x430);
    const IA32_MC12_STATUS: (u32, u32) = (0x431, 0x431);
    const IA32_MC12_ADDR: (u32, u32) = (0x432, 0x432);
    const IA32_MC12_MISC: (u32, u32) = (0x433, 0x433);
    const IA32_MC13_CTL: (u32, u32) = (0x434, 0x434);
    const IA32_MC13_STATUS: (u32, u32) = (0x435, 0x435);
    const IA32_MC13_ADDR: (u32, u32) = (0x436, 0x436);
    const IA32_MC13_MISC: (u32, u32) = (0x437, 0x437);
    const IA32_MC14_CTL: (u32, u32) = (0x438, 0x438);
    const IA32_MC14_STATUS: (u32, u32) = (0x439, 0x439);
    const IA32_MC14_ADDR: (u32, u32) = (0x43a, 0x43a);
    const IA32_MC14_MISC: (u32, u32) = (0x43b, 0x43b);
    const IA32_MC15_CTL: (u32, u32) = (0x43c, 0x43c);
    const IA32_MC15_STATUS: (u32, u32) = (0x43d, 0x43d);

    const IA32_MC15_ADDR: (u32, u32) = (0x43e, 0x43e);
    const IA32_MC15_MISC: (u32, u32) = (0x43f, 0x43f);
    const IA32_MC16_CTL: (u32, u32) = (0x440, 0x440);
    const IA32_MC16_STATUS: (u32, u32) = (0x441, 0x441);
    const IA32_MC16_ADDR: (u32, u32) = (0x442, 0x442);
    const IA32_MC16_MISC: (u32, u32) = (0x443, 0x443);
    const IA32_MC17_CTL: (u32, u32) = (0x444, 0x444);
    const IA32_MC17_STATUS: (u32, u32) = (0x445, 0x445);
    const IA32_MC17_ADDR: (u32, u32) = (0x446, 0x446);
    const IA32_MC17_MISC: (u32, u32) = (0x447, 0x447);
    const IA32_MC18_CTL: (u32, u32) = (0x448, 0x448);
    const IA32_MC18_STATUS: (u32, u32) = (0x449, 0x449);
    const IA32_MC18_ADDR: (u32, u32) = (0x44a, 0x44a);
    const IA32_MC18_MISC: (u32, u32) = (0x44b, 0x44b);
    const IA32_MC19_CTL: (u32, u32) = (0x44c, 0x44c);
    const IA32_MC19_STATUS: (u32, u32) = (0x44d, 0x44d);
    const IA32_MC19_ADDR: (u32, u32) = (0x44e, 0x44e);
    const IA32_MC19_MISC: (u32, u32) = (0x44f, 0x44f);
    const IA32_MC20_CTL: (u32, u32) = (0x450, 0x450);

    const IA32_MC20_STATUS: (u32, u32) = (0x451, 0x451);
    const IA32_MC20_ADDR: (u32, u32) = (0x452, 0x452);
    const IA32_MC20_MISC: (u32, u32) = (0x453, 0x453);
    const IA32_MC21_CTL: (u32, u32) = (0x454, 0x454);
    const IA32_MC21_STATUS: (u32, u32) = (0x455, 0x455);
    const IA32_MC21_ADDR: (u32, u32) = (0x456, 0x456);
    const IA32_MC21_MISC: (u32, u32) = (0x457, 0x457);
    const IA32_MC22_CTL: (u32, u32) = (0x458, 0x458);
    const IA32_MC22_STATUS: (u32, u32) = (0x459, 0x459);
    const IA32_MC22_ADDR: (u32, u32) = (0x45a, 0x45a);
    const IA32_MC22_MISC: (u32, u32) = (0x45b, 0x45b);
    const IA32_MC23_CTL: (u32, u32) = (0x45c, 0x45c);
    const IA32_MC23_STATUS: (u32, u32) = (0x45d, 0x45d);
    const IA32_MC23_ADDR: (u32, u32) = (0x45e, 0x45e);
    const IA32_MC23_MISC: (u32, u32) = (0x45f, 0x45f);
    const IA32_MC24_CTL: (u32, u32) = (0x460, 0x460);
    const IA32_MC24_STATUS: (u32, u32) = (0x461, 0x461);
    const IA32_MC24_ADDR: (u32, u32) = (0x462, 0x462);

    const IA32_MC24_MISC: (u32, u32) = (0x463, 0x463);
    const IA32_MC25_CTL: (u32, u32) = (0x464, 0x464);
    const IA32_MC25_STATUS: (u32, u32) = (0x465, 0x465);
    const IA32_MC25_ADDR: (u32, u32) = (0x466, 0x466);
    const IA32_MC25_MISC: (u32, u32) = (0x467, 0x467);
    const IA32_MC26_CTL: (u32, u32) = (0x468, 0x468);
    const IA32_MC26_STATUS: (u32, u32) = (0x469, 0x469);
    const IA32_MC26_ADDR: (u32, u32) = (0x46a, 0x46a);
    const IA32_MC26_MISC: (u32, u32) = (0x46b, 0x46b);
    const IA32_MC27_CTL: (u32, u32) = (0x46c, 0x46c);
    const IA32_MC27_STATUS: (u32, u32) = (0x46d, 0x46d);
    const IA32_MC27_ADDR: (u32, u32) = (0x46e, 0x46e);
    const IA32_MC27_MISC: (u32, u32) = (0x46f, 0x46f);
    const IA32_MC28_CTL: (u32, u32) = (0x470, 0x470);
    const IA32_MC28_STATUS: (u32, u32) = (0x471, 0x471);
    const IA32_MC28_ADDR: (u32, u32) = (0x472, 0x472);
    const IA32_MC28_MISC: (u32, u32) = (0x473, 0x473);
    const IA32_MC29_CTL: (u32, u32) = (0x474, 0x474);
    const IA32_MC29_STATUS: (u32, u32) = (0x475, 0x475);

    const IA32_MC29_ADDR: (u32, u32) = (0x476, 0x476);
    const IA32_MC29_MISC: (u32, u32) = (0x477, 0x477);
    const IA32_MC30_CTL: (u32, u32) = (0x478, 0x478);
    const IA32_MC30_STATUS: (u32, u32) = (0x479, 0x479);
    const IA32_MC30_ADDR: (u32, u32) = (0x47a, 0x47a);
    const IA32_MC30_MISC: (u32, u32) = (0x47b, 0x47b);
    const IA32_MC31_CTL: (u32, u32) = (0x47c, 0x47c);
    const IA32_MC31_STATUS: (u32, u32) = (0x47d, 0x47d);
    const IA32_MC31_ADDR: (u32, u32) = (0x47e, 0x47e);
    const IA32_MC31_MISC: (u32, u32) = (0x47f, 0x47f);

    const IA32_A_PMC0: (u32, u32) = (0x4c1, 0x4c1);
    const IA32_A_PMC1: (u32, u32) = (0x4c2, 0x4c2);
    const IA32_A_PMC2: (u32, u32) = (0x4c3, 0x4c3);
    const IA32_A_PMC3: (u32, u32) = (0x4c4, 0x4c4);
    const IA32_A_PMC4: (u32, u32) = (0x4c5, 0x4c5);
    const IA32_A_PMC5: (u32, u32) = (0x4c6, 0x4c6);
    const IA32_A_PMC6: (u32, u32) = (0x4c7, 0x4c7);
    const IA32_A_PMC7: (u32, u32) = (0x4c8, 0x4c8);
    const IA32_A_PMC8: (u32, u32) = (0x4c9, 0x4c9);
    const IA32_A_PMC9: (u32, u32) = (0x4ca, 0x4ca);

    const IA32_MCG_EXT_CTL: (u32, u32) = (0x4d0, 0x4d0);

    // SGX is disabled via CPUID for non-host CPU profiles
    const IA32_SGX_SVN_STATUS: (u32, u32) = (0x500, 0x500);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_RTIT_OUTPUT_BASE: (u32, u32) = (0x560, 0x560);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_RTIT_OUTPUT_MASK_PTRS: (u32, u32) = (0x561, 0x561);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_RTIT_CTL: (u32, u32) = (0x570, 0x570);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_RTIT_STATUS: (u32, u32) = (0x571, 0x571);

    // Disabled via CPU profiles
    const IA32_RTIT_CR3_MATCH: (u32, u32) = (0x572, 0x572);

    const IA32_RTIT_ADDR0_A: (u32, u32) = (0x580, 0x580);
    const IA32_RTIT_ADDR0_B: (u32, u32) = (0x581, 0x581);
    const IA32_RTIT_ADDR1_A: (u32, u32) = (0x582, 0x582);
    const IA32_RTIT_ADDR1_B: (u32, u32) = (0x583, 0x583);
    const IA32_RTIT_ADDR2_A: (u32, u32) = (0x584, 0x584);
    const IA32_RTIT_ADDR2_B: (u32, u32) = (0x585, 0x585);
    const IA32_RTIT_ADDR3_A: (u32, u32) = (0x586, 0x586);
    const IA32_RTIT_ADDR3_B: (u32, u32) = (0x587, 0x587);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_DS_AREA: (u32, u32) = (0x600, 0x600);

    // TODO: IA32_TSC_DEADLINE should be available because the TSC_DEADLINE CPUID bit
    // is set by CHV unconditionally. The availability of this MSR probably needs to be
    // handled by CHV itself and not the CPU profiles

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PKRS: (u32, u32) = (0x6e1, 0x6e1);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PM_ENABLE: (u32, u32) = (0x770, 0x770);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HWP_CAPABILITIES: (u32, u32) = (0x771, 0x771);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HWP_REQUEST_PKG: (u32, u32) = (0x772, 0x772);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HWP_INTERRUPT: (u32, u32) = (0x773, 0x773);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HWP_REQUEST: (u32, u32) = (0x774, 0x774);

    // TODO: Can we also deny IA32_PECI_HWP_REQUEST_INFO?

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HWP_CTL: (u32, u32) = (0x776, 0x776);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HWP_STATUS: (u32, u32) = (0x777, 0x777);

    // TODO: Currently permitted via IA32_ARCH_CAPABILITIES (bit 22),
    // but that bit should probably have policy Static(0) ?
    const IA32_MCU_EXT_SERVICE: (u32, u32) = (0x7a3, 0x7a3);

    const IA32_MCU_ROLLBACK_MIN_ID: (u32, u32) = (0x7a4, 0x7a4);

    // TODO: Not sure about IA32_MCU_STAGING_MBOX_ADDR

    const IA32_ROLLBACK_SIGN_ID_0: (u32, u32) = (0x7b0, 0x7b0);
    const IA32_ROLLBACK_SIGN_ID_1: (u32, u32) = (0x7b1, 0x7b1);
    const IA32_ROLLBACK_SIGN_ID_2: (u32, u32) = (0x7b2, 0x7b2);
    const IA32_ROLLBACK_SIGN_ID_3: (u32, u32) = (0x7b3, 0x7b3);
    const IA32_ROLLBACK_SIGN_ID_4: (u32, u32) = (0x7b4, 0x7b4);
    const IA32_ROLLBACK_SIGN_ID_5: (u32, u32) = (0x7b5, 0x7b5);
    const IA32_ROLLBACK_SIGN_ID_6: (u32, u32) = (0x7b6, 0x7b6);
    const IA32_ROLLBACK_SIGN_ID_7: (u32, u32) = (0x7b7, 0x7b7);
    const IA32_ROLLBACK_SIGN_ID_8: (u32, u32) = (0x7b8, 0x7b8);
    const IA32_ROLLBACK_SIGN_ID_9: (u32, u32) = (0x7b9, 0x7b9);
    const IA32_ROLLBACK_SIGN_ID_10: (u32, u32) = (0x7ba, 0x7ba);
    const IA32_ROLLBACK_SIGN_ID_11: (u32, u32) = (0x7bb, 0x7bb);
    const IA32_ROLLBACK_SIGN_ID_12: (u32, u32) = (0x7bc, 0x7bc);
    const IA32_ROLLBACK_SIGN_ID_13: (u32, u32) = (0x7bd, 0x7bd);
    const IA32_ROLLBACK_SIGN_ID_14: (u32, u32) = (0x7be, 0x7be);
    const IA32_ROLLBACK_SIGN_ID_15: (u32, u32) = (0x7bf, 0x7bf);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_TME_CAPABILITY: (u32, u32) = (0x981, 0x981);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_TME_ACTIVATE: (u32, u32) = (0x982, 0x982);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_TME_EXCLUDE_MASK: (u32, u32) = (0x983, 0x983);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_TME_EXCLUDE_BASE: (u32, u32) = (0x984, 0x984);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_UINTR_RR: (u32, u32) = (0x985, 0x985);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_UINTR_HANDLER: (u32, u32) = (0x986, 0x986);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_UINTR_STACKADJUST: (u32, u32) = (0x987, 0x987);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_UINTR_MISC: (u32, u32) = (0x988, 0x988);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_UINTR_PD: (u32, u32) = (0x989, 0x989);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_UINTR_TT: (u32, u32) = (0x98a, 0x98a);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_COPY_STATUS: (u32, u32) = (0x990, 0x990);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_IWKEYBACKUP_STATUS: (u32, u32) = (0x991, 0x991);

    const IA32_TME_CLEAR_SAVED_KEY: (u32, u32) = (0x9fb, 0x9fb);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_DEBUG_INTERFACE: (u32, u32) = (0xc80, 0xc80);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_L3_QOS_CFG: (u32, u32) = (0xc81, 0xc81);

    // Disabled via CPUID
    const IA32_L2_QOS_CFG: (u32, u32) = (0xc82, 0xc82);

    // Disabled via CPUID
    const IA32_L3_IO_QOS_CFG: (u32, u32) = (0xc83, 0xc83);

    // TODO: Not sure about IA32_RESOURCE_PRIORITY and IA32_RESOURCE_PRIORITY_PKG

    // Disabled via CPUID for non-host CPU profiles
    const IA32_QM_EVTSEL: (u32, u32) = (0xc8d, 0xc8d);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_QM_CTR: (u32, u32) = (0xc8e, 0xc8e);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PQR_ASSOC: (u32, u32) = (0xc8f, 0xc8f);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_L3_MASK_0: (u32, u32) = (0xc90, 0xc90);

    const IA32_L3_MASK_N: (u32, u32) = (0xc91, 0xd8f);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_L2_MASK_0: (u32, u32) = (0xd10, 0xd10);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_L2_MASK_N: (u32, u32) = (0xd11, 0xd4f);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_L2_QOS_EXT_BW_THRTL_I: (u32, u32) = (0xd50, 0xd5e);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_BNDCFGS: (u32, u32) = (0xd90, 0xd90);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_COPY_LOCAL_TO_PLATFORM: (u32, u32) = (0xd91, 0xd91);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_COPY_PLATFORM_TO_LOCAL: (u32, u32) = (0xd92, 0xd92);

    // TODO: Not sure about IA32_PASID

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PKG_HDC_CTL: (u32, u32) = (0xdb0, 0xdb0);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PM_CTL1: (u32, u32) = (0xdb1, 0xdb1);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_THREAD_STALL: (u32, u32) = (0xdb2, 0xdb2);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_QOS_CORE_BW_THRTL_0: (u32, u32) = (0xe00, 0xe00);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_QOS_CORE_BW_THRTL_1: (u32, u32) = (0xe01, 0xe01);

    // TODO: Is it OK to disable this for CPU profiles?
    // Note that we have CPUID 0x7.EDX.[19] = 0 (ARCH_LBR)
    const IA32_LBR_X_INFO: (u32, u32) = (0x1200, 0x121f);

    // TDX related.
    const IA32_SEAMRR_BASE: (u32, u32) = (0x1400, 0x1400);

    // TDX related.
    const IA32_SEAMRR_MASK: (u32, u32) = (0x1401, 0x1401);

    // Disabled via ARCH_CAPABILITIES for non-host CPU profiles
    const IA32_MCU_CONTROL: (u32, u32) = (0x1406, 1406);

    const IA32_LBR_CTL: (u32, u32) = (0x14ce, 0x14ce);

    const IA32_LBR_DEPTH: (u32, u32) = (0x14cf, 0x14cf);

    const IA32_LBR_X_FROM_IP: (u32, u32) = (0x1500, 0x151f);

    const IA32_LBR_X_TO_IP: (u32, u32) = (0x1600, 0x161f);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HW_FEEDBACK_PTR: (u32, u32) = (0x17d0, 0x17d0);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HW_FEEDBACK_CONFIG: (u32, u32) = (0x17d1, 0x17d1);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_HW_FEEDBACK_THREAD_CHAR: (u32, u32) = (0x17d2, 0x17d2);

    const IA32_HW_FEEDBACK_THREAD_CONFIG: (u32, u32) = (0x17d4, 0x17d4);

    const IA32_HRESET_ENABLE: (u32, u32) = (0x17da, 0x17da);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP0_CTR: (u32, u32) = (0x1900, 0x1900);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP0_CFG_A: (u32, u32) = (0x1901, 0x1901);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP0_CFG_C: (u32, u32) = (0x1903, 0x1903);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP1_CTR: (u32, u32) = (0x1904, 0x1904);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP1_CFG_A: (u32, u32) = (0x1905, 0x1905);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP1_CFG_C: (u32, u32) = (0x1907, 0x1907);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP2_CTR: (u32, u32) = (0x1908, 0x1908);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP2_CFG_A: (u32, u32) = (0x1909, 0x1909);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP2_CFG_B: (u32, u32) = (0x190a, 0x190a);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP2_CFG_C: (u32, u32) = (0x190b, 0x190b);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP3_CTR: (u32, u32) = (0x190c, 0x190c);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP3_CFG_A: (u32, u32) = (0x190d, 0x190d);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP3_CFG_B: (u32, u32) = (0x190e, 0x190e);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP3_CFG_C: (u32, u32) = (0x190f, 0x190f);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP4_CTR: (u32, u32) = (0x1910, 0x1910);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP4_CFG_A: (u32, u32) = (0x1911, 0x1911);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP4_CFG_B: (u32, u32) = (0x1912, 0x1912);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP4_CFG_C: (u32, u32) = (0x1913, 0x1913);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP5_CTR: (u32, u32) = (0x1914, 0x1914);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP5_CFG_A: (u32, u32) = (0x1915, 0x1915);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP5_CFG_B: (u32, u32) = (0x1916, 0x1916);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP5_CFG_C: (u32, u32) = (0x1917, 0x1917);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP6_CTR: (u32, u32) = (0x1918, 0x1918);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP6_CFG_A: (u32, u32) = (0x1919, 0x1919);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP6_CFG_B: (u32, u32) = (0x191a, 0x191a);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP6_CFG_C: (u32, u32) = (0x191b, 0x191b);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP7_CTR: (u32, u32) = (0x191c, 0x191c);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP7_CFG_A: (u32, u32) = (0x191d, 0x191d);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP7_CFG_B: (u32, u32) = (0x191e, 0x191e);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP7_CFG_C: (u32, u32) = (0x191f, 0x191f);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP8_CTR: (u32, u32) = (0x1920, 0x1920);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP8_CFG_A: (u32, u32) = (0x1921, 0x1921);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP9_CTR: (u32, u32) = (0x1924, 0x1924);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_GP9_CFG_A: (u32, u32) = (0x1925, 0x1925);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_FX0_CTR: (u32, u32) = (0x1980, 0x1980);

    const IA32_PMC_FX0_CFG_B: (u32, u32) = (0x1982, 0x1982);
    const IA32_PMC_FX0_CFG_C: (u32, u32) = (0x1983, 0x1983);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_FX1_CTR: (u32, u32) = (0x1984, 0x1984);
    const IA32_PMC_FX1_CFG_B: (u32, u32) = (0x1986, 0x1986);
    const IA32_PMC_FX1_CFG_C: (u32, u32) = (0x1987, 0x1987);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_FX2_CTR: (u32, u32) = (0x1988, 0x1988);

    const IA32_PMC_FX2_CFG_C: (u32, u32) = (0x198b, 0x198b);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_FX3_CTR: (u32, u32) = (0x198c, 0x198c);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_FX4_CTR: (u32, u32) = (0x1990, 0x1990);
    const IA32_PMC_FX4_CFG_C: (u32, u32) = (0x1993, 0x1993);
    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_FX5_CTR: (u32, u32) = (0x1994, 0x1994);
    const IA32_PMC_FX5_CFG_C: (u32, u32) = (0x1997, 0x1997);

    // Disabled via CPUID for non-host CPU profiles
    const IA32_PMC_FX6_CTR: (u32, u32) = (0x1998, 0x1998);
    const IA32_PMC_FX6_CFG_C: (u32, u32) = (0x199b, 0x199b);

    // TODO: Check IA32_UARCH_MISC_CTL
    //

    /// A list of ARCHITECTURAL MSR register addresses that are forbidden for all non-host CPU profiles and also not
    /// considered MSR-based FEATURE indices by KVM.
    pub(in crate::x86_64) const FORBIDDEN_IA32_MSR_RANGES: [(u32, u32); 341] = [
        // TODO: Not sure about IA32_P5_MC_ADDR & IA32_P5_MC_TYPE
        IA32_MONITOR_FILTER_SIZE,
        // TODO: Not sure about this one
        IA32_PLATFORM_ID,
        /// Only available is CPUID 0x7.0x1.EBX[0] = 1, but this is always 0 for non-host CPU profiles
        IA32_PPIN_CTL,
        /// Only available is CPUID 0x7.0x1.EBX[0] = 1, but this is always 0 for non-host CPU profiles
        IA32_PPIN,
        /// Used for microcode updates. Should not be available for guests.
        IA32_BIOS_UPDT_TRIG,
        /// Currently only related to Secure enclaves/Keylocker which is not available for non-host CPU profiles
        IA32_FEATURE_ACTIVATION,
        /// Related to microcode updates
        IA32_MCU_ENUMERATION,
        IA32_MCU_STATUS,
        /// Related to total memory encryption
        IA32_MKTME_KEYID_PARTITIONING,
        // TODO: Not sure what to do about IA32_BIOS_SIGN_ID (note that it is also a MSR-based feature according to KVM)
        IA32_SGXLEPUBKEYHASH0,
        IA32_SGXLEPUBKEYHASH1,
        IA32_SGXLEPUBKEYHASH2,
        IA32_SGXLEPUBKEYHASH3,
        IA32_SGXLEPUBKEYHASH4,
        IA32_SGXLEPUBKEYHASH5,
        // TODO: Check this
        IA32_SMBASE,
        IA32_MISC_PACKAGE_CTLS,
        // TODO: IA32_X2APIC_DISABLE_STATUS
        IA32_PMC0,
        IA32_PMC1,
        IA32_PMC2,
        IA32_PMC3,
        IA32_PMC4,
        IA32_PMC5,
        IA32_PMC6,
        IA32_PMC7,
        IA32_PMC8,
        IA32_PMC9,
        IA32_CORE_CAPABILITIES,
        // TODO: IA32_UMWAIT_CONTROL

        // Disabled by CPUID for non-host CPU profiles
        IA32_MPERF,
        IA32_APERF,
        IA32_TSX_FORCE_ABORT,
        // Disabled via static IA32_ARCH_CAPABILITIES bit for non-host CPU profiles
        IA32_TSX_CTRL,
        // NOTE: IA32_MCU_OPT_CTRL must necessarily be available, due to
        // what we set in CPUID for some CPU profiles (inherit policy)

        // TODO: Don't know about IA32_SYSENTER_CS, IA32_SYSENTER_ESP,
        // IA32_SYSENTER_EIP
        //

        // TODO: Not sure if we can/should deny this MSR, but
        // it doesn't really make sense to have it available in
        // a virtualized environment
        //
        // If we keep it denied we should document that
        // even for 06_01H one cannot rely on the existence of this MSR
        IA32_MCG_CAP,
        // TODO: Also not sure if we may deny this MSR
        IA32_MCG_STATUS,
        // TODO: Can we deny this?
        IA32_MCG_CTL,
        // TODO: 0x180- 0x185 is reserved, we should not list these MSRS at all
        /// Disabled via CPUID for all non-host CPU profiles
        IA32_PERFEVTSEL0,
        IA32_PERFEVTSEL1,
        IA32_PERFEVTSEL2,
        IA32_PERFEVTSEL3,
        IA32_PERFEVTSEL4,
        IA32_PERFEVTSEL5,
        IA32_PERFEVTSEL6,
        IA32_PERFEVTSEL7,
        IA32_PERFEVTSEL8,
        IA32_PERFEVTSEL9,
        // TODO: 0x18a - 0x194 is reserved and should not be included in any list

        // TODO: 0x196, 197 is reserved and should not be included in any list
        //
        IA32_PERF_STATUS,
        IA32_PERF_CTL,
        // Disabled via CPUID for non-host profiles
        IA32_THERM_INTERRUPT,
        // Disabled via CPUID for non-host profiles
        IA32_THERM_STATUS,
        // TODO: Consider disabling IA32_MISC_ENABLE

        // Disabled via CPUID for non-host profiles
        IA32_ENERGY_PERF_BIAS,
        // Disabled via CPUID for non-host profiles
        IA32_PACKAGE_THERM_STATUS,
        // Disabled via CPUID for non-host profiles
        IA32_PACKAGE_THERM_INTERRUPT,
        IA32_DEBUGCTL,
        IA32_LER_FROM_IP,
        IA32_LER_TO_IP,
        IA32_LER_INFO,
        // TODO: Not sure about IA32_SMRR_PHYSBASE & IA32_SMRR_PHYSMASK
        IA32_MC_I_CTL2,
        // Disabled via CPUID for non-host profiles
        IA32_INTEGRITY_STATUS,
        IA32_FIXED_CTRI,
        // IA32_PERF_CAPABILITIES is an MSR-based feature thus not listed here

        // Disabled via CPUID for non-host profiles
        IA32_FIXED_CTR_CTRL,
        // Disabled via CPUID for non-host profiles
        IA32_PERF_GLOBAL_STATUS,
        // Disabled via CPUID for non-host profiles
        IA32_PERF_GLOBAL_CTRL,
        // Disabled via CPUID for non-host profiles
        IA32_PERF_GLOBAL_STATUS_RESET,
        // Disabled via CPUID for non-host profiles
        IA32_PERF_GLOBAL_STATUS_SET,
        // Disabled via CPUID for non-host profiles
        IA32_PERF_GLOBAL_INUSE,
        // TODO: Not sure about this one, but seems to be related to performance monitoring which
        // should be disabled for non-host CPU profiles.
        IA32_PEBS_ENABLE,
        IA32_MC0_CTL,
        IA32_MC0_STATUS,
        IA32_MC0_ADDR,
        IA32_MC0_MISC,
        IA32_MC1_CTL,
        IA32_MC1_STATUS,
        IA32_MC1_ADDR,
        IA32_MC1_MISC,
        IA32_MC2_CTL,
        IA32_MC2_STATUS,
        IA32_MC2_ADDR,
        IA32_MC2_MISC,
        IA32_MC3_CTL,
        IA32_MC3_STATUS,
        IA32_MC3_ADDR1,
        IA32_MC3_MISC,
        IA32_MC4_CTL,
        IA32_MC4_STATUS,
        IA32_MC4_ADDR,
        IA32_MC4_MISC,
        IA32_MC5_CTL,
        IA32_MC5_STATUS,
        IA32_MC5_ADDR,
        IA32_MC5_MISC,
        IA32_MC6_CTL,
        IA32_MC6_STATUS,
        IA32_MC6_ADDR1,
        IA32_MC6_MISC,
        IA32_MC7_CTL,
        IA32_MC7_STATUS,
        IA32_MC7_ADDR,
        IA32_MC7_MISC,
        IA32_MC8_CTL,
        IA32_MC8_STATUS,
        IA32_MC8_ADDR,
        IA32_MC8_MISC,
        IA32_MC9_CTL,
        IA32_MC9_STATUS,
        IA32_MC9_ADDR,
        IA32_MC9_MISC,
        IA32_MC10_CTL,
        IA32_MC10_STATUS,
        IA32_MC10_ADDR,
        IA32_MC10_MISC,
        IA32_MC11_CTL,
        IA32_MC11_STATUS,
        IA32_MC11_ADDR,
        IA32_MC11_MISC,
        IA32_MC12_CTL,
        IA32_MC12_STATUS,
        IA32_MC12_ADDR,
        IA32_MC12_MISC,
        IA32_MC13_CTL,
        IA32_MC13_STATUS,
        IA32_MC13_ADDR,
        IA32_MC13_MISC,
        IA32_MC14_CTL,
        IA32_MC14_STATUS,
        IA32_MC14_ADDR,
        IA32_MC14_MISC,
        IA32_MC15_CTL,
        IA32_MC15_STATUS,
        IA32_MC15_ADDR,
        IA32_MC15_MISC,
        IA32_MC16_CTL,
        IA32_MC16_STATUS,
        IA32_MC16_ADDR,
        IA32_MC16_MISC,
        IA32_MC17_CTL,
        IA32_MC17_STATUS,
        IA32_MC17_ADDR,
        IA32_MC17_MISC,
        IA32_MC18_CTL,
        IA32_MC18_STATUS,
        IA32_MC18_ADDR,
        IA32_MC18_MISC,
        IA32_MC19_CTL,
        IA32_MC19_STATUS,
        IA32_MC19_ADDR,
        IA32_MC19_MISC,
        IA32_MC20_CTL,
        IA32_MC20_STATUS,
        IA32_MC20_ADDR,
        IA32_MC20_MISC,
        IA32_MC21_CTL,
        IA32_MC21_STATUS,
        IA32_MC21_ADDR,
        IA32_MC21_MISC,
        IA32_MC22_CTL,
        IA32_MC22_STATUS,
        IA32_MC22_ADDR,
        IA32_MC22_MISC,
        IA32_MC23_CTL,
        IA32_MC23_STATUS,
        IA32_MC23_ADDR,
        IA32_MC23_MISC,
        IA32_MC24_CTL,
        IA32_MC24_STATUS,
        IA32_MC24_ADDR,
        IA32_MC24_MISC,
        IA32_MC25_CTL,
        IA32_MC25_STATUS,
        IA32_MC25_ADDR,
        IA32_MC25_MISC,
        IA32_MC26_CTL,
        IA32_MC26_STATUS,
        IA32_MC26_ADDR,
        IA32_MC26_MISC,
        IA32_MC27_CTL,
        IA32_MC27_STATUS,
        IA32_MC27_ADDR,
        IA32_MC27_MISC,
        IA32_MC28_CTL,
        IA32_MC28_STATUS,
        IA32_MC28_ADDR,
        IA32_MC28_MISC,
        IA32_MC29_CTL,
        IA32_MC29_STATUS,
        IA32_MC29_ADDR,
        IA32_MC29_MISC,
        IA32_MC30_CTL,
        IA32_MC30_STATUS,
        IA32_MC30_ADDR,
        IA32_MC30_MISC,
        IA32_MC31_CTL,
        IA32_MC31_STATUS,
        IA32_MC31_ADDR,
        IA32_MC31_MISC,
        IA32_A_PMC0,
        IA32_A_PMC1,
        IA32_A_PMC2,
        IA32_A_PMC3,
        IA32_A_PMC4,
        IA32_A_PMC5,
        IA32_A_PMC6,
        IA32_A_PMC7,
        IA32_A_PMC8,
        IA32_A_PMC9,
        IA32_MCG_EXT_CTL,
        // SGX is disabled via CPUID for non-host CPU profiles
        IA32_SGX_SVN_STATUS,
        // Disabled via CPUID for non-host CPU profiles
        IA32_RTIT_OUTPUT_BASE,
        // Disabled via CPUID for non-host CPU profiles
        IA32_RTIT_OUTPUT_MASK_PTRS,
        // Disabled via CPUID for non-host CPU profiles
        IA32_RTIT_CTL,
        // Disabled via CPUID for non-host CPU profiles
        IA32_RTIT_STATUS,
        // Disabled via CPU profiles
        IA32_RTIT_CR3_MATCH,
        IA32_RTIT_ADDR0_A,
        IA32_RTIT_ADDR0_B,
        IA32_RTIT_ADDR1_A,
        IA32_RTIT_ADDR1_B,
        IA32_RTIT_ADDR2_A,
        IA32_RTIT_ADDR2_B,
        IA32_RTIT_ADDR3_A,
        IA32_RTIT_ADDR3_B,
        // Disabled via CPUID for non-host CPU profiles
        IA32_DS_AREA,
        // TODO: IA32_TSC_DEADLINE should be available because the TSC_DEADLINE CPUID bit
        // is set by CHV unconditionally. The availability of this MSR probably needs to be
        // handled by CHV itself and not the CPU profiles

        // Disabled via CPUID for non-host CPU profiles
        IA32_PKRS,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PM_ENABLE,
        // Disabled via CPUID for non-host CPU profiles
        IA32_HWP_CAPABILITIES,
        // Disabled via CPUID for non-host CPU profiles
        IA32_HWP_REQUEST_PKG,
        // Disabled via CPUID for non-host CPU profiles
        IA32_HWP_INTERRUPT,
        // Disabled via CPUID for non-host CPU profiles
        IA32_HWP_REQUEST,
        // TODO: Can we also deny IA32_PECI_HWP_REQUEST_INFO?

        // Disabled via CPUID for non-host CPU profiles
        IA32_HWP_CTL,
        // Disabled via CPUID for non-host CPU profiles
        IA32_HWP_STATUS,
        // TODO: Currently permitted via IA32_ARCH_CAPABILITIES (bit 22),
        // but that bit should probably have policy Static(0) ?
        IA32_MCU_EXT_SERVICE,
        IA32_MCU_ROLLBACK_MIN_ID,
        // TODO: Not sure about IA32_MCU_STAGING_MBOX_ADDR
        IA32_ROLLBACK_SIGN_ID_0,
        IA32_ROLLBACK_SIGN_ID_1,
        IA32_ROLLBACK_SIGN_ID_2,
        IA32_ROLLBACK_SIGN_ID_3,
        IA32_ROLLBACK_SIGN_ID_4,
        IA32_ROLLBACK_SIGN_ID_5,
        IA32_ROLLBACK_SIGN_ID_6,
        IA32_ROLLBACK_SIGN_ID_7,
        IA32_ROLLBACK_SIGN_ID_8,
        IA32_ROLLBACK_SIGN_ID_9,
        IA32_ROLLBACK_SIGN_ID_10,
        IA32_ROLLBACK_SIGN_ID_11,
        IA32_ROLLBACK_SIGN_ID_12,
        IA32_ROLLBACK_SIGN_ID_13,
        IA32_ROLLBACK_SIGN_ID_14,
        IA32_ROLLBACK_SIGN_ID_15,
        // Disabled via CPUID for non-host CPU profiles
        IA32_TME_CAPABILITY,
        // Disabled via CPUID for non-host CPU profiles
        IA32_TME_ACTIVATE,
        // Disabled via CPUID for non-host CPU profiles
        IA32_TME_EXCLUDE_MASK,
        // Disabled via CPUID for non-host CPU profiles
        IA32_TME_EXCLUDE_BASE,
        // Disabled via CPUID for non-host CPU profiles
        IA32_UINTR_RR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_UINTR_HANDLER,
        // Disabled via CPUID for non-host CPU profiles
        IA32_UINTR_STACKADJUST,
        // Disabled via CPUID for non-host CPU profiles
        IA32_UINTR_MISC,
        // Disabled via CPUID for non-host CPU profiles
        IA32_UINTR_PD,
        // Disabled via CPUID for non-host CPU profiles
        IA32_UINTR_TT,
        // Disabled via CPUID for non-host CPU profiles
        IA32_COPY_STATUS,
        // Disabled via CPUID for non-host CPU profiles
        IA32_IWKEYBACKUP_STATUS,
        IA32_TME_CLEAR_SAVED_KEY,
        // Disabled via CPUID for non-host CPU profiles
        IA32_DEBUG_INTERFACE,
        // Disabled via CPUID for non-host CPU profiles
        IA32_L3_QOS_CFG,
        // Disabled via CPUID
        IA32_L2_QOS_CFG,
        // Disabled via CPUID
        IA32_L3_IO_QOS_CFG,
        // TODO: Not sure about IA32_RESOURCE_PRIORITY and IA32_RESOURCE_PRIORITY_PKG

        // Disabled via CPUID for non-host CPU profiles
        IA32_QM_EVTSEL,
        // Disabled via CPUID for non-host CPU profiles
        IA32_QM_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PQR_ASSOC,
        // Disabled via CPUID for non-host CPU profiles
        IA32_L3_MASK_0,
        IA32_L3_MASK_N,
        // Disabled via CPUID for non-host CPU profiles
        IA32_L2_MASK_0,
        // Disabled via CPUID for non-host CPU profiles
        IA32_L2_MASK_N,
        // Disabled via CPUID for non-host CPU profiles
        IA32_L2_QOS_EXT_BW_THRTL_I,
        // Disabled via CPUID for non-host CPU profiles
        IA32_BNDCFGS,
        // Disabled via CPUID for non-host CPU profiles
        IA32_COPY_LOCAL_TO_PLATFORM,
        // Disabled via CPUID for non-host CPU profiles
        IA32_COPY_PLATFORM_TO_LOCAL,
        // TODO: Not sure about IA32_PASID

        // Disabled via CPUID for non-host CPU profiles
        IA32_PKG_HDC_CTL,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PM_CTL1,
        // Disabled via CPUID for non-host CPU profiles
        IA32_THREAD_STALL,
        // Disabled via CPUID for non-host CPU profiles
        IA32_QOS_CORE_BW_THRTL_0,
        // Disabled via CPUID for non-host CPU profiles
        IA32_QOS_CORE_BW_THRTL_1,
        // TODO: Is it OK to disable this for CPU profiles?
        // Note that we have CPUID 0x7.EDX.[19] = 0 (ARCH_LBR)
        IA32_LBR_X_INFO,
        // TDX related.
        IA32_SEAMRR_BASE,
        // TDX related.
        IA32_SEAMRR_MASK,
        // Disabled via ARCH_CAPABILITIES for non-host CPU profiles
        IA32_MCU_CONTROL,
        IA32_LBR_CTL,
        IA32_LBR_DEPTH,
        IA32_LBR_X_FROM_IP,
        IA32_LBR_X_TO_IP,
        // Disabled via CPUID for non-host CPU profiles
        IA32_HW_FEEDBACK_PTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_HW_FEEDBACK_CONFIG,
        // Disabled via CPUID for non-host CPU profiles
        IA32_HW_FEEDBACK_THREAD_CHAR,
        IA32_HW_FEEDBACK_THREAD_CONFIG,
        IA32_HRESET_ENABLE,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP0_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP0_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP0_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP1_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP1_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP1_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP2_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP2_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP2_CFG_B,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP2_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP3_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP3_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP3_CFG_B,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP3_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP4_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP4_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP4_CFG_B,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP4_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP5_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP5_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP5_CFG_B,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP5_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP6_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP6_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP6_CFG_B,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP6_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP7_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP7_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP7_CFG_B,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP7_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP8_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP8_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP9_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_GP9_CFG_A,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_FX0_CTR,
        IA32_PMC_FX0_CFG_B,
        IA32_PMC_FX0_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_FX1_CTR,
        IA32_PMC_FX1_CFG_B,
        IA32_PMC_FX1_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_FX2_CTR,
        IA32_PMC_FX2_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_FX3_CTR,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_FX4_CTR,
        IA32_PMC_FX4_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_FX5_CTR,
        IA32_PMC_FX5_CFG_C,
        // Disabled via CPUID for non-host CPU profiles
        IA32_PMC_FX6_CTR,
        IA32_PMC_FX6_CFG_C,
        // TODO: Check IA32_UARCH_MISC_CTL
    ];
}
