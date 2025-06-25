/// This macro exports a global alternative container to ProgramTest's SYSCALL_STUBS
#[macro_export]
macro_rules! declare_sol_loader_stubs {
    () => {
        $crate::common_stub_types!();

        pub use lazy_static;

        lazy_static::lazy_static! {
            pub static ref SYSCALL_STUBS: Arc<RwLock<Box<dyn SyscallStubs>>> = Arc::new(RwLock::new(Box::new(UnimplementedSyscallStubs {})));
        }

        pub struct UnimplementedSyscallStubs {}
        impl SyscallStubs for UnimplementedSyscallStubs {
            fn sol_get_clock_sysvar(&self, _var_addr: *mut u8) -> u64 {
                println!("Oooops! Unimplemented sol_get_clock_sysvar");
                unimplemented!()
            }
            fn sol_get_epoch_rewards_sysvar(&self, _var_addr: *mut u8) -> u64 {
                unimplemented!()
            }
            fn sol_get_epoch_schedule_sysvar(&self, _var_addr: *mut u8) -> u64 {
                unimplemented!()
            }
            fn sol_get_fees_sysvar(&self, _var_addr: *mut u8) -> u64 {
                unimplemented!()
            }
            fn sol_get_last_restart_slot(&self, _var_addr: *mut u8) -> u64 {
                unimplemented!()
            }
            fn sol_get_processed_sibling_instruction(&self, _index: usize) -> Option<Instruction> {
                unimplemented!()
            }
            fn sol_get_rent_sysvar(&self, _var_addr: *mut u8) -> u64 {
                unimplemented!()
            }
            fn sol_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)> {
                unimplemented!()
            }
            fn sol_get_stack_height(&self) -> u64 {
                unimplemented!()
            }
            fn sol_invoke_signed(
                &self,
                _instruction: &Instruction,
                _account_infos: &[AccountInfo],
                _signers_seeds: &[&[&[u8]]],
            ) -> ProgramResult {
                unimplemented!()
            }
            fn sol_log(&self, _message: &str) {
                unimplemented!()
            }
            fn sol_log_compute_units(&self) {
                unimplemented!()
            }
            fn sol_log_data(&self, _fields: &[&[u8]]) {
                unimplemented!()
            }
            unsafe fn sol_memcmp(&self, _s1: *const u8, _s2: *const u8, _n: usize, _result: *mut i32) {
                unimplemented!()
            }
            unsafe fn sol_memcpy(&self, _dst: *mut u8, _src: *const u8, _n: usize) {
                unimplemented!()
            }
            unsafe fn sol_memmove(&self, _dst: *mut u8, _src: *const u8, _n: usize) {
                unimplemented!()
            }
            unsafe fn sol_memset(&self, _s: *mut u8, _c: u8, _n: usize) {
                unimplemented!()
            }
            fn sol_remaining_compute_units(&self) -> u64 {
                unimplemented!()
            }
            fn sol_set_return_data(&self, _data: &[u8]) {
                unimplemented!()
            }
        }

        pub extern "C" fn sol_log(msg_ptr: *const u8, len: usize) {
            let message = unsafe { std::slice::from_raw_parts(msg_ptr, len) };
            let message = std::str::from_utf8(message).expect("Invalid UTF-8");
            SYSCALL_STUBS.read().unwrap().sol_log(message);
        }

        pub extern "C" fn sol_log_compute_units() {
            SYSCALL_STUBS.read().unwrap().sol_log_compute_units();
        }

        pub extern "C" fn sol_remaining_compute_units() -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_remaining_compute_units()
        }

        pub extern "C" fn sol_invoke_signed(
            cinstruction: CInstruction,
            caccount_infos: *mut CAccountInfoSlice,
            csigners_seeds: CBytesArrayArray,
        ) -> i64 {
            let instruction = Instruction::from(cinstruction);
            let signers_seeds = CBytesArrayArray::to_array_array_array(&csigners_seeds);
            let signers_seeds = &CBytesArrayArray::convert(&signers_seeds)[..];
            let account_infos = &CAccountInfoSlice::to_vec_account_infos(caccount_infos)[..];

            println!("instruction {:#?}", instruction);
            println!("signers: {:#?}", signers_seeds);
            println!("account_infos: {:#?}", account_infos);
            for ai in account_infos.iter() {
                println!("BEFORE ai: {} -> lamports: {}", ai.key, ai.lamports());
                println!("BEFORE ai: {} -> data.len: {}", ai.key, ai.data_len());
                println!("BEFORE ai: {} -> data.ptr: {:p}", ai.key, ai.data.as_ptr());
            }
            let data_ptrs: Vec<_> = account_infos.iter().map(|ai| ai.data.as_ptr()).collect();
            let res = SYSCALL_STUBS.read().unwrap().sol_invoke_signed(
                &instruction,
                account_infos,
                &signers_seeds,
            );
            let post_tx_data_ptrs: Vec<_> = account_infos.iter().map(|ai| ai.data.as_ptr()).collect();
            // If these mismatch we'll have to find out why and fix it.
            assert!(data_ptrs == post_tx_data_ptrs);

            for (i, ai) in account_infos.iter().enumerate() {
                println!("AFTER ai: {} -> lamports: {}", ai.key, ai.lamports());
                println!("AFTER ai: {} -> data.len: {}", ai.key, ai.data_len());
                println!("AFTER ai: {} -> data.ptr: {:p}", ai.key, ai.data.as_ptr());
                // Update the data len since after the transaction it might have changed.
                // We'll use this new len in the caller's code to have it update there as well.
                let cai: *mut CAccountInfo = unsafe { (*caccount_infos).ptr.add(i) } as *mut _;
                unsafe { (*cai).data_len = ai.data_len() };
            }

            let res = if res.is_ok() { 0 } else { -1 };
            println!("Result: {}", res);
            res
        }

        pub extern "C" fn sol_get_clock_sysvar(var_addr: *mut u8) -> u64 {
            println!("Calling sol_get_clock_sysvar from solana rust program");
            let res = SYSCALL_STUBS.read().unwrap().sol_get_clock_sysvar(var_addr);
            res
        }

        pub extern "C" fn sol_get_epoch_schedule_sysvar(var_addr: *mut u8) -> u64 {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_epoch_schedule_sysvar(var_addr)
        }

        pub extern "C" fn sol_get_fees_sysvar(var_addr: *mut u8) -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_get_fees_sysvar(var_addr)
        }

        pub extern "C" fn sol_get_rent_sysvar(var_addr: *mut u8) -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_get_rent_sysvar(var_addr)
        }

        pub extern "C" fn sol_get_last_restart_slot(var_addr: *mut u8) -> u64 {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_last_restart_slot(var_addr)
        }

        pub extern "C" fn sol_memcpy(dst: *mut u8, src: *const u8, n: usize) {
            unsafe {
                SYSCALL_STUBS.read().unwrap().sol_memcpy(dst, src, n);
            }
        }

        pub extern "C" fn sol_memmove(dst: *mut u8, src: *const u8, n: usize) {
            unsafe {
                SYSCALL_STUBS.read().unwrap().sol_memmove(dst, src, n);
            }
        }

        pub extern "C" fn sol_memcmp(s1: *const u8, s2: *const u8, n: usize, result: *mut i32) {
            unsafe {
                SYSCALL_STUBS.read().unwrap().sol_memcmp(s1, s2, n, result);
            }
        }

        pub extern "C" fn sol_memset(s: *mut u8, c: u8, n: usize) {
            unsafe {
                SYSCALL_STUBS.read().unwrap().sol_memset(s, c, n);
            }
        }

        // TODO
        pub extern "C" fn sol_get_return_data() -> Option<(Pubkey, Vec<u8>)> {
            SYSCALL_STUBS.read().unwrap().sol_get_return_data()
        }

        pub extern "C" fn sol_set_return_data(data_ptr: *const u8, len: usize) {
            let data = unsafe { std::slice::from_raw_parts(data_ptr, len) };
            SYSCALL_STUBS.read().unwrap().sol_set_return_data(data)
        }

        // TODO
        pub extern "C" fn sol_log_data(data: &[&[u8]]) {
            SYSCALL_STUBS.read().unwrap().sol_log_data(data)
        }

        // TODO
        pub extern "C" fn sol_get_processed_sibling_instruction(index: usize) -> Option<Instruction> {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_processed_sibling_instruction(index)
        }

        pub extern "C" fn sol_get_stack_height() -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_get_stack_height()
        }

        pub extern "C" fn sol_get_epoch_rewards_sysvar(var_addr: *mut u8) -> u64 {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_epoch_rewards_sysvar(var_addr)
        }

        impl SyscallStubsApi {
            pub fn new() -> Self {
                Self {
                    sol_get_clock_sysvar: sol_get_clock_sysvar,
                    sol_get_epoch_rewards_sysvar: sol_get_epoch_rewards_sysvar,
                    sol_get_epoch_schedule_sysvar: sol_get_epoch_schedule_sysvar,
                    sol_get_fees_sysvar: sol_get_fees_sysvar,
                    sol_get_last_restart_slot: sol_get_last_restart_slot,
                    sol_get_rent_sysvar: sol_get_rent_sysvar,
                    sol_get_stack_height: sol_get_stack_height,
                    sol_log: sol_log,
                    sol_log_compute_units: sol_log_compute_units,
                    sol_memcmp: sol_memcmp,
                    sol_memcpy: sol_memcpy,
                    sol_memmove: sol_memmove,
                    sol_memset: sol_memset,
                    sol_remaining_compute_units: sol_remaining_compute_units,
                    sol_set_return_data: sol_set_return_data,
                    sol_invoke_signed: sol_invoke_signed,
                }
            }
        }
    };
}
