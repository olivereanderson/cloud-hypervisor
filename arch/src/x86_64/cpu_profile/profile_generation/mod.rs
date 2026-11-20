pub mod cpuid;
pub mod msrs;

use std::ops::{BitAnd, BitOr, RangeInclusive, Shl};

use hypervisor::arch::x86::{CpuIdEntry, MsrEntry};
use hypervisor::{CpuVendor, Hypervisor, HypervisorError};

use crate::x86_64::Error;
use crate::x86_64::cpu_profile::CpuProfile;

pub const AMD_AND_INTEL: &[CpuVendor] = &[CpuVendor::AMD, CpuVendor::Intel];

/// Describes a policy for how a given range of bits within a CPUID output register or
/// feature MSR should be recorded when the CPU profile is generated.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ProfilePolicy<T: Copy> {
    /// Write the value extracted from the hardware and the hypervisor into the CPU profile.
    Inherit,
    /// The CPU profile should not alter the value upon applying the CPU profile at runtime.
    Passthrough,
    /// Write the given fixed value into the CPU profile.
    Static(T),
}

/// A description of a range of bits within a CPUID output register or feature MSR.
#[derive(Clone, Debug)]
pub struct ValueDefinition<T: Copy> {
    /// A short name describing the value (e.g. "avx2")
    pub name: &'static str,
    // The range of bits that back the value.
    pub bits_range: RangeInclusive<u8>,
    /// The policy corresponding to this value when building CPU profiles.
    pub policy: ProfilePolicy<T>,
    // The CPU vendors that have documented this value.
    pub vendors: &'static [CpuVendor],
}

impl<T: Copy + PartialEq + Eq> ValueDefinition<T> {
    fn mask(definitions: &[Self], vendor: CpuVendor) -> Option<T>
    where
        T: Shl<u8, Output = T>,
        T: BitOr<Output = T>,
        T: From<u8>,
    {
        definitions
            .iter()
            .filter_map(|def| {
                (def.policy == ProfilePolicy::Passthrough && def.vendors.contains(&vendor))
                    .then_some(&def.bits_range)
                    .map(bit_range_mask)
            })
            .reduce(|acc, next| acc | next)
    }

    fn replacements(definitions: &[Self], value: T, vendor: CpuVendor) -> T
    where
        T: Shl<u8, Output = T>,
        T: BitOr<Output = T>,
        T: From<u8>,
        T: BitAnd<Output = T>,
    {
        definitions
            .iter()
            .filter_map(|def| {
                def.vendors.contains(&vendor).then_some(match def.policy {
                    ProfilePolicy::Inherit => value & bit_range_mask::<T>(&def.bits_range),
                    ProfilePolicy::Static(overwrite_value) => {
                        overwrite_value << *def.bits_range.start()
                    }
                    ProfilePolicy::Passthrough => T::from(0_u8),
                })
            })
            .fold(T::from(0_u8), |acc, partial_value| acc | partial_value)
    }
}

fn bit_range_mask<T>(range: &RangeInclusive<u8>) -> T
where
    T: Shl<u8, Output = T>,
    T: BitOr<Output = T>,
    T: From<u8>,
{
    (range.clone()).fold(T::from(0_u8), |acc, next| acc | ((T::from(1_u8)) << next))
}

fn generate_cpu_profile_with(
    supported_cpuid: Vec<CpuIdEntry>,
    supported_feature_msrs: &[MsrEntry],
    supported_msr_index_list: &[u32],
    vendor: CpuVendor,
) -> CpuProfile {
    CpuProfile {
        cpuid: cpuid::generate_cpuid_profile_data_with(supported_cpuid, vendor),
        msr: msrs::generate_msrs_profile_data_with(
            supported_feature_msrs,
            supported_msr_index_list,
            vendor,
        ),
    }
}

