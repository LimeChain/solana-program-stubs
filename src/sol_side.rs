#![allow(unexpected_cfgs)]

#[macro_export]
#[cfg(not(target_os = "solana"))]
macro_rules! declare_sol_app_stubsv2 {
    ($entry_function:expr) => {
        $crate::common_stub_typesv2!();

        #[repr(C)]
        pub struct SolAppSyscallStubs2 {
            pub stubs_api2: SyscallStubsApi2,
        }

        impl SyscallStubs for SolAppSyscallStubs2 {
            fn sol_get_clock_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api2.sol_get_clock_sysvar)(var_addr)
            }
            fn sol_get_epoch_rewards_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api2.sol_get_epoch_rewards_sysvar)(var_addr)
            }
            fn sol_get_epoch_schedule_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api2.sol_get_epoch_schedule_sysvar)(var_addr)
            }
            fn sol_get_epoch_stake(&self, vote_address: *const u8) -> u64 {
                (self.stubs_api2.sol_get_epoch_stake)(vote_address)
            }
            fn sol_get_fees_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api2.sol_get_fees_sysvar)(var_addr)
            }
            fn sol_get_last_restart_slot(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api2.sol_get_last_restart_slot)(var_addr)
            }
            fn sol_get_rent_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api2.sol_get_rent_sysvar)(var_addr)
            }
            fn sol_get_stack_height(&self) -> u64 {
                (self.stubs_api2.sol_get_stack_height)()
            }
            fn sol_remaining_compute_units(&self) -> u64 {
                (self.stubs_api2.sol_remaining_compute_units)()
            }
            unsafe fn sol_memcmp(&self, s1: *const u8, s2: *const u8, n: usize, result: *mut i32) {
                (self.stubs_api2.sol_memcmp_)(s1, s2, n as u64, result);
            }
            unsafe fn sol_memcpy(&self, dst: *mut u8, src: *const u8, n: usize) {
                (self.stubs_api2.sol_memcpy_)(dst, src, n as u64)
            }
            unsafe fn sol_memmove(&self, dst: *mut u8, src: *const u8, n: usize) {
                (self.stubs_api2.sol_memmove_)(dst, src, n as u64)
            }
            unsafe fn sol_memset(&self, s: *mut u8, c: u8, n: usize) {
                (self.stubs_api2.sol_memset_)(s, c, n as u64)
            }
            fn sol_get_sysvar(
                &self,
                sysvar_id_addr: *const u8,
                var_addr: *mut u8,
                offset: u64,
                length: u64,
            ) -> u64 {
                (self.stubs_api2.sol_get_sysvar)(sysvar_id_addr, var_addr, offset, length)
            }
            fn sol_log_compute_units(&self) {
                (self.stubs_api2.sol_log_compute_units_)()
            }
            fn sol_log(&self, message: &str) {
                (self.stubs_api2.sol_log_)(message.as_ptr(), message.len() as u64)
            }
            fn sol_log_data(&self, fields: &[&[u8]]) {
                (self.stubs_api2.sol_log_data)(fields.as_ptr() as *const u8, fields.len() as u64);
            }
            fn sol_set_return_data(&self, data: &[u8]) {
                (self.stubs_api2.sol_set_return_data)(data.as_ptr(), data.len() as u64);
            }
            fn sol_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)> {
                let mut program_id = CPubkey::from([0u8; 32]);
                let data_bytes_to_alloc =
                    (self.stubs_api2.sol_get_return_data)(&mut u8::default(), 0, &mut program_id);
                if data_bytes_to_alloc == 0 {
                    return None;
                }
                let mut vdata = vec![0u8; data_bytes_to_alloc as usize];
                let same_bytes_num_expected = (self.stubs_api2.sol_get_return_data)(
                    vdata.as_mut_ptr(),
                    vdata.len() as _,
                    &mut program_id,
                );
                if same_bytes_num_expected == data_bytes_to_alloc {
                    Some((Pubkey::new_from_array(*program_id.as_array()), vdata))
                } else {
                    None
                }
            }
            fn sol_get_processed_sibling_instruction(&self, index: usize) -> Option<Instruction> {
                let mut meta = CProcessedSiblingInstruction {
                    accounts_len: 0,
                    data_len: 0,
                };
                let mut program_id = CPubkey::from([0u8; 32]);
                if 1 == (self.stubs_api2.sol_get_processed_sibling_instruction)(
                    index as _,
                    &mut meta,
                    &mut program_id,
                    &mut u8::default(),
                    &mut CAccountMeta::default(),
                ) {
                    let accounts_to_alloc = meta.accounts_len;
                    let data_bytes_to_alloc = meta.data_len;
                    let mut caccount_metas = vec![CAccountMeta::default(); accounts_to_alloc as _];
                    let mut vdata = vec![0u8; data_bytes_to_alloc as _];
                    let res = (self.stubs_api2.sol_get_processed_sibling_instruction)(
                        index as _,
                        &mut meta,
                        &mut program_id,
                        vdata.as_mut_ptr(),
                        caccount_metas.as_mut_ptr(),
                    );
                    if res != 0 && res != 1 {
                        let mut account_metas = vec![];
                        for cai in &caccount_metas {
                            // let pubkey = unsafe { *Box::from_raw(cai.pubkey as *mut _) };
                            let pubkey = unsafe { *(*cai.pubkey).as_array() };
                            let account_meta = AccountMeta {
                                is_signer: cai.is_signer,
                                is_writable: cai.is_writable,
                                pubkey: Pubkey::new_from_array(pubkey),
                            };
                            account_metas.push(account_meta);
                        }
                        return Some(Instruction {
                            accounts: account_metas,
                            data: vdata,
                            program_id: Pubkey::new_from_array(*program_id.as_array()),
                        });
                    }
                }
                None
            }
            fn sol_invoke_signed(
                &self,
                instruction: &Instruction,
                account_infos: &[AccountInfo],
                signers_seeds: &[&[&[u8]]],
            ) -> ProgramResult {
                let mut caccounts = vec![];
                for account_meta in &instruction.accounts {
                    let caccount = CAccountMeta {
                        is_signer: account_meta.is_signer,
                        is_writable: account_meta.is_writable,
                        pubkey: &account_meta.pubkey as *const _ as *const CPubkey,
                    };
                    caccounts.push(caccount);
                }
                let cinstr = CInstruction {
                    program_id: &instruction.program_id as *const _ as *const CPubkey,
                    accounts_len: instruction.accounts.len() as _,
                    data_len: instruction.data.len() as _,
                    accounts: caccounts.as_ptr(),
                    data: instruction.data.as_ptr(),
                };
                let mut caccount_infos = vec![];
                for account_info in account_infos {
                    let lamports_ref = &mut *account_info.lamports.borrow_mut();
                    let data_ref = &mut *account_info.data.borrow_mut();
                    let caccount_info = CAccountInfo {
                        is_signer: account_info.is_signer,
                        is_writable: account_info.is_writable,
                        executable: account_info.executable,
                        rent_epoch: account_info.rent_epoch,
                        data_len: data_ref.len() as _,
                        data: data_ref.as_mut_ptr(),
                        lamports: *lamports_ref as *mut u64,
                        key: account_info.key as *const _ as *const CPubkey,
                        owner: account_info.owner as *const _ as *const CPubkey,
                    };
                    let cai = &caccount_info;
                    let key: &Pubkey = unsafe { &*((*cai).key as *const Pubkey) };
                    let owner: &Pubkey = unsafe { &*((*cai).owner as *const Pubkey) };
                    caccount_infos.push(caccount_info);
                }

                let res = (self.stubs_api2.sol_invoke_signed_c)(
                    &cinstr as *const _ as *const u8,
                    caccount_infos.as_ptr() as *const u8,
                    caccount_infos.len() as _,
                    signers_seeds.as_ptr() as *const u8,
                    signers_seeds.len() as _,
                );
                if res == 0 {
                    for (i, ai) in account_infos.iter().enumerate() {
                        let new_data_slice = unsafe {
                            std::slice::from_raw_parts_mut(
                                (*ai.data.borrow_mut()).as_mut_ptr(),
                                caccount_infos[i].data_len as _,
                            )
                        };
                        *ai.data.borrow_mut() = new_data_slice;
                        assert!(ai.lamports() == unsafe { *caccount_infos[i].lamports });
                    }
                    Ok(())
                } else {
                    Err(ProgramError::Custom(res as _))
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn set_stubs(stubs_api2: SyscallStubsApi2) {
            let stubs = Box::new(SolAppSyscallStubs2 { stubs_api2 });
            let _ = set_syscall_stubs(stubs);
        }
    };
}
