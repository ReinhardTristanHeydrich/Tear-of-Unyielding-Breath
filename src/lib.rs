//==============================================================================================================
//Compile Time
use std::time::Duration;
use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
};
use eldenring_util::{
    program::Program,
    singleton::get_instance,
    system::wait_for_system_init,
    task::CSTaskImpExt,
};
use winapi::um::libloaderapi::DisableThreadLibraryCalls as DisableThreadCalls;

const GAME_INJECTION: u32 = 1;

//==============================================================================================================

#[unsafe(no_mangle)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe extern "C" fn DllMain(hmodule: usize, reason: u32) -> bool {
    if reason != GAME_INJECTION {return true;}

    DisableThreadCalls(hmodule as *mut _);

    std::thread::spawn(|| {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");

        let task_system = get_instance::<CSTaskImp>().unwrap().unwrap();
        //==============================================================
        for task_type in ALL_TASKS {
        task_system.run_recurring(
            |_task_data: &FD4TaskData| {
                let Some(main_player) = get_instance::<WorldChrMan>()
                    .expect("No reflection data for WorldChrMan")
                    .and_then(|wcm| wcm.main_player.as_mut())
                else {return;};

                main_player.player_game_data.current_stamina = main_player.player_game_data.current_max_stamina;
            },
            *task_type,
        );
        }
        //==============================================================
    });

    true
}