/// Get as many of the supported CPUID entries from the hypervisor as possible.
fn supported_cpuid(hypervisor: &dyn Hypervisor) -> Result<Vec<CpuIdEntry>, HypervisorError> {
    // Try enabling AMX before querying the hypervisor for CPUID.
    hypervisor.enable_amx_state_components().or_else(|e| {
        if let HypervisorError::CouldNotEnableAmxStateComponents(amx_err) = &e
            && !matches!(
                amx_err,
                hypervisor::arch::x86::AmxGuestSupportError::AmxGuestTileRequest { .. }
            )
        {
            // AMX is not supported on the host so we do not need to enable it
            Ok(())
        } else {
            Err(e)
        }
    })?;
    hypervisor.get_supported_cpuid()
}

pub fn generate_cpu_profile(hypervisor: &dyn Hypervisor) -> Result<CpuProfile, Error> {
    let cpuid = supported_cpuid(hypervisor).map_err(Error::GenerateCpuProfile)?;
    let feature_msrs = hypervisor
        .get_feature_msrs()
        .map_err(Error::GenerateCpuProfile)?;
    let msr_index_list = hypervisor
        .get_msr_index_list()
        .map_err(Error::GenerateCpuProfile)?;
    let vendor = hypervisor.get_cpu_vendor();

    Ok(generate_cpu_profile_with(
        cpuid,
        &feature_msrs,
        &msr_index_list,
        vendor,
    ))
}

#[cfg(test)]
mod unit_tests {
    use std::ops::RangeInclusive;

    use hypervisor::CpuVendor;
    use hypervisor::arch::x86::msr_index::MSR_IA32_VMX_TRUE_EXIT_CTLS;
    use hypervisor::arch::x86::{CpuIdEntry, MsrEntry};

    use crate::x86_64::cpu_profile::profile_generation::generate_cpu_profile_with;

