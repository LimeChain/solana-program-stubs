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
            let account_infos = unsafe { &CAccountInfoSlice::to_vec_account_infos(caccount_infos)[..] };

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
                // Update the data len and the lamports since after the transaction they might have changed.
                // We'll use this new len in the caller's code to have it update there as well.
                let cai: *mut CAccountInfo = unsafe { (*caccount_infos).ptr.add(i) } as *mut _;
                unsafe {
                    *(*cai).lamports = ai.lamports();
                    (*cai).data_len = ai.data_len();
                };
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

        pub extern "C" fn sol_get_return_data() -> CReturnData {
            let ret_data = SYSCALL_STUBS.read().unwrap().sol_get_return_data();
            let c_ret_data = CReturnData::from(ret_data);
            c_ret_data
        }

        pub extern "C" fn sol_set_return_data(data_ptr: *const u8, len: usize) {
            let data = unsafe { std::slice::from_raw_parts(data_ptr, len) };
            SYSCALL_STUBS.read().unwrap().sol_set_return_data(data)
        }

        pub extern "C" fn sol_log_data(data: CBytesArray) {
            let arr_arr = CBytesArray::to_array_array(&data);
            SYSCALL_STUBS.read().unwrap().sol_log_data(&arr_arr[..])
        }

        pub extern "C" fn sol_get_processed_sibling_instruction(index: usize) -> OptionCInstructionOwned {
            println!("in EXTERN C addr of SYSCALL_STUBS: {:p}", &SYSCALL_STUBS);
            let opt_instr = SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_processed_sibling_instruction(index);
            println!("sol_get_processed_sibling_instruction() in loader: {:?}", opt_instr);
            let c_opt_instr_owned = OptionCInstructionOwned::from(opt_instr);
            c_opt_instr_owned
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
                    sol_get_return_data: sol_get_return_data,
                    sol_set_return_data: sol_set_return_data,
                    sol_log_data: sol_log_data,
                    sol_invoke_signed: sol_invoke_signed,
                    sol_get_processed_sibling_instruction: sol_get_processed_sibling_instruction,
                }
            }
        }
    };
}