//==============================================================================================================
//Compile Time but only the long things
const ALL_TASKS: &[CSTaskGroupIndex] = &[
    CSTaskGroupIndex::FrameBegin,
    CSTaskGroupIndex::SteamThread0,
    CSTaskGroupIndex::SteamThread1,
    CSTaskGroupIndex::SteamThread2,
    CSTaskGroupIndex::SteamThread3,
    CSTaskGroupIndex::SteamThread4,
    CSTaskGroupIndex::SteamThread5,
    CSTaskGroupIndex::SystemStep,
    CSTaskGroupIndex::ResStep,
    CSTaskGroupIndex::PadStep,
    CSTaskGroupIndex::GameFlowStep,
    CSTaskGroupIndex::EndShiftWorldPosition,
    CSTaskGroupIndex::GameMan,
    CSTaskGroupIndex::TaskLineIdx_Sys,
    CSTaskGroupIndex::TaskLineIdx_Test,
    CSTaskGroupIndex::TaskLineIdx_NetworkFlowStep,
    CSTaskGroupIndex::TaskLineIdx_InGame_InGameStep,
    CSTaskGroupIndex::TaskLineIdx_InGame_InGameStayStep,
    CSTaskGroupIndex::MovieStep,
    CSTaskGroupIndex::RemoStep,
    CSTaskGroupIndex::TaskLineIdx_InGame_MoveMapStep,
    CSTaskGroupIndex::FieldArea_EndWorldAiManager,
    CSTaskGroupIndex::EmkSystem_Pre,
    CSTaskGroupIndex::EmkSystem_ConditionStatus,
    CSTaskGroupIndex::EmkSystem_Post,
    CSTaskGroupIndex::EventMan,
    CSTaskGroupIndex::FlverResDelayDelectiionBegin,
    CSTaskGroupIndex::TaskLineIdx_InGame_FieldAreaStep,
    CSTaskGroupIndex::TaskLineIdx_InGame_TestNetStep,
    CSTaskGroupIndex::TaskLineIdx_InGame_InGameMenuStep,
    CSTaskGroupIndex::TaskLineIdx_InGame_TitleMenuStep,
    CSTaskGroupIndex::TaskLineIdx_InGame_CommonMenuStep,
    CSTaskGroupIndex::TaskLineIdx_FrpgNet_Sys,
    CSTaskGroupIndex::TaskLineIdx_FrpgNet_Lobby,
    CSTaskGroupIndex::TaskLineIdx_FrpgNet_ConnectMan,
    CSTaskGroupIndex::TaskLineIdx_FrpgNet_Connect,
    CSTaskGroupIndex::TaskLineIdx_FrpgNet_Other,
    CSTaskGroupIndex::SfxMan,
    CSTaskGroupIndex::FaceGenMan,
    CSTaskGroupIndex::FrpgNetMan,
    CSTaskGroupIndex::NetworkUserManager,
    CSTaskGroupIndex::SessionManager,
    CSTaskGroupIndex::BlockList,
    CSTaskGroupIndex::LuaConsoleServer,
    CSTaskGroupIndex::RmiMan,
    CSTaskGroupIndex::ResMan,
    CSTaskGroupIndex::SfxDebugger,
    CSTaskGroupIndex::REMOTEMAN,
    CSTaskGroupIndex::Geom_WaitActivateFade,
    CSTaskGroupIndex::Geom_UpdateDraw,
    CSTaskGroupIndex::Grass_BatchUpdate,
    CSTaskGroupIndex::Grass_ResourceLoadKick,
    CSTaskGroupIndex::Grass_ResourceLoad,
    CSTaskGroupIndex::Grass_ResourceCleanup,
    CSTaskGroupIndex::WorldChrMan_Respawn,
    CSTaskGroupIndex::WorldChrMan_Prepare,
    CSTaskGroupIndex::ChrIns_CalcUpdateInfo_PerfBegin,
    CSTaskGroupIndex::ChrIns_CalcUpdateInfo,
    CSTaskGroupIndex::ChrIns_CalcUpdateInfo_PerfEnd,
    CSTaskGroupIndex::WorldChrMan_PrePhysics,
    CSTaskGroupIndex::WorldChrMan_CalcOmissionLevel_Begin,
    CSTaskGroupIndex::WorldChrMan_CalcOmissionLevel,
    CSTaskGroupIndex::WorldChrMan_CalcOmissionLevel_End,
    CSTaskGroupIndex::WorldChrMan_ConstructUpdateList,
    CSTaskGroupIndex::WorldChrMan_ChrNetwork,
    CSTaskGroupIndex::ChrIns_Prepare,
    CSTaskGroupIndex::ChrIns_NaviCache,
    CSTaskGroupIndex::ChrIns_AILogic_PerfBegin,
    CSTaskGroupIndex::ChrIns_AILogic,
    CSTaskGroupIndex::ChrIns_AILogic_PerfEnd,
    CSTaskGroupIndex::AI_SimulationStep,
    CSTaskGroupIndex::ChrIns_PreBehavior,
    CSTaskGroupIndex::ChrIns_PreBehaviorSafe,
    CSTaskGroupIndex::GeomModelInsCreatePartway_Begin,
    CSTaskGroupIndex::HavokBehavior,
    CSTaskGroupIndex::GeomModelInsCreatePartway_End,
    CSTaskGroupIndex::ChrIns_BehaviorSafe,
    CSTaskGroupIndex::ChrIns_PrePhysics_Begin,
    CSTaskGroupIndex::ChrIns_PrePhysics,
    CSTaskGroupIndex::ChrIns_PrePhysics_End,
    CSTaskGroupIndex::NetFlushSendData,
    CSTaskGroupIndex::ChrIns_PrePhysicsSafe,
    CSTaskGroupIndex::ChrIns_RagdollSafe,
    CSTaskGroupIndex::ChrIns_GarbageCollection,
    CSTaskGroupIndex::GeomModelInsCreate,
    CSTaskGroupIndex::AiBeginCollectGabage,
    CSTaskGroupIndex::WorldChrMan_Update_RideCheck,
    CSTaskGroupIndex::InGameDebugViewer,
    CSTaskGroupIndex::LocationStep,
    CSTaskGroupIndex::LocationUpdate_PrePhysics,
    CSTaskGroupIndex::LocationUpdate_PrePhysics_Parallel,
    CSTaskGroupIndex::LocationUpdate_PrePhysics_Post,
    CSTaskGroupIndex::LocationUpdate_PostCloth,
    CSTaskGroupIndex::LocationUpdate_PostCloth_Parallel,
    CSTaskGroupIndex::LocationUpdate_PostCloth_Post,
    CSTaskGroupIndex::LocationUpdate_DebugDraw,
    CSTaskGroupIndex::EventCondition_BonfireNearEnemyCheck,
    CSTaskGroupIndex::HavokWorldUpdate_Pre,
    CSTaskGroupIndex::RenderingSystemUpdate,
    CSTaskGroupIndex::HavokWorldUpdate_Post,
    CSTaskGroupIndex::ChrIns_PreCloth,
    CSTaskGroupIndex::ChrIns_PreClothSafe,
    CSTaskGroupIndex::HavokClothUpdate_Pre_AddRemoveRigidBody,
    CSTaskGroupIndex::HavokClothUpdate_Pre_ClothModelInsSafe,
    CSTaskGroupIndex::HavokClothUpdate_Pre_ClothModelIns,
    CSTaskGroupIndex::HavokClothUpdate_Pre_ClothManager,
    CSTaskGroupIndex::CameraStep,
    CSTaskGroupIndex::DrawParamUpdate,
    CSTaskGroupIndex::GetNPAuthCode,
    CSTaskGroupIndex::SoundStep,
    CSTaskGroupIndex::HavokClothUpdate_Post_ClothManager,
    CSTaskGroupIndex::HavokClothUpdate_Post_ClothModelIns,
    CSTaskGroupIndex::HavokClothVertexUpdateFinishWait,
    CSTaskGroupIndex::ChrIns_PostPhysics,
    CSTaskGroupIndex::ChrIns_PostPhysicsSafe,
    CSTaskGroupIndex::CSDistViewManager_Update,
    CSTaskGroupIndex::HavokAi_SilhouetteGeneratorHelper_Begin,
    CSTaskGroupIndex::WorldChrMan_PostPhysics,
    CSTaskGroupIndex::GameFlowInGame_MoveMap_PostPhysics_0,
    CSTaskGroupIndex::HavokAi_SilhouetteGeneratorHelper_End,
    CSTaskGroupIndex::DmgMan_Pre,
    CSTaskGroupIndex::DmgMan_ShapeCast,
    CSTaskGroupIndex::DmgMan_Post,
    CSTaskGroupIndex::GameFlowInGame_MoveMap_PostPhysics_1_Core0,
    CSTaskGroupIndex::GameFlowInGame_MoveMap_PostPhysics_1_Core1,
    CSTaskGroupIndex::GameFlowInGame_MoveMap_PostPhysics_1_Core2,
    CSTaskGroupIndex::MenuMan,
    CSTaskGroupIndex::WorldChrMan_Update_BackreadRequestPre,
    CSTaskGroupIndex::ChrIns_Update_BackreadRequest,
    CSTaskGroupIndex::WorldChrMan_Update_BackreadRequestPost,
    CSTaskGroupIndex::HavokAi_World,
    CSTaskGroupIndex::WorldAiManager_BeginUpdateFormation,
    CSTaskGroupIndex::WorldAiManager_EndUpdateFormation,
    CSTaskGroupIndex::GameFlowInGame_TestNet,
    CSTaskGroupIndex::GameFlowInGame_InGameMenu,
    CSTaskGroupIndex::GameFlowInGame_TitleMenu,
    CSTaskGroupIndex::GameFlowInGame_CommonMenu,
    CSTaskGroupIndex::GameFlowFrpgNet_Sys,
    CSTaskGroupIndex::GameFlowFrpgNet_Lobby,
    CSTaskGroupIndex::GameFlowFrpgNet_ConnectMan,
    CSTaskGroupIndex::GameFlowFrpgNet_Connect,
    CSTaskGroupIndex::GameFlowStep_Post,
    CSTaskGroupIndex::ScaleformStep,
    CSTaskGroupIndex::FlverResDelayDelectiionEnd,
    CSTaskGroupIndex::Draw_Pre,
    CSTaskGroupIndex::GraphicsStep,
    CSTaskGroupIndex::DebugDrawMemoryBar,
    CSTaskGroupIndex::DbgMenuStep,
    CSTaskGroupIndex::DbgRemoteStep,
    CSTaskGroupIndex::PlaylogSystemStep,
    CSTaskGroupIndex::ReviewMan,
    CSTaskGroupIndex::ReportSystemStep,
    CSTaskGroupIndex::DbgDispStep,
    CSTaskGroupIndex::DrawStep,
    CSTaskGroupIndex::DrawBegin,
    CSTaskGroupIndex::GameSceneDraw,
    CSTaskGroupIndex::AdhocDraw,
    CSTaskGroupIndex::DrawEnd,
    CSTaskGroupIndex::Draw_Post,
    CSTaskGroupIndex::SoundPlayLimitterUpdate,
    CSTaskGroupIndex::BeginShiftWorldPosition,
    CSTaskGroupIndex::FileStep,
    CSTaskGroupIndex::FileStepUpdate_Begin,
    CSTaskGroupIndex::FileStepUpdate_End,
    CSTaskGroupIndex::Flip,
    CSTaskGroupIndex::DelayDeleteStep,
    CSTaskGroupIndex::AiEndCollectGabage,
    CSTaskGroupIndex::RecordHeapStats,
    CSTaskGroupIndex::FrameEnd,
];