    #[test]
    fn cpu_profile_generation_skylake_snapshot_test() {
        const CPUID_LEAF_07_EBX_AVX512_BIT: u8 = 16;
        const CPUID_LEAF_07H_EBX_MPX_BIT: u8 = 14;

        const AVX512_XFEAT_BITS_RANGE: RangeInclusive<u8> = 5..=7;
        const MPX_XFEAT_BITS_RANGE: RangeInclusive<u8> = 3..=4;

        const ALLOWED_ONE_LOAD_PERF_GLOBAL_CTRL_BIT: u8 = 44;

        let leaf_is_zero = |entry: &CpuIdEntry| {
            (entry.eax == entry.ebx)
                && (entry.ebx == entry.ecx)
                && (entry.ecx == entry.edx)
                && (entry.edx == 0)
        };

        // The following values where dumped from a Skylake server + KVM (kernel version 7.2.8).
        let cpuid = vec![
            CpuIdEntry {
                function: 0x0,
                index: 0x0,
                flags: 0x0,
                eax: 0x16,
                ebx: 0x756e6547,
                ecx: 0x6c65746e,
                edx: 0x49656e69,
            },
            CpuIdEntry {
                function: 0x1,
                index: 0x0,
                flags: 0x0,
                eax: 0x50654,
                ebx: 0x5400800,
                ecx: 0xf7fab223,
                edx: 0xf8bfbff,
            },
            CpuIdEntry {
                function: 0x2,
                index: 0x0,
                flags: 0x0,
                eax: 0x76036301,
                ebx: 0xf0b5ff,
                ecx: 0x0,
                edx: 0xc30000,
            },
            CpuIdEntry {
                function: 0x3,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x4,
                index: 0x0,
                flags: 0x1,
                eax: 0x7c004121,
                ebx: 0x1c0003f,
                ecx: 0x3f,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x4,
                index: 0x1,
                flags: 0x1,
                eax: 0x7c004122,
                ebx: 0x1c0003f,
                ecx: 0x3f,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x4,
                index: 0x2,
                flags: 0x1,
                eax: 0x7c004143,
                ebx: 0x3c0003f,
                ecx: 0x3ff,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x4,
                index: 0x3,
                flags: 0x1,
                eax: 0x7c0fc163,
                ebx: 0x280003f,
                ecx: 0x8fff,
                edx: 0x4,
            },
            CpuIdEntry {
                function: 0x4,
                index: 0x4,
                flags: 0x1,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x5,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x6,
                index: 0x0,
                flags: 0x0,
                eax: 0x4,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x7,
                index: 0x0,
                flags: 0x1,
                eax: 0x0,
                ebx: 0xd19f6ffb,
                ecx: 0xc,
                edx: 0xbc000400,
            },
            CpuIdEntry {
                function: 0x8,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x9,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xa,
                index: 0x0,
                flags: 0x0,
                eax: 0x7300402,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x8603,
            },
            CpuIdEntry {
                function: 0xb,
                index: 0x0,
                flags: 0x1,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x5,
            },
            CpuIdEntry {
                function: 0xc,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x0,
                flags: 0x1,
                eax: 0x2ff,
                ebx: 0xa88,
                ecx: 0xa88,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x1,
                flags: 0x1,
                eax: 0xf,
                ebx: 0xa08,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x2,
                flags: 0x1,
                eax: 0x100,
                ebx: 0x240,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x3,
                flags: 0x1,
                eax: 0x40,
                ebx: 0x3c0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x4,
                flags: 0x1,
                eax: 0x40,
                ebx: 0x400,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x5,
                flags: 0x1,
                eax: 0x40,
                ebx: 0x440,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x6,
                flags: 0x1,
                eax: 0x200,
                ebx: 0x480,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x7,
                flags: 0x1,
                eax: 0x400,
                ebx: 0x680,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xd,
                index: 0x9,
                flags: 0x1,
                eax: 0x8,
                ebx: 0xa80,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xe,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0xf,
                index: 0x0,
                flags: 0x1,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x10,
                index: 0x0,
                flags: 0x1,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x11,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x12,
                index: 0x0,
                flags: 0x1,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x13,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x14,
                index: 0x0,
                flags: 0x1,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x15,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x16,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x80000000,
                index: 0x0,
                flags: 0x0,
                eax: 0x80000008,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x80000001,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x121,
                edx: 0x2c100800,
            },
            CpuIdEntry {
                function: 0x80000002,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x80000003,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x80000004,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x80000005,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x80000006,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x1006040,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x80000007,
                index: 0x0,
                flags: 0x0,
                eax: 0x0,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x100,
            },
            CpuIdEntry {
                function: 0x80000008,
                index: 0x0,
                flags: 0x0,
                eax: 0x2e302e,
                ebx: 0x4100d000,
                ecx: 0x0,
                edx: 0x0,
            },
            CpuIdEntry {
                function: 0x40000000,
                index: 0x0,
                flags: 0x0,
                eax: 0x40000001,
                ebx: 0x4b4d564b,
                ecx: 0x564b4d56,
                edx: 0x4d,
            },
            CpuIdEntry {
                function: 0x40000001,
                index: 0x0,
                flags: 0x0,
                eax: 0x1007efb,
                ebx: 0x0,
                ecx: 0x0,
                edx: 0x0,
            },
        ];

        let feature_msrs = &[
            MsrEntry {
                index: 0x480,
                data: 0xd8100011e57ed0,
            },
            MsrEntry {
                index: 0x481,
                data: 0xff00000016,
            },
            MsrEntry {
                index: 0x482,
                data: 0xfff9fffe0401e172,
            },
            MsrEntry {
                index: 0x483,
                data: 0xffffff00036dff,
            },
            MsrEntry {
                index: 0x484,
                data: 0x1f3ff000011ff,
            },
            MsrEntry {
                index: 0x485,
                data: 0x20000165,
            },
            MsrEntry {
                index: 0x486,
                data: 0x80000021,
            },
            MsrEntry {
                index: 0x487,
                data: 0xffffffff,
            },
            MsrEntry {
                index: 0x488,
                data: 0x2000,
            },
            MsrEntry {
                index: 0x489,
                data: 0x776fff,
            },
            MsrEntry {
                index: 0x48a,
                data: 0x32,
            },
            MsrEntry {
                index: 0x48b,
                data: 0x2537bff00000000,
            },
            MsrEntry {
                index: 0x48c,
                data: 0xf0106734041,
            },
            MsrEntry {
                index: 0x48d,
                data: 0xff00000016,
            },
            MsrEntry {
                index: 0x48e,
                data: 0xfff9fffe04006172,
            },
            MsrEntry {
                index: 0x48f,
                data: 0xffffff00036dfb,
            },
            MsrEntry {
                index: 0x490,
                data: 0x1f3ff000011fb,
            },
            MsrEntry {
                index: 0x491,
                data: 0x1,
            },
            MsrEntry {
                index: 0x8b,
                data: 0x200700600000000,
            },
            MsrEntry {
                index: 0x10a,
                data: 0x400000000c00004c,
            },
            MsrEntry {
                index: 0x345,
                data: 0x2005,
            },
            MsrEntry {
                index: 0xce,
                data: 0x80000000,
            },
        ];

        let msr_index_list = &[
            0x174, 0x175, 0x176, 0xc0000081, 0xc0000083, 0xc0000102, 0xc0000084, 0xc0000082, 0x10,
            0x277, 0x3a, 0xd90, 0xc0000103, 0x48, 0x1d9, 0x1db, 0x1dc, 0x1dd, 0x1de, 0x309, 0x30a,
            0x30b, 0x38d, 0x38e, 0x38f, 0x3f1, 0x600, 0xc1, 0xc2, 0xc3, 0xc4, 0x186, 0x187, 0x188,
            0x189, 0x12, 0x11, 0x4b564d01, 0x4b564d00, 0x40000000, 0x40000001, 0x40000020,
            0x40000021, 0x40000022, 0x40000023, 0x40000100, 0x40000101, 0x40000102, 0x40000103,
            0x40000104, 0x40000105, 0x40000003, 0x40000002, 0x40000010, 0x40000080, 0x400000b0,
            0x40000073, 0x40000106, 0x40000107, 0x40000108, 0x40000118, 0x400000ff, 0x400000f1,
            0x400000f2, 0x400000f3, 0x400000f4, 0x400000f5, 0x4b564d02, 0x4b564d03, 0x4b564d04,
            0x4b564d06, 0x4b564d07, 0x3b, 0x6e0, 0x10a, 0x345, 0x1a0, 0x17a, 0x17b, 0x4d0, 0x9e,
            0x34, 0xce, 0x140, 0x1fc, 0x8b, 0x480, 0x48d, 0x48e, 0x48f, 0x490, 0x485, 0x486, 0x488,
            0x48a, 0x48b, 0x48c, 0x491, 0xc0010015, 0x4b564d05, 0x2ff, 0x200, 0x201, 0x202, 0x203,
            0x204, 0x205, 0x206, 0x207, 0x208, 0x209, 0x20a, 0x20b, 0x20c, 0x20d, 0x20e, 0x20f,
            0x250, 0x258, 0x259, 0x268, 0x269, 0x26a, 0x26b, 0x26c, 0x26d, 0x26e, 0x26f,
        ];

        let cpu_profile = generate_cpu_profile_with(
            cpuid.clone(),
            feature_msrs,
            msr_index_list,
            CpuVendor::Intel,
        );

        let modified_cpuid = cpu_profile
            .adjust_cpuid(cpuid.clone(), false, CpuVendor::Intel)
            .unwrap();

        let required_msr_uodates = cpu_profile
            .required_msr_updates(feature_msrs, msr_index_list, false, true)
            .unwrap();

        let original_leaf_7h_ebx = cpuid
            .iter()
            .find(|entry| entry.function == 0x7 && entry.index == 0)
            .unwrap()
            .ebx;

        let original_leaf_dh_eax = cpuid
            .iter()
            .find(|entry| entry.function == 0xd && entry.index == 0)
            .unwrap()
            .eax;

        let modified_leaf_7h_ebx = modified_cpuid
            .iter()
            .find(|entry| entry.function == 0x7 && entry.index == 0)
            .unwrap()
            .ebx;
        let modified_leaf_dh_eax = modified_cpuid
            .iter()
            .find(|entry| entry.function == 0xd && entry.index == 0)
            .unwrap()
            .eax;

        // Both have AVX512 listed as supported
        assert!(original_leaf_7h_ebx & (1 << CPUID_LEAF_07_EBX_AVX512_BIT) != 0);
        assert!(modified_leaf_7h_ebx & (1 << CPUID_LEAF_07_EBX_AVX512_BIT) != 0);

        for bit in AVX512_XFEAT_BITS_RANGE {
            assert!(original_leaf_dh_eax & (1 << bit) != 0);
            assert!(modified_leaf_dh_eax & (1 << bit) != 0);
        }

        // Same AVX512 XSAVE state component information
        for sub_leaf_idx in AVX512_XFEAT_BITS_RANGE {
            let original_component_info = cpuid
                .iter()
                .find(|entry| entry.function == 0xd && entry.index == u32::from(sub_leaf_idx))
                .unwrap();
            let adjusted_component_info = modified_cpuid
                .iter()
                .find(|entry| entry.function == 0xd && entry.index == u32::from(sub_leaf_idx))
                .unwrap();

            assert_eq!(original_component_info, adjusted_component_info);
        }

        // Original indicates MPX support, but not modified
        assert!(original_leaf_7h_ebx & (1 << CPUID_LEAF_07H_EBX_MPX_BIT) != 0);
        assert!(modified_leaf_7h_ebx & (1 << CPUID_LEAF_07H_EBX_MPX_BIT) == 0);

        for bit in MPX_XFEAT_BITS_RANGE {
            assert!(original_leaf_dh_eax & (1 << bit) != 0);
            assert!(modified_leaf_dh_eax & (1 << bit) == 0);
        }

        // The MPX XSAVE state component information leaves should be 0 (or not existing) after
        // applying the CPU profile
        for sub_leaf_idx in MPX_XFEAT_BITS_RANGE {
            let Some(adjusted_component_info) = modified_cpuid
                .iter()
                .find(|entry| entry.function == 0xd && entry.index == u32::from(sub_leaf_idx))
            else {
                continue;
            };
            assert!(leaf_is_zero(adjusted_component_info));
        }

        // Check that the computed MAX std leaf makes sense
        let original_max_std_leaf = cpuid
            .iter()
            .find_map(|entry| (entry.function == 0 && entry.index == 0).then_some(entry.eax))
            .unwrap();
        let adjusted_max_std_leaf = modified_cpuid
            .iter()
            .find_map(|entry| (entry.function == 0 && entry.index == 0).then_some(entry.eax))
            .unwrap();

        assert!(adjusted_max_std_leaf <= original_max_std_leaf);
        for leaf in &modified_cpuid {
            if leaf.function <= original_max_std_leaf && !leaf_is_zero(leaf) {
                assert!(leaf.function <= adjusted_max_std_leaf);
            }
        }

        // Check that the computed max extended leaf makes sense
        let original_max_ext_leaf = cpuid
            .iter()
            .find_map(|entry| {
                (entry.function == 0x8000_0000 && entry.index == 0).then_some(entry.eax)
            })
            .unwrap();
        let adjusted_max_ext_leaf = modified_cpuid
            .iter()
            .find_map(|entry| {
                (entry.function == 0x8000_0000 && entry.index == 0).then_some(entry.eax)
            })
            .unwrap();

        assert!(adjusted_max_ext_leaf <= original_max_ext_leaf);
        for leaf in &modified_cpuid {
            if leaf.function <= original_max_ext_leaf && !leaf_is_zero(leaf) {
                assert!(leaf.function <= adjusted_max_ext_leaf);
            }
        }

        // Check that the generated profile also zeroes bits in feature MSRs as expected
        let true_exit_ctls_msr = feature_msrs
            .iter()
            .find(|msr| msr.index == MSR_IA32_VMX_TRUE_EXIT_CTLS)
            .unwrap();

        let modified_true_exit_ctls_msr = required_msr_uodates
            .feature_msrs
            .iter()
            .find(|msr| msr.index == MSR_IA32_VMX_TRUE_EXIT_CTLS)
            .unwrap();

        assert!(true_exit_ctls_msr.data & (1_u64 << ALLOWED_ONE_LOAD_PERF_GLOBAL_CTRL_BIT) != 0);
        assert!(
            modified_true_exit_ctls_msr.data & (1_u64 << ALLOWED_ONE_LOAD_PERF_GLOBAL_CTRL_BIT)
                == 0
        );
    }
}
