#[macro_export]
macro_rules! declare_sol_app_stubs {
    ($entry_function:expr) => {
        use solana_program::instruction::AccountMeta;
        use solana_program::instruction::Instruction;

        $crate::common_stub_types!();

        #[repr(C)]
        pub struct SolAppSyscallStubs {
            pub stubs_api: SyscallStubsApi,
        }

        #[cfg(not(target_os = "solana"))]
        impl solana_program::program_stubs::SyscallStubs for SolAppSyscallStubs {
            fn sol_get_clock_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api.sol_get_clock_sysvar)(var_addr)
            }
            fn sol_get_epoch_rewards_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api.sol_get_epoch_rewards_sysvar)(var_addr)
            }
            fn sol_get_epoch_schedule_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api.sol_get_epoch_schedule_sysvar)(var_addr)
            }
            fn sol_get_fees_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api.sol_get_fees_sysvar)(var_addr)
            }
            fn sol_get_last_restart_slot(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api.sol_get_last_restart_slot)(var_addr)
            }
            fn sol_get_processed_sibling_instruction(
                &self,
                _index: usize,
            ) -> Option<solana_program::instruction::Instruction> {
                println!("sol_get_processed_sibling_instruction called!");
                unimplemented!()
            }
            fn sol_get_rent_sysvar(&self, var_addr: *mut u8) -> u64 {
                (self.stubs_api.sol_get_rent_sysvar)(var_addr)
            }
            fn sol_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)> {
                println!("sol_get_return_data called!");
                unimplemented!()
            }
            fn sol_get_stack_height(&self) -> u64 {
                (self.stubs_api.sol_get_stack_height)()
            }
            fn sol_invoke_signed(
                &self,
                instruction: &solana_program::instruction::Instruction,
                account_infos: &[AccountInfo],
                signers_seeds: &[&[&[u8]]],
            ) -> solana_program::entrypoint::ProgramResult {
                println!("sol_invoke_signed called!");
                println!("FAV: instruction {:#?}", instruction);
                println!("FAV: signers: {:#?}", signers_seeds);
                println!("FAV: account_infos: {:#?}", account_infos);
                // // TEST
                // for ai in account_infos.iter() {
                //     Box::leak(Box::new(Rc::clone(&ai.lamports)));
                //     Box::leak(Box::new(Rc::clone(&ai.data)));
                // }
                // // TEST
                let cinstr = CInstruction::from(&instruction);
                let caccountinfos =
                    &mut CAccountInfoSlice::to_c_account_info_slice(&account_infos) as *mut _;
                let cbytesarrayarray = CBytesArrayArray::from(&signers_seeds);
                for ai in account_infos.iter() {
                    println!("FAV BEFORE ai: {} -> lamports: {}", ai.key, ai.lamports());
                    println!("FAV BEFORE ai: {} -> data.len: {}", ai.key, ai.data_len());
                    println!(
                        "FAV BEFORE ai: {} -> data.ptr: {:p}",
                        ai.key,
                        ai.data.as_ptr()
                    );
                }
                (self.stubs_api.sol_invoke_signed)(cinstr, caccountinfos, cbytesarrayarray);
                for (i, ai) in account_infos.iter().enumerate() {
                    println!("FAV AFTER ai: {} -> lamports: {}", ai.key, ai.lamports());
                    println!("FAV AFTER ai: {} -> data.len: {}", ai.key, ai.data_len());
                    println!(
                        "FAV AFTER ai: {} -> data.ptr: {:p}",
                        ai.key,
                        ai.data.as_ptr()
                    );

                    // After the transaction the data might have changed, so update it.
                    // We expect that the remote has updated it accordingly.
                    let cai: *mut CAccountInfo = unsafe { (*caccountinfos).ptr.add(i) } as *mut _;
                    let data_ptr = (*ai.data.borrow_mut()).as_mut_ptr();
                    let data_len = unsafe { (*cai).data_len };
                    let new_slice = unsafe { std::slice::from_raw_parts_mut(data_ptr, data_len) };
                    println!(
                        "Data's len has changed to new_slice.len(): {}",
                        new_slice.len()
                    );
                    (*ai.data.borrow_mut()) = new_slice;
                }
                println!("DONE!");
                Ok(())
            }
            fn sol_log(&self, message: &str) {
                let len = message.len();
                let msg_ptr = message.as_ptr();
                (self.stubs_api.sol_log)(msg_ptr, len);
            }
            fn sol_log_compute_units(&self) {
                (self.stubs_api.sol_log_compute_units)();
            }
            fn sol_log_data(&self, _fields: &[&[u8]]) {
                println!("sol_log_data called!");
                unimplemented!()
            }
            unsafe fn sol_memcmp(&self, s1: *const u8, s2: *const u8, n: usize, result: *mut i32) {
                (self.stubs_api.sol_memcmp)(s1, s2, n, result);
            }
            unsafe fn sol_memcpy(&self, dst: *mut u8, src: *const u8, n: usize) {
                (self.stubs_api.sol_memcpy)(dst, src, n);
            }
            unsafe fn sol_memmove(&self, dst: *mut u8, src: *const u8, n: usize) {
                (self.stubs_api.sol_memmove)(dst, src, n);
            }
            unsafe fn sol_memset(&self, s: *mut u8, c: u8, n: usize) {
                (self.stubs_api.sol_memset)(s, c, n);
            }
            fn sol_remaining_compute_units(&self) -> u64 {
                (self.stubs_api.sol_remaining_compute_units)()
            }
            fn sol_set_return_data(&self, data: &[u8]) {
                let len = data.len();
                let data_ptr = data.as_ptr();
                (self.stubs_api.sol_set_return_data)(data_ptr, len);
            }
        }

        #[cfg(not(target_os = "solana"))]
        #[no_mangle]
        pub extern "C" fn set_stubs(stubs_api: SyscallStubsApi) {
            println!("Calling set_stubs!");
            let stubs = Box::new(SolAppSyscallStubs { stubs_api });
            let _ = solana_program::program_stubs::set_syscall_stubs(stubs);
            // let _ = solana_program::program_stubs::set_syscall_stubs(Box::new(
            //     solana_program_test::SyscallStubs {},
            // ));
        }

        #[no_mangle]
        pub unsafe extern "C" fn get_rust_entrypoint() -> *const () {
            println!("We're here!");
            $entry_function as *const ()
        }
    };
}
