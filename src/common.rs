/// A macro providing the common types for both a Solana program and a loader of it.
#[macro_export]
macro_rules! common_stub_types {
    () => {
        pub const PUBKEY_BYTES: usize = 32;
        #[repr(C)]
        pub struct CPubkey(pub [u8; PUBKEY_BYTES]);

        impl CPubkey {
            pub const fn as_array(&self) -> &[u8; PUBKEY_BYTES] {
                &self.0
            }
        }

        impl AsRef<[u8]> for CPubkey {
            fn as_ref(&self) -> &[u8] {
                &self.0[..]
            }
        }

        impl AsMut<[u8]> for CPubkey {
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0[..]
            }
        }

        impl From<[u8; 32]> for CPubkey {
            #[inline]
            fn from(from: [u8; 32]) -> Self {
                Self(from)
            }
        }

        impl From<&[u8; 32]> for CPubkey {
            #[inline]
            fn from(from: &[u8; 32]) -> Self {
                Self(*from)
            }
        }

        impl PartialEq<[u8; 32]> for CPubkey {
            fn eq(&self, other: &[u8; 32]) -> bool {
                &self.0 == other
            }
        }

        #[repr(C)]
        pub struct CProcessedSiblingInstruction {
            pub data_len: u64,
            pub accounts_len: u64,
        }

        #[repr(C)]
        #[derive(Clone)]
        pub struct CAccountMeta {
            pub pubkey: *const CPubkey,
            pub is_writable: bool,
            pub is_signer: bool,
        }

        impl Default for CAccountMeta {
            fn default() -> Self {
                Self {
                    pubkey: &CPubkey::from([0u8; 32]),
                    is_writable: false,
                    is_signer: false,
                }
            }
        }

        #[repr(C)]
        #[derive(Debug)]
        pub struct CAccountInfo {
            // Public key of the account.
            pub key: *const CPubkey,

            // Number of lamports owned by this account.
            pub lamports: *const u64,

            // Length of data in bytes.
            pub data_len: u64,

            // On-chain data within this account.
            pub data: *const u8,

            // Program that owns this account.
            pub owner: *const CPubkey,

            // The epoch at which this account will next owe rent.
            pub rent_epoch: u64,

            // Transaction was signed by this account's key?
            pub is_signer: bool,

            // Is the account writable?
            pub is_writable: bool,

            // This account's data contains a loaded program (and is now read-only).
            pub executable: bool,
        }

        #[repr(C)]
        #[derive(Debug)]
        pub struct CInstruction {
            /// Public key of the program.
            pub program_id: *const CPubkey,

            /// Accounts expected by the program instruction.
            pub accounts: *const CAccountMeta,

            /// Number of accounts expected by the program instruction.
            pub accounts_len: u64,

            /// Data expected by the program instruction.
            pub data: *const u8,

            /// Length of the data expected by the program instruction.
            pub data_len: u64,
        }

        #[repr(C)]
        pub struct SyscallStubsApi {
            pub sol_log_: extern "C" fn(message: *const u8, len: u64),
            pub sol_log_compute_units_: extern "C" fn(),
            pub sol_remaining_compute_units: extern "C" fn() -> u64,
            pub sol_invoke_signed_c: extern "C" fn(
                instruction_addr: *const u8,
                account_infos_addr: *const u8,
                account_infos_len: u64,
                signers_seeds_addr: *const u8,
                signers_seeds_len: u64,
            ) -> u64,
            pub sol_get_clock_sysvar: extern "C" fn(addr: *mut u8) -> u64,
            pub sol_get_epoch_schedule_sysvar: extern "C" fn(addr: *mut u8) -> u64,
            pub sol_get_fees_sysvar: extern "C" fn(addr: *mut u8) -> u64,
            pub sol_get_rent_sysvar: extern "C" fn(addr: *mut u8) -> u64,
            pub sol_get_last_restart_slot: extern "C" fn(addr: *mut u8) -> u64,
            pub sol_get_sysvar: extern "C" fn(
                sysvar_id_addr: *const u8,
                result: *mut u8,
                offset: u64,
                length: u64,
            ) -> u64,
            pub sol_memcpy_: extern "C" fn(dst: *mut u8, src: *const u8, n: u64),
            pub sol_memmove_: extern "C" fn(dst: *mut u8, src: *const u8, n: u64),
            pub sol_memcmp_: extern "C" fn(s1: *const u8, s2: *const u8, n: u64, result: *mut i32),
            pub sol_memset_: extern "C" fn(s: *mut u8, c: u8, n: u64),
            pub sol_get_return_data:
                extern "C" fn(data: *mut u8, length: u64, program_id: *mut CPubkey) -> u64,
            pub sol_set_return_data: extern "C" fn(data: *const u8, length: u64),
            pub sol_log_data: extern "C" fn(data: *const u8, data_len: u64),
            pub sol_get_processed_sibling_instruction: extern "C" fn(
                index: u64,
                meta: *mut CProcessedSiblingInstruction,
                program_id: *mut CPubkey,
                data: *mut u8,
                accounts: *mut CAccountMeta,
            ) -> u64,
            pub sol_get_stack_height: extern "C" fn() -> u64,
            pub sol_get_epoch_rewards_sysvar: extern "C" fn(addr: *mut u8) -> u64,
            pub sol_get_epoch_stake: extern "C" fn(vote_address: *const u8) -> u64,
        }
    };
}
