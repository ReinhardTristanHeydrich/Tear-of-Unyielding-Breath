use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrManDbg, WorldChrManDbgFlags},
    fd4::FD4TaskData,
};
use eldenring_util::{
    program::Program,
    singleton::get_instance,
    system::wait_for_system_init,
    task::CSTaskImpExt,
};

#[link(name = "kernel32")]
unsafe extern "C" {
    unsafe fn DisableThreadLibraryCalls(hmodule: usize) -> bool;
}

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(hmodule: usize, reason: u32) -> bool {
    const GAME_INJECTION: u32 = 1;
    if reason != GAME_INJECTION {
        return true;
    }
    DisableThreadLibraryCalls(hmodule);

    std::thread::spawn(|| {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");

        // Pega a instância do sistema de tasks
        let task_system = get_instance::<CSTaskImp>().unwrap().unwrap();

        // Hook que roda uma vez por frame para manter a flag ativada
        task_system.run_recurring(
            |_task_data: &FD4TaskData| {
                // Tenta pegar a instância do WorldChrManDbg
                let Some(world_chr_man_dbg) = get_instance::<WorldChrManDbg>()
                    .expect("No reflection data for WorldChrManDbg")
                else {
                    return;
                };

                // O WorldChrManDbgFlags normalmente fica logo após o WorldChrManDbg na memória
                // Calculamos o offset para as flags
                let flags_ptr = (world_chr_man_dbg as *const WorldChrManDbg as usize + 
                                std::mem::size_of::<WorldChrManDbg>()) as *mut WorldChrManDbgFlags;
                
                // Verifica se o ponteiro é válido antes de acessar
                if !flags_ptr.is_null() {
                    let flags = &mut *flags_ptr;
                    
                    // Ativa a flag de não consumir stamina
                    flags.no_stamina_consume = true;
                    
                    // Opcionalmente, você pode ativar outras flags úteis:
                    // flags.no_fp_consume = true;      // Não consome FP
                    // flags.no_goods_consume = true;   // Não consome itens
                    // flags.no_dead = true;            // Não pode morrer
                }
            },
            CSTaskGroupIndex::FrameBegin,
        );
    });

    true
}