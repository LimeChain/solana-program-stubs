#[macro_export]
macro_rules! common_stub_types {
    () => {
        #[repr(C)]
        pub struct CInstruction<'a> {
            pub program_id: Pubkey,
            pub accounts_ptr: *const AccountMeta,
            pub accounts_len: usize,
            pub data_ptr: *const u8,
            pub data_len: usize,
            // single thread only, tie to 'a lifetime
            marker: std::marker::PhantomData<&'a *const ()>,
        }

        impl<'a> From<&'a Instruction> for CInstruction<'a> {
            fn from(instruction: &'a Instruction) -> Self {
                CInstruction {
                    program_id: instruction.program_id,
                    accounts_ptr: instruction.accounts.as_ptr(),
                    accounts_len: instruction.accounts.len(),
                    data_ptr: instruction.data.as_ptr(),
                    data_len: instruction.data.len(),
                    marker: std::marker::PhantomData,
                }
            }
        }

        impl<'a> From<CInstruction<'a>> for Instruction {
            fn from(cinstruction: CInstruction) -> Self {
                let accounts: Vec<AccountMeta> = Vec::from(unsafe {
                    std::slice::from_raw_parts(cinstruction.accounts_ptr, cinstruction.accounts_len)
                });
                let data = Vec::from(unsafe {
                    std::slice::from_raw_parts(cinstruction.data_ptr, cinstruction.data_len)
                });
                Instruction {
                    program_id: cinstruction.program_id,
                    accounts,
                    data,
                }
            }
        }

        #[repr(C)]
        pub struct CBytes<'a> {
            pub ptr: *const u8,
            pub len: usize,
            marker: std::marker::PhantomData<&'a *const ()>,
        }

        #[repr(C)]
        pub struct CBytesArray<'a> {
            pub ptr: *const CBytes<'a>,
            pub len: usize,
            marker: std::marker::PhantomData<&'a *const ()>,
        }

        impl<'a> CBytesArray<'a> {
            pub fn from(input: &'a [&'a [u8]]) -> (CBytesArray<'a>, Vec<CBytes<'a>>) {
                let mut all_cbytes = Vec::new();

                for slice in input {
                    all_cbytes.push(CBytes {
                        ptr: slice.as_ptr(),
                        len: slice.len(),
                        marker: std::marker::PhantomData,
                    });
                }

                (
                    CBytesArray {
                        ptr: all_cbytes.as_ptr(),
                        len: all_cbytes.len(),
                        marker: std::marker::PhantomData,
                    },
                    all_cbytes,
                )
            }

            pub fn to_array_array(c: &'a CBytesArray) -> Vec<&'a [u8]> {
                let mut result = Vec::new();
                for i in 0..c.len {
                    let c_inner = unsafe { &*c.ptr.add(i) };
                    let slice = unsafe { std::slice::from_raw_parts(c_inner.ptr, c_inner.len) };
                    result.push(slice);
                }
                result
            }
        }

        #[repr(C)]
        pub struct CBytesArrayArray<'a> {
            pub ptr: *const CBytesArray<'a>,
            pub len: usize,
            marker: std::marker::PhantomData<&'a *const ()>,
        }

        impl<'a> CBytesArrayArray<'a> {
            pub fn from(
                input: &'a [&'a [&'a [u8]]],
            ) -> (
                CBytesArrayArray<'a>,
                Vec<CBytesArray<'a>>,
                Vec<Vec<CBytes<'a>>>,
            ) {
                let mut outer = Vec::new();
                let mut all_cbytes = Vec::new(); // To hold all CBytes flat

                for inner in input {
                    let mut inner_cbytes = Vec::new();
                    for slice in *inner {
                        inner_cbytes.push(CBytes {
                            ptr: slice.as_ptr(),
                            len: slice.len(),
                            marker: std::marker::PhantomData,
                        });
                    }

                    let inner_ptr = inner_cbytes.as_ptr();
                    all_cbytes.push(inner_cbytes); // store to keep memory alive

                    outer.push(CBytesArray {
                        ptr: inner_ptr,
                        len: inner.len(),
                        marker: std::marker::PhantomData,
                    });
                }

                let outer_ptr = outer.as_ptr();
                (
                    CBytesArrayArray {
                        ptr: outer_ptr,
                        len: input.len(),
                        marker: std::marker::PhantomData,
                    },
                    outer,
                    all_cbytes,
                )
            }

            pub fn to_array_array_array(c: &'a CBytesArrayArray) -> Vec<Vec<&'a [u8]>> {
                let mut result = Vec::new();
                for i in 0..c.len {
                    let c_inner = unsafe { &*c.ptr.add(i) };
                    let mut inner = Vec::new();

                    for j in 0..c_inner.len {
                        let c_bytes = unsafe { &*c_inner.ptr.add(j) };
                        let slice = unsafe { std::slice::from_raw_parts(c_bytes.ptr, c_bytes.len) };
                        inner.push(slice);
                    }

                    result.push(inner);
                }
                result
            }

            pub fn convert(input: &'a Vec<Vec<&'a [u8]>>) -> Vec<&'a [&'a [u8]]> {
                input.iter().map(|inner| inner.as_slice()).collect()
            }
        }

        #[repr(C)]
        pub struct CAccountInfoSlice<'a, 'b> {
            pub ptr: *const CAccountInfo<'b>,
            pub len: usize,
            marker: std::marker::PhantomData<&'a *const ()>,
        }

        #[repr(C)]
        pub struct CAccountInfo<'b> {
            // A &Pubkey must map to *const u8
            pub key: *const u8, // [u8; 32]
            pub lamports: *mut u64,
            pub data: *mut u8,
            pub data_len: usize,
            // A &Pubkey must map to *const u8
            pub owner: *const u8, // [u8; 32]
            pub rent_epoch: u64,
            pub is_signer: bool,
            pub is_writable: bool,
            pub executable: bool,
            marker: std::marker::PhantomData<&'b *const ()>,
        }

        impl<'a, 'b> CAccountInfoSlice<'a, 'b> {
            #[allow(dead_code)]
            fn from(
                ais: &'a [AccountInfo<'b>],
            ) -> (CAccountInfoSlice<'a, 'b>, Vec<CAccountInfo<'b>>) {
                let mut c_infos = Vec::with_capacity(ais.len());

                for ai in ais {
                    let lamports_ref = &mut *ai.lamports.borrow_mut();
                    let data_ref = &mut *ai.data.borrow_mut();

                    c_infos.push(CAccountInfo {
                        key: ai.key.as_ref().as_ptr(),
                        lamports: *lamports_ref as *mut u64,
                        data: data_ref.as_mut_ptr(),
                        data_len: data_ref.len(),
                        owner: ai.owner.as_ref().as_ptr(),
                        rent_epoch: ai.rent_epoch,
                        is_signer: ai.is_signer,
                        is_writable: ai.is_writable,
                        executable: ai.executable,
                        marker: std::marker::PhantomData,
                    });
                }

                let slice = CAccountInfoSlice {
                    ptr: c_infos.as_ptr(),
                    len: c_infos.len(),
                    marker: std::marker::PhantomData,
                };

                (slice, c_infos)
            }

            pub unsafe fn to_vec_account_infos(
                slice: *mut CAccountInfoSlice<'a, 'b>,
            ) -> Vec<AccountInfo<'b>> {
                let mut result = unsafe { Vec::with_capacity((*slice).len) };

                unsafe {
                    for i in 0..(*slice).len {
                        let cai = &(*slice).ptr.add(i);

                        let lamports =
                            std::rc::Rc::new(std::cell::RefCell::new(&mut *(**cai).lamports));
                        let data = std::rc::Rc::new(std::cell::RefCell::new(
                            std::slice::from_raw_parts_mut((**cai).data, (**cai).data_len),
                        ));

                        result.push(AccountInfo {
                            key: &*((**cai).key as *const Pubkey),
                            lamports,
                            data,
                            owner: &*((**cai).owner as *const Pubkey),
                            rent_epoch: (**cai).rent_epoch,
                            is_signer: (**cai).is_signer,
                            is_writable: (**cai).is_writable,
                            executable: (**cai).executable,
                        });
                    }
                }

                result
            }
        }

        #[repr(C)]
        pub struct CReturnData {
            pub pubkey: Pubkey,
            pub data_ptr: *const u8,
            pub data_len: usize,
            pub cap: usize,
            pub has_data: bool, // whether return data exists
        }

        impl CReturnData {
            pub fn from(ret_data: Option<(Pubkey, Vec<u8>)>) -> CReturnData {
                if let Some((pubkey, v)) = ret_data {
                    let (data_ptr, data_len, cap) = (v.as_ptr(), v.len(), v.capacity());
                    std::mem::forget(v); // insure no double free, now CReturnData is the owner!
                    CReturnData {
                        pubkey,
                        data_ptr,
                        data_len,
                        cap,
                        has_data: true,
                    }
                } else {
                    CReturnData {
                        pubkey: Pubkey::new_from_array([0 as u8; 32]),
                        data_ptr: std::ptr::null(),
                        data_len: 0,
                        cap: 0,
                        has_data: false,
                    }
                }
            }

            pub fn to_ret_data(&self) -> Option<(Pubkey, Vec<u8>)> {
                if self.has_data == false {
                    None
                } else {
                    let pub_key = self.pubkey;
                    let vec = unsafe {
                        Vec::from_raw_parts(self.data_ptr as *mut _, self.data_len, self.cap)
                    };
                    Some((pub_key, vec))
                }
            }
        }

        #[repr(C)]
        pub struct OptionCInstructionOwned {
            pub program_id: Pubkey,
            pub accounts_ptr: *const AccountMeta,
            pub accounts_len: usize,
            pub accounts_cap: usize,
            pub data_ptr: *const u8,
            pub data_len: usize,
            pub data_cap: usize,
            pub is_some: bool,
        }

        impl<'a> From<Option<Instruction>> for OptionCInstructionOwned {
            fn from(instruction: Option<Instruction>) -> Self {
                if let Some(mut instr) = instruction {
                    // take accounts leaving an empty vec in its place.
                    let accounts = std::mem::take(&mut instr.accounts);
                    let (accounts_ptr, accounts_len, accounts_cap) =
                        (accounts.as_ptr(), accounts.len(), accounts.capacity());
                    // insure no double free, OptionCInstructionOwned is the owner now.
                    std::mem::forget(accounts);
                    let data = std::mem::take(&mut instr.data);
                    let (data_ptr, data_len, data_cap) =
                        (data.as_ptr(), data.len(), data.capacity());
                    std::mem::forget(data);
                    OptionCInstructionOwned {
                        program_id: instr.program_id,
                        accounts_ptr,
                        accounts_len,
                        accounts_cap,
                        data_ptr,
                        data_len,
                        data_cap,
                        is_some: true,
                    }
                } else {
                    OptionCInstructionOwned {
                        program_id: Pubkey::new_from_array([0 as u8; 32]),
                        accounts_ptr: std::ptr::null(),
                        accounts_cap: 0,
                        accounts_len: 0,
                        data_ptr: std::ptr::null(),
                        data_cap: 0,
                        data_len: 0,
                        is_some: false,
                    }
                }
            }
        }

        impl<'a> From<OptionCInstructionOwned> for Option<Instruction> {
            fn from(ocio: OptionCInstructionOwned) -> Self {
                if ocio.is_some == false {
                    None
                } else {
                    let accounts: Vec<AccountMeta> = unsafe {
                        Vec::from_raw_parts(
                            ocio.accounts_ptr as *mut _,
                            ocio.accounts_len,
                            ocio.accounts_cap,
                        )
                    };
                    let data: Vec<u8> = unsafe {
                        Vec::from_raw_parts(ocio.data_ptr as *mut _, ocio.data_len, ocio.data_cap)
                    };
                    Some(Instruction {
                        program_id: ocio.program_id,
                        accounts,
                        data,
                    })
                }
            }
        }

        #[repr(C)]
        pub struct SyscallStubsApi {
            pub sol_log: extern "C" fn(msg_ptr: *const u8, len: usize),
            pub sol_log_compute_units: extern "C" fn(),
            pub sol_remaining_compute_units: extern "C" fn() -> u64,
            pub sol_invoke_signed: extern "C" fn(
                instruction: CInstruction,
                account_infos: *mut CAccountInfoSlice,
                signers_seeds: CBytesArrayArray,
            ) -> i64,
            pub sol_get_clock_sysvar: extern "C" fn(var_addr: *mut u8) -> u64,
            pub sol_get_epoch_schedule_sysvar: extern "C" fn(var_addr: *mut u8) -> u64,
            pub sol_get_fees_sysvar: extern "C" fn(var_addr: *mut u8) -> u64,
            pub sol_get_rent_sysvar: extern "C" fn(var_addr: *mut u8) -> u64,
            pub sol_get_last_restart_slot: extern "C" fn(var_addr: *mut u8) -> u64,
            pub sol_memcpy: extern "C" fn(dst: *mut u8, src: *const u8, n: usize),
            pub sol_memmove: extern "C" fn(dst: *mut u8, src: *const u8, n: usize),
            pub sol_memcmp: extern "C" fn(s1: *const u8, s2: *const u8, n: usize, result: *mut i32),
            pub sol_memset: extern "C" fn(s: *mut u8, c: u8, n: usize),
            pub sol_get_return_data: extern "C" fn() -> CReturnData,
            pub sol_set_return_data: extern "C" fn(data_ptr: *const u8, len: usize),
            pub sol_log_data: extern "C" fn(data: CBytesArray),
            pub sol_get_processed_sibling_instruction:
                extern "C" fn(index: usize) -> OptionCInstructionOwned,
            pub sol_get_stack_height: extern "C" fn() -> u64,
            pub sol_get_epoch_rewards_sysvar: extern "C" fn(var_addr: *mut u8) -> u64,
        }
    };
}

#[macro_export]
macro_rules! common_stub_typesv2 {
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
        pub struct SyscallStubsApi2 {
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