/// This macro exports a global alternative container v2 to ProgramTest's SYSCALL_STUBS
#[macro_export]
macro_rules! declare_sol_loader_stubsv2 {
    () => {
        $crate::common_stub_typesv2!();

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

        #[no_mangle]
        pub extern "C" fn sol_log_(msg: *const u8, len: u64) {
            let message = unsafe { std::slice::from_raw_parts(msg, len as _) };
            let m = String::from_utf8_lossy(message);
            SYSCALL_STUBS.read().unwrap().sol_log(&m);
        }

        #[no_mangle]
        pub extern "C" fn sol_log_compute_units_() {
            SYSCALL_STUBS.read().unwrap().sol_log_compute_units();
        }

        #[no_mangle]
        pub extern "C" fn sol_remaining_compute_units() -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_remaining_compute_units()
        }

        #[no_mangle]
        pub extern "C" fn sol_memcpy_(dst: *mut u8, src: *const u8, n: u64) {
            unsafe {
                SYSCALL_STUBS.read().unwrap().sol_memcpy(dst, src, n as _);
            }
        }

        #[no_mangle]
        pub extern "C" fn sol_memmove_(dst: *mut u8, src: *const u8, n: u64) {
            unsafe {
                SYSCALL_STUBS.read().unwrap().sol_memmove(dst, src, n as _);
            }
        }

        #[no_mangle]
        pub extern "C" fn sol_memcmp_(s1: *const u8, s2: *const u8, n: u64, result: *mut i32) {
            unsafe {
                SYSCALL_STUBS
                    .read()
                    .unwrap()
                    .sol_memcmp(s1, s2, n as _, result);
            }
        }

        #[no_mangle]
        pub extern "C" fn sol_memset_(s: *mut u8, c: u8, n: u64) {
            unsafe {
                SYSCALL_STUBS.read().unwrap().sol_memset(s, c, n as _);
            }
        }

        #[no_mangle]
        pub extern "C" fn sol_get_stack_height() -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_get_stack_height()
        }

        #[no_mangle]
        pub extern "C" fn sol_get_clock_sysvar(addr: *mut u8) -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_get_clock_sysvar(addr)
        }

        #[no_mangle]
        pub extern "C" fn sol_get_epoch_schedule_sysvar(addr: *mut u8) -> u64 {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_epoch_schedule_sysvar(addr)
        }

        #[no_mangle]
        pub extern "C" fn sol_get_fees_sysvar(addr: *mut u8) -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_get_fees_sysvar(addr)
        }

        #[no_mangle]
        pub extern "C" fn sol_get_rent_sysvar(addr: *mut u8) -> u64 {
            SYSCALL_STUBS.read().unwrap().sol_get_rent_sysvar(addr)
        }

        #[no_mangle]
        pub extern "C" fn sol_get_epoch_rewards_sysvar(addr: *mut u8) -> u64 {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_epoch_rewards_sysvar(addr)
        }

        #[no_mangle]
        pub extern "C" fn sol_get_last_restart_slot(addr: *mut u8) -> u64 {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_last_restart_slot(addr)
        }

        #[no_mangle]
        pub extern "C" fn sol_get_epoch_stake(vote_address: *const u8) -> u64 {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_epoch_stake(vote_address)
        }

        #[no_mangle]
        pub extern "C" fn sol_get_sysvar(
            sysvar_id_addr: *const u8,
            result: *mut u8,
            offset: u64,
            length: u64,
        ) -> u64 {
            SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_sysvar(sysvar_id_addr, result, offset, length)
        }

        #[no_mangle]
        pub extern "C" fn sol_set_return_data(data: *const u8, length: u64) {
            let slice = unsafe { std::slice::from_raw_parts(data, length as _) };
            SYSCALL_STUBS.read().unwrap().sol_set_return_data(slice);
        }

        #[no_mangle]
        pub extern "C" fn sol_get_return_data(data: *mut u8, length: u64, program_id: *mut CPubkey) -> u64 {
            let ret_data = SYSCALL_STUBS.read().unwrap().sol_get_return_data();

            match ret_data {
                None => 0,
                Some((key, src)) => {
                    // Caller is wondering how many to allocate.
                    if length == 0 {
                        unsafe { *program_id = key.as_array().into() };
                        return src.len() as _;
                    }

                    // Caller is ready with the allocation - we're expected to copy the data.
                    // Let's check if there's enough space.
                    let src_len = src.len() as _;
                    if src_len > length || unsafe { *(*program_id).as_array() } != *key.as_array() {
                        return 0;
                    }
                    unsafe {
                        std::ptr::copy_nonoverlapping(src.as_ptr(), data, length as _);
                    };
                    src_len
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn sol_log_data(data: *const u8, data_len: u64) {
            // reinterpret the buffer as a fat pointer to (*const u8, usize) pairs
            let fat_ptrs = data as *const (*const u8, u64);
            let mut v: Vec<&[u8]> = Vec::with_capacity(data_len as _);
            for i in 0..data_len {
                let (data_ptr, len) = unsafe { *fat_ptrs.add(i as _) };
                let slice = unsafe { std::slice::from_raw_parts(data_ptr, len as _) };
                v.push(slice);
            }
            SYSCALL_STUBS.read().unwrap().sol_log_data(&v[..]);
        }

        #[no_mangle]
        pub extern "C" fn sol_get_processed_sibling_instruction(
            index: u64,
            meta: *mut CProcessedSiblingInstruction,
            program_id: *mut CPubkey,
            data: *mut u8,
            accounts: *mut CAccountMeta,
        ) -> u64 {
            let instruction = SYSCALL_STUBS
                .read()
                .unwrap()
                .sol_get_processed_sibling_instruction(index as _);
            match instruction {
                None => 0, // 0 - No processed sibling instruction.
                Some(instr) => {
                    let data_len = instr.data.len();
                    let accounts_len = instr.accounts.len();
                    unsafe {
                        if (*meta).accounts_len == 0 && (*meta).data_len == 0 {
                            // Caller is wondering how many to allocate.
                            // https://github.com/anza-xyz/solana-sdk/blob/master/instruction/src/syscalls.rs#L32
                            (*meta).data_len = data_len as _;
                            (*meta).accounts_len = accounts_len as _;
                            *program_id = instr.program_id.as_array().into();

                            // 1 - Return the allocation details so that caller can prepare.
                            return 1;
                        }
                    }

                    // Caller is ready with the allocation.
                    // But first - a little sanity check.
                    unsafe {
                        if (*meta).data_len != data_len as u64
                            || (*meta).accounts_len != accounts_len as u64
                            || *(*program_id).as_array() != *instr.program_id.as_array()
                        {
                            return 0;
                        }

                        // Now just copy the data and the account metas.
                        std::ptr::copy_nonoverlapping(instr.data.as_ptr(), data, data_len);
                        // Now copy the account metas taking into consideration that pubkey is a *const u8.
                        // https://github.com/anza-xyz/pinocchio/blob/main/sdk/pinocchio/src/instruction.rs#L116
                        for i in 0..instr.accounts.len() {
                            let account_meta = accounts.add(i);
                            (*account_meta).is_signer = instr.accounts[i].is_signer;
                            (*account_meta).is_writable = instr.accounts[i].is_writable;
                            (*account_meta).pubkey =
                                Box::leak(Box::new(instr.accounts[i].pubkey)) as *const _ as *const CPubkey;
                        }
                    }
                    2 // 2 - All good.
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn sol_invoke_signed_c(
            instruction_addr: *const u8,
            account_infos_addr: *const u8,
            account_infos_len: u64,
            signers_seeds_addr: *const u8,
            signers_seeds_len: u64,
        ) -> u64 {
            // instruction
            let cinstr = instruction_addr as *const CInstruction;
            let instruction = unsafe {
                Instruction {
                    program_id: Pubkey::new_from_array(*(*(*cinstr).program_id).as_array()),
                    accounts: {
                        (0..(*cinstr).accounts_len)
                            .map(|i| {
                                let cam = (*cinstr).accounts.add(i as _);
                                AccountMeta {
                                    pubkey: Pubkey::new_from_array(*(*(*cam).pubkey).as_array()),
                                    is_signer: (*cam).is_signer,
                                    is_writable: (*cam).is_writable,
                                }
                            })
                            .collect()
                    },
                    data: {
                        let slice = std::slice::from_raw_parts((*cinstr).data, (*cinstr).data_len as _);
                        slice.to_vec()
                    },
                }
            };

            // account_infos
            let ai_ptr = account_infos_addr as *const CAccountInfo;
            let mut account_infos: Vec<AccountInfo<'_>> = vec![];
            for i in 0..account_infos_len {
                let cai = unsafe { &*ai_ptr.add(i as _) };
                let ai = unsafe {
                    AccountInfo {
                        key: &*((*cai).key as *const Pubkey),
                        lamports: std::rc::Rc::new(std::cell::RefCell::new(
                            &mut *((*cai).lamports as *mut _),
                        )),
                        data: {
                            let slice =
                                std::slice::from_raw_parts_mut((*cai).data as _, (*cai).data_len as _);
                            std::rc::Rc::new(std::cell::RefCell::new(slice))
                        },
                        owner: &*((*cai).owner as *const Pubkey),
                        rent_epoch: (*cai).rent_epoch,
                        is_signer: (*cai).is_signer,
                        is_writable: (*cai).is_writable,
                        executable: (*cai).executable,
                    }
                };
                account_infos.push(ai);
            }

            // signers_seeds
            let q_fat_ptr = signers_seeds_addr as *const (*const u8, u64);
            let mut qv: Vec<Vec<&[u8]>> = vec![];
            for q in 0..signers_seeds_len {
                let (q_data_ptr, q_data_len) = unsafe { *q_fat_ptr.add(q as _) };
                let mut pv: Vec<&[u8]> = vec![];
                for p in 0..q_data_len {
                    let p_fat_ptr = q_data_ptr as *const (*const u8, u64);
                    let (p_data_ptr, p_data_len) = unsafe { *p_fat_ptr.add(p as _) };
                    let slice = unsafe { std::slice::from_raw_parts(p_data_ptr, p_data_len as usize) };
                    pv.push(slice);
                }
                qv.push(pv);
            }

            let signers_seeds: Vec<_> = qv.iter().map(|e| &e[..]).collect();
            match SYSCALL_STUBS.read().unwrap().sol_invoke_signed(
                &instruction,
                &account_infos[..],
                &signers_seeds[..],
            ) {
                Ok(_) => {
                    let ai_ptr = account_infos_addr as *mut CAccountInfo;
                    for (i, acc) in account_infos.iter().enumerate() {
                        // Update data lens so that caller has them.
                        let cai = unsafe { &mut *ai_ptr.add(i as _) };
                        cai.data_len = acc.data_len() as _;
                    }
                    0
                }
                Err(e) => e.into(),
            }
        }

        #[no_mangle]
        pub extern "C" fn sol_log_pubkey(pubkey: *const u8) {
            let pubkey = unsafe { &*(pubkey as *const Pubkey) };
            SYSCALL_STUBS.read().unwrap().sol_log(&pubkey.to_string());
        }

        impl SyscallStubsApi2 {
            pub fn new() -> Self {
                Self {
                    sol_get_clock_sysvar: sol_get_clock_sysvar,
                    sol_get_epoch_rewards_sysvar: sol_get_epoch_rewards_sysvar,
                    sol_get_epoch_schedule_sysvar: sol_get_epoch_schedule_sysvar,
                    sol_get_fees_sysvar: sol_get_fees_sysvar,
                    sol_get_last_restart_slot: sol_get_last_restart_slot,
                    sol_get_rent_sysvar: sol_get_rent_sysvar,
                    sol_get_stack_height: sol_get_stack_height,
                    sol_log_: sol_log_,
                    sol_log_compute_units_: sol_log_compute_units_,
                    sol_memcmp_: sol_memcmp_,
                    sol_memcpy_: sol_memcpy_,
                    sol_memmove_: sol_memmove_,
                    sol_memset_: sol_memset_,
                    sol_remaining_compute_units: sol_remaining_compute_units,
                    sol_get_return_data: sol_get_return_data,
                    sol_set_return_data: sol_set_return_data,
                    sol_log_data: sol_log_data,
                    sol_invoke_signed_c: sol_invoke_signed_c,
                    sol_get_processed_sibling_instruction: sol_get_processed_sibling_instruction,
                    sol_get_epoch_stake: sol_get_epoch_stake,
                    sol_get_sysvar: sol_get_sysvar,
                }
            }
        }
    };
}
