use crate::{*, uuid::*, types::*};
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PacketDirection {
    ClientBound,
    ServerBound,
}

impl PacketDirection {
    pub fn opposite(&self) -> Self {
        use PacketDirection::*;
        match self {
            ClientBound => ServerBound,
            ServerBound => ClientBound,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

impl State {
    pub fn name(&self) -> String {
        use State::*;
        match self {
            Handshaking => "Handshaking",
            Status => "Status",
            Login => "Login",
            Play => "Play",
        }.to_owned()
    }
}

define_protocol!(Packet578, PacketDirection, State, i32, Id => {
    // handshaking
    Handshake, 0x00, Handshaking, ServerBound => HandshakeSpec {
        version: VarInt,
        server_address: String,
        server_port: u16,
        next_state: HandshakeNextState
    },

    // status
    StatusRequest, 0x00, Status, ServerBound => StatusRequestSpec {},
    StatusPing, 0x01, Status, ServerBound => StatusPingSpec {
        payload: i64
    },
    StatusResponse, 0x00, Status, ClientBound => StatusResponseSpec {
        response: super::status::StatusSpec
    },
    StatusPong, 0x01, Status, ClientBound => StatusPongSpec {
        payload: i64
    },

    // login
    LoginDisconnect, 0x00, Login, ClientBound => LoginDisconnectSpec {
        message: Chat
    },
    LoginEncryptionRequest, 0x01, Login, ClientBound => LoginEncryptionRequestSpec {
        server_id: String,
        public_key: VarIntCountedArray<u8>,
        verify_token: VarIntCountedArray<u8>
    },
    LoginSuccess, 0x02, Login, ClientBound => LoginSuccessSpec {
        uuid_string: String,
        username: String
    },
    LoginSetCompression, 0x03, Login, ClientBound => LoginSetCompressionSpec {
        threshold: VarInt
    },
    LoginPluginRequest, 0x04, Login, ClientBound => LoginPluginRequestSpec {
        message_id: VarInt,
        channel: String,
        data: RemainingBytes
    },
    LoginStart, 0x00, Login, ServerBound => LoginStartSpec {
        name: String
    },
    LoginEncryptionResponse, 0x01, Login, ServerBound => LoginEncryptionResponseSpec {
        shared_secret: VarIntCountedArray<u8>,
        verify_token: VarIntCountedArray<u8>
    },
    LoginPluginResponse, 0x02, Login, ServerBound => LoginPluginResponseSpec {
        message_id: VarInt,
        successful: bool,
        data: RemainingBytes
    },

    // play
    // client bound
    PlaySpawnEntity, 0x00, Play, ClientBound => PlaySpawnEntitySpec {
        entity_id: VarInt,
        object_uuid: UUID4,
        entity_type: VarInt,
        x: f64,
        y: f64,
        z: f64,
        pitch: Angle,
        yaw: Angle,
        data: i32,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16
    },
    PlaySpawnExperienceOrb, 0x01, Play, ClientBound => PlaySpawnExperienceOrbSpec {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        count: i16
    },
    PlaySpawnWeatherEntity, 0x02, Play, ClientBound => PlaySpawnWeatherEntitySpec {
        entity_id: VarInt,
        entity_type: u8,
        x: f64,
        y: f64,
        z: f64
    },
    PlaySpawnLivingEntity, 0x03, Play, ClientBound => PlaySpawnLivingEntitySpec {
        entity_id: VarInt,
        entity_uuid: UUID4,
        entity_type: VarInt,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
        head_pitch: Angle,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16
    },
    PlaySpawnPainting, 0x04, Play, ClientBound => PlaySpawnPaintingSpec {
        entity_id: VarInt,
        entity_uuid: UUID4,
        motive: VarInt,
        location: IntPosition,
        direction: CardinalDirection
    },
    PlaySpawnPlayer, 0x05, Play, ClientBound => PlaySpawnPlayerSpec {
        entity_id: VarInt,
        uuid: UUID4,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle
    },
    PlayEntityAnimation, 0x06, Play, ClientBound => PlayEntityAnimationSpec {
        entity_id: VarInt,
        animation: EntityAnimationKind
    },
    PlayStatistics, 0x07, Play, ClientBound => PlayStatisticsSpec {
        entries: VarIntCountedArray<Statistic>
    },
    PlayAcknowledgePlayerDigging, 0x08, Play, ClientBound => PlayAcknowledgePlayerDiggingSpec {
        location: IntPosition,
        block: VarInt,
        status: DiggingStatus,
        successful: bool
    },
    PlayBlockBreakAnimation, 0x09, Play, ClientBound => PlayBlockBreakAnimationSpec {
        entity_id: VarInt,
        location: IntPosition,
        destroy_stage: i8
    },
    PlayBlockEntityData, 0x0A, Play, ClientBound => PlayBlockEntityDataSpec {
        location: IntPosition,
        action: BlockEntityDataAction,
        nbt_data: NamedNbtTag
    },
    PlayBlockAction, 0x0B, Play, ClientBound => PlayBlockActionSpec {
        location: IntPosition,
        action_id: u8,
        action_payload: u8,
        block_type: VarInt
    },
    PlayBlockChange, 0x0C, Play, ClientBound => PlayBlockChangeSpec {
        location: IntPosition,
        block_id: VarInt
    },
    PlayBossBar, 0x0D, Play, ClientBound => PlayBossBarSpec {
        uuid: UUID4,
        action: BossBarAction
    },
    PlayServerDifficulty, 0x0E, Play, ClientBound => PlayServerDifficultySpec {
        difficulty: Difficulty,
        locked: bool
    },
    PlayServerChatMessage, 0x0F, Play, ClientBound => PlayServerChatMessageSpec {
        message: Chat,
        position: ChatPosition
    },
    PlayMultiBlockChange, 0x10, Play, ClientBound => PlayMultiBlockChangeSpec {
        chunk_x: i32,
        chunk_z: i32,
        changes: VarIntCountedArray<MultiBlockChangeRecord>
    },
    PlayTabComplete, 0x11, Play, ClientBound => PlayTabCompleteSpec {
        id: VarInt,
        start: VarInt,
        length: VarInt,
        matches: VarIntCountedArray<TabCompleteMatch>
    },
    // SKIP PlayDeclareCommands
    PlayDeclareCommands, 0x12, Play, ClientBound => PlayDeclareCommandsSpec {
        raw_data: RemainingBytes
    },
    PlayServerWindowConfirmation, 0x13, Play, ClientBound => PlayServerWindowConfirmationSpec {
        window_id: u8,
        action_number: i16,
        accepted: bool
    },
    PlayServerCloseWindow, 0x14, Play, ClientBound => PlayServerCloseWindowSpec {
        window_id: u8
    },
    PlayWindowItems, 0x15, Play, ClientBound => PlayWindowItemsSpec {
        window_id: u8,
        slots: ShortCountedArray<Option<Slot>>
    },
    PlayWindowProperty, 0x16, Play, ClientBound => PlayWindowPropertySpec {
        window_id: u8,
        property: i16,
        value: i16
    },
    PlaySetSlot, 0x17, Play, ClientBound => PlaySetSlotSpec {
        window_id: u8,
        slow: i16,
        slot_data: Option<Slot>
    },
    PlaySetCooldown, 0x18, Play, ClientBound => PlaySetCooldownSpec {
        item_id: VarInt,
        cooldown_ticks: VarInt
    },
    PlayServerPluginMessage, 0x19, Play, ClientBound => PlayServerPluginMessageSpec {
        channel: String,
        data: RemainingBytes
    },
    PlayNamedSoundEffect, 0x1A, Play, ClientBound => PlayNamedSoundEffectSpec {
        sound_name: String,
        sound_category: SoundCategory,
        position_x: FixedInt,
        position_y: FixedInt,
        position_z: FixedInt,
        volume: f32,
        pitch: f32
    },
    PlayDisconnect, 0x1B, Play, ClientBound => PlayDisconnectSpec {
        reason: Chat
    },
    PlayEntityStatus, 0x1C, Play, ClientBound => PlayEntityStatusSpec {
        entity_id: VarInt,
        raw_status: u8 // todo deal with the gigantic table
    },
    PlayExposion, 0x1D, Play, ClientBound => PlayExposionSpec {
        x: f32,
        y: f32,
        z: f32,
        strength: f32,
        records: IntCountedArray<ExposionRecord>,
        player_motion_x: f32,
        player_motion_y: f32,
        player_motion_z: f32
    },
    PlayUnloadChunk, 0x1E, Play, ClientBound => PlayUnloadChunkSpec {
        x: i32,
        y: i32
    },
    PlayChangeGameState, 0x1F, Play, ClientBound => PlayChangeGameStateSpec {
        reason: GameChangeReason
    },
    PlayOpenHorseWindow, 0x20, Play, ClientBound => PlayOpenHorseWindowSpec {
        window_id: u8,
        number_of_slots: VarInt,
        entity_id: i32
    },
    PlayServerKeepAlive, 0x21, Play, ClientBound => PlayServerKeepAliveSpec {
        id: i64
    },
    PlayChunkData, 0x22, Play, ClientBound => PlayChunkDataWrapper {
        data: ChunkData
    },
    PlayEffect, 0x23, Play, ClientBound => PlayEffectSpec {
        effect_id: i32,
        location: IntPosition,
        data: i32,
        disable_relative_volume: bool
    },
    PlayParticle, 0x24, Play, ClientBound => PlayParticleSpec {
        particle_id: i32,
        long_distance: bool,
        x: f64,
        y: f64,
        z: f64,
        offset_x: f32,
        offset_y: f32,
        offset_z: f32,
        particle_data: i32,
        data: RemainingBytes // todo
    },
    PlayUpdateLight, 0x25, Play, ClientBound => PlayUpdateLoightSpec {
        chunk_x: VarInt,
        chunk_z: VarInt,
        sky_light_mask: VarInt,
        block_light_mask: VarInt,
        empty_sky_light_mask: VarInt,
        empty_block_light_mask: VarInt,
        sky_lights: VarIntCountedArray<u8>,
        block_lights: VarIntCountedArray<u8>
    },
    PlayJoinGame, 0x26, Play, ClientBound => PlayJoinGameSpec {
        entity_id: i32,
        gamemode: GameMode,
        dimension: Dimension,
        hashed_seed: i64,
        max_players: u8,
        level_type: String,
        view_distance: VarInt,
        reduced_debug_info: bool,
        enable_respawn_screen: bool
    },
    PlayMapData, 0x27, Play, ClientBound => PlayMapDataSpec {
        map_id: VarInt,
        scale: i8,
        tracking_position: bool,
        locked: bool,
        icons: VarIntCountedArray<MapIconSpec>,
        columns: MapColumns
    },
    PlayTradeList, 0x28, Play, ClientBound => PlayTradeListSpec {
        window_id: VarInt,
        trades: ByteCountedArray<TradeSpec>,
        villager_level: VarInt,
        experience: VarInt,
        regular_villager: bool,
        can_restock: bool
    },
    PlayEntityPosition, 0x29, Play, ClientBound => PlayEntityPositionSpec {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        on_ground: bool
    },
    PlayEntityPositionAndRotation, 0x2A, Play, ClientBound => PlayEntityPositionAndRotationSpec {
        entity_id: VarInt,
        delta_x: i16,
        delta_y: i16,
        delta_z: i16,
        yaw: Angle,
        pitch: Angle,
        on_ground: bool
    },
    PlayEntityRotation, 0x2B, Play, ClientBound => PlayEntityRotationSpec {
        entity_id: VarInt,
        yaw: Angle,
        pitch: Angle,
        on_ground: bool
    },
    PlayEntityMovement, 0x2C, Play, ClientBound => PlayEntityMovementSpec {
        entity_id: VarInt
    },
    PlayServerVehicleMove, 0x2D, Play, ClientBound => PlayEntityVehicleMoveSpec {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32
    },
    PlayOpenBook, 0x2E, Play, ClientBound => PlayOpenBookSpec {
        hand: Hand
    },
    PlayOpenWindow, 0x2F, Play, ClientBound => PlayOpenWindowSpec {
        id: VarInt,
        kind: WindowType,
        title: String
    },
    PlayOpenSignEditor, 0x30, Play, ClientBound => PlayOpenSignEditorSpec {
        location: IntPosition
    },
    PlayCraftRecipeResponse, 0x31, Play, ClientBound => PlayCraftRecipeResponseSpec {
        window_id: u8,
        recipe: String
    },
    PlayServerPlayerAbilities, 0x32, Play, ClientBound => PlayServerPlayerAbilitiesSpec {
        flags: PlayerAbilityFlags,
        flying_speed: f32,
        field_of_view_modifier: f32
    },
    PlayCombatEvent, 0x33, Play, ClientBound => PlayCombatEventSpec {
        event: CombatEvent
    },
    PlayPlayerInfo, 0x34, Play, ClientBound => PlayPlayerInfoSpec {
        actions: PlayerInfoActionList
    },
    PlayFacePlayer, 0x35, Play, ClientBound => PlayFacePlayerSpec {
        face_kind: FacePlayerKind,
        target_x: f64,
        target_y: f64,
        target_z: f64,
        entity: Option<FacePlayerEntityTarget>
    },
    PlayServerPlayerPositionAndLook, 0x36, Play, ClientBound => PlayServerPlayerPositionAndLookSpec {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        flags: PositionAndLookFlags,
        teleport_id: VarInt
    },
    PlayUnlockRecipes, 0x37, Play, ClientBound => PlayUnlockRecipesSpec {
        action: RecipeUnlockAction,
        crafting_book_open: bool,
        crafting_book_active: bool,
        smelting_book_open: bool,
        smelting_book_active: bool,
        recipe_ids: VarIntCountedArray<String>,
        other_recipe_ids: RemainingBytes // todo
    },
    PlayDestroyEntities, 0x38, Play, ClientBound => PlayDestroyEntitiesSpec {
        entity_ids: VarIntCountedArray<VarInt>
    },
    PlayRemoveEntityEffect, 0x39, Play, ClientBound => PlayRemoveEntityEffectSpec {
        entity_id: VarInt,
        effect: EntityEffectKind
    },
    PlayResourcePackSend, 0x3A, Play, ClientBound => PlayResourcePackSendSpec {
        url: String,
        hash: String
    },
    PlayRespawn, 0x3B, Play, ClientBound => PlayRespawnSpec {
        dimension: Dimension,
        hashed_seed: i64,
        gamemode: GameMode,
        level_type: String
    },
    PlayEntityHeadLook, 0x3C, Play, ClientBound => PlayEntityHeadLookSpec {
        entity_id: VarInt,
        head_yaw: Angle
    },
    PlaySelectAdvancementTab, 0x3D, Play, ClientBound => PlaySelectAdvancementTabSpec {
        identifier: Option<String>
    },
    PlayWorldBorder, 0x3E, Play, ClientBound => PlayWorldBorderSpec {
        action: WorldBorderAction
    },
    PlayCamera, 0x3F, Play, ClientBound => PlayCameraSpec {
        camera_id: VarInt
    },
    PlayServerHeldItemChange, 0x40, Play, ClientBound => PlayServerHeldItemChangeSpec {
        slot: i8
    },
    PlayUpdateViewPosition, 0x41, Play, ClientBound => PlayUpdateViewPositionSpec {
        chunk_x: VarInt,
        chunk_z: VarInt
    },
    PlayUpdateViewDistance, 0x42, Play, ClientBound => PlayUpdateViewDistanceSpec {
        view_distance: VarInt
    },
    PlayDisplayScoreboard, 0x43, Play, ClientBound => PlayDisplayScoreboardSpec {
        position: ScoreboardPosition,
        score_name: String
    },
    PlayEntityMetadata, 0x44, Play, ClientBound => PlayEntityMetadataSpec {
        entity_id: VarInt,
        metadata_raw: RemainingBytes
    },
    PlayAttachEntity, 0x45, Play, ClientBound => PlayAttachEntitySpec {
        attached_entity_id: i32,
        holding_entity_id: i32
    },
    PlayEntityVelocity, 0x46, Play, ClientBound => PlayEntityVelocitySpec {
        entity_id: VarInt,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16
    },
    PlayEntityEquipment, 0x47, Play, ClientBound => PlayEntityEquiptmentSpec {
        entity_id: VarInt,
        slot: EquipmentSlot,
        item: Option<Slot>
    },
    PlaySetExperience, 0x48, Play, ClientBound => PlaySetExperienceSpec {
        experience_bar: f32,
        level: VarInt,
        total_experience: VarInt
    },
    PlayUpdatehealth, 0x49, Play, ClientBound => PlayUpdateHealthSpec {
        health: f32,
        food: VarInt,
        saturation: f32
    },
    PlayScoreboardObjective, 0x4A, Play, ClientBound => PlayScoreboardObjectiveSpec {
        objective_name: String,
        action: ScoreboardObjectiveAction
    },
    PlaySetPassengers, 0x4B, Play, ClientBound => PlaySetPassengersSpec {
        entity_id: VarInt,
        passenger_entitiy_ids: VarIntCountedArray<VarInt>
    },
    // todo teams
    // todo update score
    PlaySpawnPosition, 0x4E, Play, ClientBound => PlaySpawnPositionSpec {
        location: IntPosition
    },
    PlayTimeUpdate, 0x4F, Play, ClientBound => PlayTimeUpdateSpec {
        world_age: i64,
        time_of_day: i64
    },
    // todo title
    PlayEntitySoundEffect, 0x51, Play, ClientBound => PlayEntitySoundEffectSpec {
        sound_id: VarInt,
        sound_category: SoundCategory,
        entity_id: VarInt,
        volume: f32,
        pitch: f32
    },
    PlaySoundEffect, 0x52, Play, ClientBound => PlaySoundEffectSpec {
        sound_id: VarInt,
        sound_category: SoundCategory,
        position_x: FixedInt,
        position_y: FixedInt,
        position_z: FixedInt,
        volume: f32,
        pitch: f32
    },
    // todo stop sound
    PlayerPlayerListHeaderAndFooter, 0x54, Play, ClientBound => PlayPlayerListHeaderAndFooterSpec {
        header: Chat,
        footer: Chat
    },
    PlayNbtQueryResponse, 0x55, Play, ClientBound => PlayNbtQueryResponseSpec {
        transaction_id: VarInt,
        nbt: NamedNbtTag
    },
    PlayCollectItem, 0x56, Play, ClientBound => PlayCollectItemSpec {
        collected_entity_id: VarInt,
        collector_entity_id: VarInt,
        pickup_item_count: VarInt
    },
    PlayEntityTeleport, 0x57, Play, ClientBound => PlayEntityTeleportSpec {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
        on_ground: bool
    },
    // todo advancements
    PlayAdvancements, 0x58, Play, ClientBound => PlayAdvancementsSpec {
        raw: RemainingBytes
    },
    PlayEntityProperties, 0x59, Play, ClientBound => PlayEntityPropertiesSpec {
        entity_id: VarInt,
        properties: IntCountedArray<EntityPropertySpec>
    },
    PlayEntityEffect, 0x5A, Play, ClientBound => PlayEntityEffectSpec {
        entity_id: VarInt,
        effect_id: EntityEffectKind,
        amplifier: i8,
        duration_ticks: VarInt,
        flags: EntityEffectFlags
    },
    PlayDeclareRecipes, 0x5B, Play, ClientBound => PlayDeclareRecipesSpec {
        recipes: VarIntCountedArray<RecipeSpec>
    },
    PlayTags, 0x5C, Play, ClientBound => PlayTagsSpec {
        block_tags: VarIntCountedArray<TagSpec>,
        item_tags: VarIntCountedArray<TagSpec>,
        fluid_tags: VarIntCountedArray<TagSpec>,
        entity_tags: VarIntCountedArray<TagSpec>
    },

    // play server bound
    PlayTeleportConfirm, 0x00, Play, ServerBound => PlayTeleportConfirmSpec {
        teleport_id: VarInt
    },
    PlayQueryBlockNbt, 0x01, Play, ServerBound => PlayQueryBlockNbtSpec {
        transaction_id: VarInt,
        location: IntPosition
    },
    PlayQueryEntityNbt, 0x0D, Play, ServerBound => PlayQueryEntityNbtSpec {
        transaction_id: VarInt,
        entity_id: VarInt
    },
    PlaySetDifficulty, 0x02, Play, ServerBound => PlaySetDifficultySpec {
        new_difficulty: Difficulty
    },
    PlayClientChatMessage, 0x03, Play, ServerBound => PlayClientChatMessageSpec {
        message: String
    },
    PlayClientStatus, 0x04, Play, ServerBound => PlayClientStatusSpec {
        action: ClientStatusAction
    },
    PlayClientSettings, 0x05, Play, ServerBound => PlayClientSettingsSpec {
        locale: String,
        view_distance: i8,
        chat_mode: ClientChatMode,
        chat_colors: bool,
        displayed_skin_parts: ClientDisplayedSkinParts,
        main_hand: ClientMainHand
    },
    PlayClientTabComplete, 0x06, Play, ServerBound => PlayClientTabCompleteSpec {
        transaction_id: VarInt,
        text: String
    },
    PlayClientWindowConfirmation, 0x07, Play, ServerBound => PlayClientWindowConfirmationSpec {
        window_id: i8,
        action_num: i16,
        accepted: bool
    },
    PlayClickWindowButton, 0x08, Play, ServerBound => PlayClickWindowButtonSpec {
        window_id: i8,
        button_id: i8
    },
    PlayClickWindow, 0x09, Play, ServerBound => PlayClickWindowSpec {
        window_id: u8,
        slot: i16,
        button: i8,
        action_number: i16,
        mode: InventoryOperationMode,
        clicked_item: Option<Slot>
    },
    PlayClientCloseWindow, 0x0A, Play, ServerBound => PlayClientCloseWindowSpec {
        window_id: u8
    },
    PlayClientPluginMessage, 0x0B, Play, ServerBound => PlayClientPluginMessageSpec {
        channel: String,
        data: RemainingBytes
    },
    PlayEditBook, 0x0C, Play, ServerBound => PlayEditBookSpec {
        new_book: Option<Slot>,
        is_signing: bool,
        hand: Hand
    },
    PlayInteractEntity, 0x0E, Play, ServerBound => PlayInteractEntitySpec {
        entity_id: VarInt,
        kind: InteractKind
    },
    PlayClientKeepAlive, 0x0F, Play, ServerBound => PlayClientKeepAliveSpec {
        id: i64
    },
    PlayLockDifficulty, 0x10, Play, ServerBound => PlayLockDifficultySpec {
        locked: bool
    },
    PlayPlayerPosition, 0x11, Play, ServerBound => PlayPlayerPositionSpec {
        x: f64,
        feet_y: f64,
        z: f64,
        on_ground: bool
    },
    PlayClientPlayerPositionAndRotation, 0x12, Play, ServerBound => PlayClientPlayerPositionAndRotationSpec {
        x: f64,
        feet_y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool
    },
    PlayPlayerRotation, 0x13, Play, ServerBound => PlayPlayerRotationSpec {
        yaw: f32,
        pitch: f32,
        on_ground: bool
    },
    PlayPlayerMovement, 0x14, Play, ServerBound => PlayPlayerMovementSpec {
        on_ground: bool
    },
    PlayClientVehicleMove, 0x15, Play, ServerBound => PlayClientVehicleMoveSpec {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32
    },
    PlaySteerBoat, 0x16, Play, ServerBound => PlaySteerBoatSpec {
        left_paddle_turning: bool,
        right_paddle_turning: bool
    },
    PlayPickItem, 0x17, Play, ServerBound => PlayPickItemSpec {
        slot_idx: VarInt
    },
    PlayCraftRecipeRequest, 0x18, Play, ServerBound => PlayCraftRecipeRequestSpec {
        window_id: i8,
        recipe: String,
        make_all: bool
    },
    PlayClientPlayerAbilities, 0x19, Play, ServerBound => PlayClientPlayerAbilitiesSpec {
        flags: ClientPlayerAbilities,
        flying_speed: f32,
        walking_speed: f32
    },
    PlayPlayerDigging, 0x1A, Play, ServerBound => PlayPlayerDiggingSpec {
        status: PlayerDiggingStatus,
        location: IntPosition,
        face: DiggingFace
    },
    PlayEntityAction, 0x1B, Play, ServerBound => PlayEntityActionSpec {
        entity_id: VarInt,
        action: EntityActionKind,
        jump_boot: VarInt
    },
    PlaySteerVehicle, 0x1C, Play, ServerBound => PlaySteerVehicleSpec {
        sideways: f32,
        forward: f32,
        flags: SteerVehicleFlags
    },
    // todo recipe book data
    PlayNameItem, 0x1E, Play, ServerBound => PlayNameItemSpec {
        name: String
    },
    PlayResourcePackStatus, 0x1F, Play, ServerBound => PlayResourcePackStatusSpec {
        status: ResourcePackStatus
    },
    // todo advancement tab
    PlaySelectTrade, 0x21, Play, ServerBound => PlaySelectTradeSpec {
        selected_slot: VarInt
    },
    PlaySetBeaconEffect, 0x22, Play, ServerBound => PlaySetBeaconEffectSpec {
        primary_effect: VarInt,
        secondary_effect: VarInt
    },
    PlayClientHeldItemChange, 0x23, Play, ServerBound => PlayClientHeldItemChangeSpec {
        slot: i16
    },
    PlayUpdateCommandBlock, 0x24, Play, ServerBound => PlayUpdateCommandBlockSpec {
        location: IntPosition,
        command: String,
        mode: CommandBlockMode,
        flags: CommandBlockFlags
    },
    PlayUpdateCommandBlockMinecart, 0x25, Play, ServerBound => PlayUpdateCommandBlockMinecartSpec {
        entity_id: VarInt,
        command: String,
        track_output: bool
    },
    PlayCreativeInventoryAction, 0x26, Play, ServerBound => PlayCreativeInventoryActionSpec {
        slot: i16,
        clicked_item: Option<Slot>
    },
    PlayUpdateJigsawBlock, 0x27, Play, ServerBound => PlayUpdateJigsawBlockSpec {
        location: IntPosition,
        attachment_type: String,
        target_pool: String,
        final_state: String
    },
    PlayUpdateStructureBlock, 0x28, Play, ServerBound => PlayUpdateStructureBlockSpec {
        location: IntPosition,
        action: UpdateStructureBlockAction,
        mode: UpdateStructureBlockMode,
        name: String,
        offset_x: i8,
        offset_y: i8,
        offset_z: i8,
        size_x: i8,
        size_y: i8,
        size_z: i8,
        mirror: UpdateStructureBlockMirror,
        rotation: UpdateStructureBlockRotation,
        metadata: String,
        integrity: f32,
        seed: VarLong,
        flags: UpdateStructureBlockFlags
    },
    PlayUpdateSign, 0x29, Play, ServerBound => PlayUpdateSignSpec {
        location: IntPosition,
        line1: String,
        line2: String,
        line3: String,
        line4: String
    },
    PlayClientAnimation, 0x2A, Play, ServerBound => PlayClientAnimationSpec {
        hand: Hand
    },
    PlaySpectate, 0x2B, Play, ServerBound => PlaySpectateSpec {
        target: UUID4
    },
    PlayBlockPlacement, 0x2C, Play, ServerBound => PlayBlockPlacementSpec {
        hand: Hand,
        location: IntPosition,
        face: DiggingFace,
        cursor_position_x: f32,
        cursor_position_y: f32,
        cursor_position_z: f32,
        inside_block: bool
    },
    PlayUseItem, 0x2D, Play, ServerBound => PlayUseItemSpec {
        hand: Hand
    }
});

// helper types

// handshake enum
proto_byte_enum!(HandshakeNextState,
    0x01 :: Status,
    0x02 :: Login
);

macro_rules! counted_array_type {
    ($name: ident, $countert: ty, $tousize_fn: ident, $fromusize_fn: ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name<T> where T: Debug + Clone + PartialEq {
            pub data: Vec<T>
        }

        impl<T> Serialize for $name<T> where T: Serialize + Debug + Clone + PartialEq {
            fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
                let count: $countert = $fromusize_fn(self.data.len());
                to.serialize_other(&count)?;

                for entry in &self.data {
                    to.serialize_other(entry)?;
                }

                Ok(())
            }
        }

        impl<T> Deserialize for $name<T> where T: Deserialize + Debug + Clone + PartialEq {
            fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
                let Deserialized{value: raw_count, mut data} = <$countert>::mc_deserialize(data)?;
                let count: usize = $tousize_fn(raw_count);

                let mut out = Vec::with_capacity(count);
                for _ in 0..count {
                    let Deserialized{value: next, data: rest} = T::mc_deserialize(data)?;
                    data = rest;
                    out.push(next);
                }

                Deserialized::ok(Self { data: out }, data)
            }
        }

        impl<T> Into<Vec<T>> for $name<T> where T: Debug + Clone + PartialEq {
            fn into(self) -> Vec<T> {
                self.data
            }
        }

        impl<T> From<Vec<T>> for $name<T> where T: Debug + Clone + PartialEq {
            fn from(data: Vec<T>) -> Self {
                Self { data }
            }
        }
    }
}

#[inline]
fn varint_to_usize(v: VarInt) -> usize {
    v.into()
}

#[inline]
fn varint_from_usize(u: usize) -> VarInt {
    u.into()
}
counted_array_type!(VarIntCountedArray, VarInt, varint_to_usize, varint_from_usize);

#[inline]
fn i16_to_usize(v: i16) -> usize {
    v as usize
}

#[inline]
fn i16_from_usize(u: usize) -> i16 {
    u as i16
}
counted_array_type!(ShortCountedArray, i16, i16_to_usize, i16_from_usize);

#[inline]
fn i32_to_usize(v: i32) -> usize {
    v as usize
}

#[inline]
fn i32_from_usize(u: usize) -> i32 {
    u as i32
}
counted_array_type!(IntCountedArray, i32, i32_to_usize, i32_from_usize);

#[inline]
fn i8_to_usize(v: i8) -> usize {
    v as usize
}

#[inline]
fn i8_from_usize(u: usize) -> i8 {
    u as i8
}
counted_array_type!(ByteCountedArray, i8, i8_to_usize, i8_from_usize);

#[derive(Debug, Clone, PartialEq)]
pub struct RemainingBytes {
    pub data: Vec<u8>,
}

impl Serialize for RemainingBytes {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_bytes(self.data.as_slice())
    }
}

impl Deserialize for RemainingBytes {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Deserialized::ok(RemainingBytes { data: Vec::from(data) }, &[])
    }
}

impl Into<Vec<u8>> for RemainingBytes {
    fn into(self) -> Vec<u8> {
        self.data
    }
}

impl From<Vec<u8>> for RemainingBytes {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}

proto_byte_enum!(CardinalDirection,
    0x00 :: South,
    0x01 :: West,
    0x02 :: North,
    0x03:: East
);

proto_byte_enum!(EntityAnimationKind,
    0x00 :: SwingMainArm,
    0x01 :: TakeDamage,
    0x02 :: LeaveBed,
    0x03 :: SwingOffHand,
    0x04 :: CriticalEffect,
    0x05 :: MagicCriticalEffect
);

proto_varint_enum!(StatisticCategory,
    0x00 :: Mined,
    0x01 :: Crafted,
    0x02 :: Used,
    0x03 :: Broken,
    0x04 :: PickedUp,
    0x05 :: Dropped,
    0x06 :: Killed,
    0x07 :: KilledBy,
    0x08 :: Custom
);

proto_varint_enum!(StatisticKind,
    0x00 :: LeaveGame,
    0x01 :: PlayOneMinute,
    0x02 :: TimeSinceDeath,
    0x03 :: SneakTime,
    0x04 :: WealkOneCm,
    0x05 :: CrouchOneCm,
    0x06 :: SprintOneCm,
    0x07 :: SwimOneCm,
    0x08 :: FallOneCm,
    0x09 :: ClimbOneCm,
    0x0A :: FlyOneCm,
    0x0B :: DiveOneCm,
    0x0C :: MinecartOneCm,
    0x0D :: BoatOneCm,
    0x0E :: PigOneCm,
    0x0F :: HorseOneCm,
    0x10 :: AviateOneCm,
    0x11 :: Jumps,
    0x12 :: Drops,
    0x13 :: DamageDealt,
    0x14 :: DamageTaken,
    0x15 :: Deaths,
    0x16 :: MobKills,
    0x17 :: AnimalsBread,
    0x18 :: PlayerKills,
    0x19 :: FishCaught,
    0x1A :: TalkedToVillager,
    0x1B :: TradedWithVillager,
    0x1C :: EatCakeSlice,
    0x1D :: FillCauldron,
    0x1E :: UseCauldron,
    0x1F :: CleanArmor,
    0x20 :: CleanBanner,
    0x21 :: InteractWithBrewingStand,
    0x22 :: InteractWithBeaccon,
    0x23 :: InspectDropper,
    0x24 :: InspectHopper,
    0x25 :: InspectDispenser,
    0x26 :: PlayNoteBlock,
    0x27 :: TuneNoteBlock,
    0x28 :: PotFlower,
    0x29 :: TriggerTrappedChest,
    0x2A :: OpenEnderChest,
    0x2B :: EnchantItem,
    0x2C :: PlayRecord,
    0x2D :: InteractWithFurnace,
    0x2E :: InteractWithCraftingTable,
    0x2F :: OpenChest,
    0x30 :: SleepInBed,
    0x31 :: OpenShulkerBox
);

__protocol_body_def_helper!(Statistic {
    category: StatisticCategory,
    statistic: StatisticKind,
    value: VarInt
});

proto_byte_enum!(DiggingStatus,
    0x00 :: Started,
    0x01 :: Cancelled,
    0x02 :: Finished
);

proto_byte_enum!(BlockEntityDataAction,
    0x01 :: SetMobSpawnerData,
    0x02 :: SetCommandBlockText,
    0x03 :: SetBeaconLevelAndPower,
    0x04 :: SetMobHeadRotationAndSkin,
    0x05 :: DeclareConduit,
    0x06 :: SetBannerColorAndPatterns,
    0x07 :: SetStructureTileEntityData,
    0x08 :: SetEndGatewayDestination,
    0x09 :: SetSignText,
    0x0B :: DeclareBed,
    0x0C :: SetJigsawBlockData,
    0x0D :: SetCampfireItems,
    0x0E :: BeehiveInformation
);

proto_byte_enum!(Difficulty,
    0x00 :: Peaceful,
    0x01 :: Easy,
    0x02 :: Normal,
    0x03 :: Hard
);

proto_byte_enum!(ChatPosition,
    0x00 :: ChatBox,
    0x01 :: SystemMessage,
    0x02 :: Hotbar
);

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BlockChangeHorizontalPosition {
    pub rel_x: u8,
    pub rel_y: u8,
}

impl Serialize for BlockChangeHorizontalPosition {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte((self.rel_x & 0xF) << 4 | (self.rel_y & 0xF))
    }
}

impl Deserialize for BlockChangeHorizontalPosition {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        Ok(u8::mc_deserialize(data)?.map(move |b| {
            BlockChangeHorizontalPosition {
                rel_x: (b >> 4) & 0xF,
                rel_y: b & 0xF,
            }
        }))
    }
}

__protocol_body_def_helper!(MultiBlockChangeRecord {
    horizontal_position: BlockChangeHorizontalPosition,
    y_coordinate: u8,
    block_id: VarInt
});

#[derive(Debug, Clone, PartialEq)]
pub enum BossBarAction {
    Add(BossBarAddSpec),
    Remove,
    UpdateHealth(BossBarUpdateHealthSpec),
    UpdateTitle(BossBarUpdateTitleSpec),
    UpdateStyle(BossBarUpdateStyleSpec),
    UpdateFlags(BossBarUpdateFlagsSpec),
}

impl Serialize for BossBarAction {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        use BossBarAction::*;
        to.serialize_other(&VarInt(match self {
            Add(_) => 0x00,
            Remove => 0x01,
            UpdateHealth(_) => 0x02,
            UpdateTitle(_) => 0x03,
            UpdateStyle(_) => 0x04,
            UpdateFlags(_) => 0x05,
        }))?;
        match self {
            Add(body) => to.serialize_other(body),
            Remove => Ok(()),
            UpdateHealth(body) => to.serialize_other(body),
            UpdateTitle(body) => to.serialize_other(body),
            UpdateStyle(body) => to.serialize_other(body),
            UpdateFlags(body) => to.serialize_other(body),
        }
    }
}

impl Deserialize for BossBarAction {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: type_id, data } = VarInt::mc_deserialize(data)?;
        use BossBarAction::*;
        match type_id.0 {
            0x00 => Ok(BossBarAddSpec::mc_deserialize(data)?.map(move |body| Add(body))),
            0x01 => Deserialized::ok(Remove, data),
            0x02 => Ok(BossBarUpdateHealthSpec::mc_deserialize(data)?.map(move |body| UpdateHealth(body))),
            0x03 => Ok(BossBarUpdateTitleSpec::mc_deserialize(data)?.map(move |body| UpdateTitle(body))),
            0x04 => Ok(BossBarUpdateStyleSpec::mc_deserialize(data)?.map(move |body| UpdateStyle(body))),
            0x05 => Ok(BossBarUpdateFlagsSpec::mc_deserialize(data)?.map(move |body| UpdateFlags(body))),
            other => Err(DeserializeErr::CannotUnderstandValue(format!("invalid boss bar action id {:x}", other)))
        }
    }
}

proto_varint_enum!(BossBarColor,
    0x00 :: Pink,
    0x01 :: Blue,
    0x02 :: Red,
    0x03 :: Green,
    0x04 :: Yellow,
    0x05 :: Purple,
    0x06 :: White
);

proto_varint_enum!(BossBarDivision,
    0x00 :: NoDivision,
    0x01 :: SixNotches,
    0x02 :: TenNotches,
    0x03 :: TwelveNotches,
    0x04 :: TwentyNotches
);

proto_byte_flag!(BossBarFlags,
    0x01 :: darken_sky,
    0x02 :: dragon_bar,
    0x04 :: create_fog
);

__protocol_body_def_helper!(BossBarAddSpec {
    title: Chat,
    health: f32,
    color: BossBarColor,
    division: BossBarDivision,
    flags: BossBarFlags
});

__protocol_body_def_helper!(BossBarUpdateHealthSpec {
    health: f32
});

__protocol_body_def_helper!(BossBarUpdateTitleSpec {
    title: String
});

__protocol_body_def_helper!(BossBarUpdateStyleSpec {
    color: BossBarColor,
    dividers: BossBarDivision
});

__protocol_body_def_helper!(BossBarUpdateFlagsSpec {
    flags: BossBarFlags
});

__protocol_body_def_helper!(TabCompleteMatch {
    match_: String,
    tooltip: Option<Chat>
});

proto_varint_enum!(SoundCategory,
    0x00 :: Master,
    0x01 :: Music,
    0x02 :: Records,
    0x03 :: Weather,
    0x04 :: Block,
    0x05 :: Hostile,
    0x06 :: Neutral,
    0x07 :: Player,
    0x08 :: Ambient,
    0x09 :: Voice
);

__protocol_body_def_helper!(ExposionRecord {
    x: i8,
    y: i8,
    z: i8
});

proto_byte_enum!(GameMode,
    0x00 :: Survival,
    0x01 :: Creative,
    0x02 :: Adventure,
    0x03 :: Spectator
);

proto_byte_enum!(WinGameAction,
    0x00 :: Respawn,
    0x01 :: RollCreditsAndRespawn
);

proto_byte_enum!(DemoEvent,
    0x00 :: ShowWelcomeScreen,
    0x65 :: TellMovementControls,
    0x66 :: TellJumpControl,
    0x67 :: TellInventoryControl,
    0x68 :: EndDemo
);

proto_byte_enum!(RespawnRequestType,
    0x00 :: Screen,
    0x01 :: Immediate
);

proto_int_enum!(Dimension,
    -0x01 :: Nether,
     0x00 :: Overworld,
     0x01 :: End
);

#[derive(Clone, Debug, PartialEq)]
pub enum GameChangeReason {
    NoRespawnAvailable,
    EndRaining,
    BeginRaining,
    ChangeGameMode(GameMode),
    WinGame(WinGameAction),
    Demo(DemoEvent),
    ArrowHitPlayer,
    RainLevelChange(f32),
    ThunderLevelChange(f32),
    PufferfishSting,
    ElderGuardianMobAppearance,
    Respawn(RespawnRequestType),
}

impl Serialize for GameChangeReason {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        use GameChangeReason::*;
        to.serialize_byte(match self {
            NoRespawnAvailable => 0x00,
            EndRaining => 0x01,
            BeginRaining => 0x02,
            ChangeGameMode(_) => 0x03,
            WinGame(_) => 0x04,
            Demo(_) => 0x05,
            ArrowHitPlayer => 0x06,
            RainLevelChange(_) => 0x07,
            ThunderLevelChange(_) => 0x08,
            PufferfishSting => 0x09,
            ElderGuardianMobAppearance => 0x0A,
            Respawn(_) => 0x0B
        })?;

        let value = match self {
            ChangeGameMode(body) => {
                body.as_byte() as f32
            }
            WinGame(body) => {
                body.as_byte() as f32
            }
            Demo(body) => {
                body.as_byte() as f32
            }
            RainLevelChange(body) => *body,
            ThunderLevelChange(body) => *body,
            Respawn(body) => {
                body.as_byte() as f32
            }
            _ => 0 as f32
        };
        to.serialize_other(&value)
    }
}

impl Deserialize for GameChangeReason {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: reason_id, data } = u8::mc_deserialize(data)?;
        let Deserialized { value, data } = f32::mc_deserialize(data)?;
        use GameChangeReason::*;
        let out = match reason_id {
            0x00 => Ok(NoRespawnAvailable),
            0x01 => Ok(EndRaining),
            0x02 => Ok(BeginRaining),
            0x03 => Ok(ChangeGameMode(
                GameMode::from_byte(value as u8)
                    .map(move |v| Ok(v))
                    .unwrap_or_else(|| Err(DeserializeErr::CannotUnderstandValue(format!("unknown gamemode value {}", value))))?)),
            0x04 => Ok(WinGame(
                WinGameAction::from_byte(value as u8)
                    .map(move |v| Ok(v))
                    .unwrap_or_else(|| Err(DeserializeErr::CannotUnderstandValue(format!("unknown WinGame value {}", value))))?)),
            0x05 => Ok(Demo(
                DemoEvent::from_byte(value as u8)
                    .map(move |v| Ok(v))
                    .unwrap_or_else(|| Err(DeserializeErr::CannotUnderstandValue(format!("unknown DemoEvent value {}", value))))?)),
            0x06 => Ok(ArrowHitPlayer),
            0x07 => Ok(RainLevelChange(value)),
            0x08 => Ok(ThunderLevelChange(value)),
            0x09 => Ok(PufferfishSting),
            0x0A => Ok(ElderGuardianMobAppearance),
            0x0B => Ok(Respawn(
                RespawnRequestType::from_byte(value as u8)
                    .map(move |v| Ok(v))
                    .unwrap_or_else(|| Err(DeserializeErr::CannotUnderstandValue(format!("invalid respawn reason {}", value))))?)),
            other => Err(DeserializeErr::CannotUnderstandValue(format!("invalid game change reason id {}", other)))
        }?;

        Deserialized::ok(out, data)
    }
}

proto_varint_enum!(MapIconType,
    0x00 :: WhiteArrow,
    0x01 :: GreenArrow,
    0x02 :: RedArrow,
    0x03 :: BlueArrow,
    0x04 :: WhiteCross,
    0x05 :: RedPointer,
    0x06 :: WhiteCircle,
    0x07 :: SmallWhiteCircle,
    0x08 :: Mansion,
    0x09 :: Temple,
    0x0A :: WhiteBanner,
    0x0B :: OrangeBanner,
    0x0C :: MagentaBanner,
    0x0D :: YellowBanner,
    0x0E :: LimeBanner,
    0x0F :: PinkBanner,
    0x10 :: GrayBanner,
    0x11 :: LightGrayBanner,
    0x12 :: CyanBanner,
    0x13 :: PurpleBanner,
    0x14 :: BlueBanner,
    0x15 :: BrownBanner,
    0x16 :: GreenBanner,
    0x17 :: RedBanner,
    0x18 :: BlackBanner,
    0x19 :: TreasureMarker
);

__protocol_body_def_helper!(MapIconSpec {
    kind: MapIconType,
    x: i8,
    z: i8,
    direction: i8,
    display_name: Option<Chat>
});

#[derive(Clone, PartialEq, Debug)]
pub enum MapColumns {
    NoUpdates,
    Updated(MapColumnsSpec),
}

__protocol_body_def_helper!(MapColumnsSpec {
    columns: u8,
    rows: u8,
    x: u8,
    z: u8,
    data: VarIntCountedArray<u8>
});

impl Serialize for MapColumns {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        use MapColumns::*;
        match self {
            NoUpdates => to.serialize_other(&0u8),
            Updated(body) => to.serialize_other(body),
        }
    }
}

impl Deserialize for MapColumns {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: columns, data: rest } = u8::mc_deserialize(data)?;
        use MapColumns::*;
        match columns {
            0x00 => Deserialized::ok(NoUpdates, rest),
            _ => Ok(MapColumnsSpec::mc_deserialize(data)?.map(move |v| Updated(v)))
        }
    }
}

impl Into<Option<MapColumnsSpec>> for MapColumns {
    fn into(self) -> Option<MapColumnsSpec> {
        use MapColumns::*;
        match self {
            NoUpdates => None,
            Updated(body) => Some(body)
        }
    }
}

impl From<Option<MapColumnsSpec>> for MapColumns {
    fn from(other: Option<MapColumnsSpec>) -> Self {
        use MapColumns::*;
        match other {
            Some(body) => Updated(body),
            None => NoUpdates
        }
    }
}

__protocol_body_def_helper!(TradeSpec {
    input_item_1: Option<Slot>,
    output_item: Option<Slot>,
    input_item_2: Option<Slot>,
    trade_disabled: bool,
    trade_uses: i32,
    max_trade_uses: i32,
    xp: i32,
    special_price: i32,
    price_multiplier: f32,
    demand: i32
});

proto_varint_enum!(Hand,
    0x00 :: MainHand,
    0x01 :: OffHand
);

proto_varint_enum!(WindowType,
    0x00 :: GenericOneRow,
    0x01 :: GenericTwoRow,
    0x02 :: GenericThreeRow,
    0x03 :: GenericFourRow,
    0x04 :: GenericFiveRow,
    0x05 :: GenericSixRow,
    0x06 :: GenericSquare,
    0x07 :: Anvil,
    0x08 :: Beacon,
    0x09 :: BlastFurnace,
    0x0A :: BrewingStand,
    0x0B :: CraftingTable,
    0x0C :: EnchantmentTable,
    0x0D :: Furnace,
    0x0E :: Grindstone,
    0x0F :: Hopper,
    0x10 :: Lectern,
    0x11 :: Loom,
    0x12 :: Merchant,
    0x13 :: ShulkerBox,
    0x14 :: Smoker,
    0x15 :: Cartography,
    0x16 :: StoneCutter
);

proto_byte_flag!(PlayerAbilityFlags,
    0x01 :: invulnerable,
    0x02 :: flying,
    0x04 :: allow_flying,
    0x08 :: instant_break
);

#[derive(Clone, PartialEq, Debug)]
pub enum CombatEvent {
    Enter,
    End(CombatEndSpec),
    EntityDead(CombatEntityDeadSpec),
}

__protocol_body_def_helper!(CombatEndSpec {
    duration_ticks: VarInt,
    entity_id: i32
});

__protocol_body_def_helper!(CombatEntityDeadSpec {
    player_id: VarInt,
    entity_id: i32,
    message: Chat
});

impl Serialize for CombatEvent {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        use CombatEvent::*;
        to.serialize_other(&VarInt(match self {
            Enter => 0x00,
            End(_) => 0x01,
            EntityDead(_) => 0x02
        }))?;

        match self {
            End(body) => to.serialize_other(body)?,
            EntityDead(body) => to.serialize_other(body)?,
            _ => {}
        }

        Ok(())
    }
}

impl Deserialize for CombatEvent {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: action_id, data } = VarInt::mc_deserialize(data)?;

        use CombatEvent::*;
        match action_id.0 {
            0x00 => Deserialized::ok(Enter, data),
            0x01 => Ok(CombatEndSpec::mc_deserialize(data)?.map(move |body| End(body))),
            0x02 => Ok(CombatEntityDeadSpec::mc_deserialize(data)?.map(move |body| EntityDead(body))),
            other => Err(DeserializeErr::CannotUnderstandValue(format!("invalid combat event id {:?}", other)))
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PlayerInfoAction<A: Clone + PartialEq + Debug> {
    pub uuid: UUID4,
    pub action: A
}

impl<A> Serialize for PlayerInfoAction<A> where A: Serialize + Clone + PartialEq + Debug {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_other(&self.uuid)?;
        to.serialize_other(&self.action)
    }
}

impl<A> Deserialize for PlayerInfoAction<A> where A: Deserialize + Clone + PartialEq + Debug {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: uuid, data } = UUID4::mc_deserialize(data)?;
        Ok(A::mc_deserialize(data)?.map(move |action| Self { uuid, action }))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum PlayerInfoActionList {
    Add(Vec<PlayerInfoAction<PlayerAddActionSpec>>),
    UpdateGameMode(Vec<PlayerInfoAction<GameMode>>),
    UpdateLatency(Vec<PlayerInfoAction<VarInt>>),
    UpdateDisplayName(Vec<PlayerInfoAction<Option<Chat>>>),
    Remove(Vec<UUID4>)
}

__protocol_body_def_helper!(PlayerAddActionSpec {
    name: String,
    properties: VarIntCountedArray<PlayerAddProperty>,
    game_mode: GameMode,
    ping_ms: VarInt,
    display_name: Option<Chat>
});

__protocol_body_def_helper!(PlayerAddProperty {
    name: String,
    value: String,
    signature: Option<String>
});

impl PlayerInfoActionList {

    pub fn player_ids(&self) -> Vec<UUID4> {
        use PlayerInfoActionList::*;

        match self {
            Add(vec) => vec.iter().map(move |v| v.uuid).collect(),
            UpdateGameMode(vec) => vec.iter().map(move |v| v.uuid).collect(),
            UpdateLatency(vec) => vec.iter().map(move |v| v.uuid).collect(),
            UpdateDisplayName(vec) => vec.iter().map(move |v| v.uuid).collect(),
            Remove(vec) => vec.clone()
        }
    }

    pub fn id(&self) -> VarInt {
        use PlayerInfoActionList::*;

        match self {
            Add(_) => 0x00,
            UpdateGameMode(_) => 0x01,
            UpdateLatency(_) => 0x02,
            UpdateDisplayName(_) => 0x03,
            Remove(_) => 0x04
        }.into()
    }

    pub fn len(&self) -> usize {
        use PlayerInfoActionList::*;

        match self {
            Add(vec) => vec.len(),
            UpdateGameMode(vec) => vec.len(),
            UpdateLatency(vec) => vec.len(),
            UpdateDisplayName(vec) => vec.len(),
            Remove(vec) => vec.len()
        }
    }
}

impl Serialize for PlayerInfoActionList {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let id = self.id();
        to.serialize_other(&id)?;

        let len = VarInt(self.len() as i32);
        to.serialize_other(&len)?;

        use PlayerInfoActionList::*;

        match self {
            Add(body) => serialize_vec_directly(body, to),
            UpdateGameMode(body) => serialize_vec_directly(body, to),
            UpdateLatency(body) => serialize_vec_directly(body, to),
            UpdateDisplayName(body) => serialize_vec_directly(body, to),
            Remove(body) => serialize_vec_directly(body, to),
        }
    }
}

impl Deserialize for PlayerInfoActionList {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: action_id, data } = VarInt::mc_deserialize(data)?;
        let Deserialized { value: raw_count, mut data } = VarInt::mc_deserialize(data)?;

        let count = raw_count.0 as usize;

        use PlayerInfoActionList::*;
        match action_id.0 {
            0x00 => Ok(deserialize_vec_directly(count, &mut data)?.map(move |v| Add(v))),
            0x01 => Ok(deserialize_vec_directly(count, &mut data)?.map(move |v| UpdateGameMode(v))),
            0x02 => Ok(deserialize_vec_directly(count, &mut data)?.map(move |v| UpdateLatency(v))),
            0x03 => Ok(deserialize_vec_directly(count, &mut data)?.map(move |v| UpdateDisplayName(v))),
            0x04 => Ok(deserialize_vec_directly(count, &mut data)?.map(move |v| Remove(v))),
            other => Err(DeserializeErr::CannotUnderstandValue(format!("invalid player info action id {}", other))),
        }
    }
}

fn serialize_vec_directly<I: Serialize, S: Serializer>(items: &Vec<I>, to: &mut S) -> SerializeResult {
    for item in items {
        to.serialize_other(item)?;
    }

    Ok(())
}

fn deserialize_vec_directly<I: Deserialize>(count: usize, mut data: &[u8]) -> DeserializeResult<Vec<I>> {
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let Deserialized { value: item, data: rest } = I::mc_deserialize(data)?;
        data = rest;
        out.push(item);
    }

    Deserialized::ok(out, data)
}

proto_varint_enum!(FacePlayerKind,
    0x00 :: Feet,
    0x01 :: Eyes
);

__protocol_body_def_helper!(FacePlayerEntityTarget {
    entity_id: VarInt,
    kind: FacePlayerKind
});

proto_byte_flag!(PositionAndLookFlags,
    0x01 :: x,
    0x02 :: y,
    0x04 :: z,
    0x08 :: y_rotation,
    0x10 :: x_rotation
);

proto_byte_enum!(EntityEffectKind,
    0x01 :: Speed,
    0x02 :: Slowness,
    0x03 :: Haste,
    0x04 :: MiningFatigue,
    0x05 :: Strength,
    0x06 :: InstantHealth,
    0x07 :: InstantDamage,
    0x08 :: JumpBoost,
    0x09 :: Nausea,
    0x0A :: Regeneration,
    0x0B :: Resistance,
    0x0C :: FireResistance,
    0x0D :: WaterBreathing,
    0x0E :: Invisibility,
    0x0F :: Blindness,
    0x10 :: NightVision,
    0x11 :: Hunger,
    0x12 :: Weakness,
    0x13 :: Poison,
    0x14 :: Wither,
    0x15 :: HealthBoost,
    0x16 :: Absorption,
    0x17 :: Saturation,
    0x18 :: Glowing,
    0x19 :: Levetation,
    0x1A :: Luck,
    0x1B :: Unluck,
    0x1C :: SlowFalling,
    0x1D :: ConduitPower,
    0x1E :: DolphinsGrace,
    0x1F :: BadOmen,
    0x20 :: HeroOfTheVillage
);

#[derive(Clone, PartialEq, Debug)]
pub enum WorldBorderAction {
    SetSize(WorldBorderSetSizeSpec),
    LerpSize(WorldBorderLerpSizeSpec),
    SetCenter(WorldBorderSetCenterSpec),
    Initialize(WorldBorderInitiaializeSpec),
    SetWarningTime(WorldBorderWarningTimeSpec),
    SetWarningBlocks(WorldBorderWarningBlocksSpec)
}

__protocol_body_def_helper!(WorldBorderSetSizeSpec {
    diameter: f64
});

__protocol_body_def_helper!(WorldBorderLerpSizeSpec {
    old_diameter: f64,
    new_diameter: f64,
    speed: VarLong
});

__protocol_body_def_helper!(WorldBorderSetCenterSpec {
    x: f64,
    z: f64
});

__protocol_body_def_helper!(WorldBorderInitiaializeSpec {
    x: f64,
    z: f64,
    old_diameter: f64,
    new_diameter: f64,
    speed: VarLong,
    portal_teleport_boundary: VarLong,
    warning_time: VarInt,
    warning_blocks: VarInt
});

__protocol_body_def_helper!(WorldBorderWarningTimeSpec {
    warning_time: VarInt
});

__protocol_body_def_helper!(WorldBorderWarningBlocksSpec {
    warning_blocks: VarInt
});

impl WorldBorderAction {
    pub fn id(&self) -> VarInt {
        use WorldBorderAction::*;
        match self {
            SetSize(_) => 0x00,
            LerpSize(_) => 0x01,
            SetCenter(_) => 0x02,
            Initialize(_) => 0x03,
            SetWarningTime(_) => 0x04,
            SetWarningBlocks(_) => 0x05,
        }.into()
    }
}

impl Serialize for WorldBorderAction {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let id = self.id();
        to.serialize_other(&id)?;

        use WorldBorderAction::*;
        match self {
            SetSize(body) => to.serialize_other(body),
            LerpSize(body) => to.serialize_other(body),
            SetCenter(body) => to.serialize_other(body),
            Initialize(body) => to.serialize_other(body),
            SetWarningTime(body) => to.serialize_other(body),
            SetWarningBlocks(body) => to.serialize_other(body),
        }
    }
}

impl Deserialize for WorldBorderAction {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized{ value: id, data } = VarInt::mc_deserialize(data)?;

        use WorldBorderAction::*;
        match id.0 {
            0x00 => Ok(WorldBorderSetSizeSpec::mc_deserialize(data)?.map(move |body| SetSize(body))),
            0x01 => Ok(WorldBorderLerpSizeSpec::mc_deserialize(data)?.map(move |body| LerpSize(body))),
            0x02 => Ok(WorldBorderSetCenterSpec::mc_deserialize(data)?.map(move |body| SetCenter(body))),
            0x03 => Ok(WorldBorderInitiaializeSpec::mc_deserialize(data)?.map(move |body| Initialize(body))),
            0x04 => Ok(WorldBorderWarningTimeSpec::mc_deserialize(data)?.map(move |body| SetWarningTime(body))),
            0x05 => Ok(WorldBorderWarningBlocksSpec::mc_deserialize(data)?.map(move |body| SetWarningBlocks(body))),
            other => Err(DeserializeErr::CannotUnderstandValue(format!("invalid world border action id {}", other)))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum ScoreboardPosition {
    List,
    Sidebar,
    BelowName,
    TeamSpecific(i8)
}

impl ScoreboardPosition {
    pub fn id(&self) -> i8 {
        use ScoreboardPosition::*;
        match self {
            List => 0x00,
            Sidebar => 0x01,
            BelowName => 0x02,
            TeamSpecific(team_id) => 0x03 + team_id
        }
    }
}

impl Serialize for ScoreboardPosition {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_byte(self.id() as u8)
    }
}

impl Deserialize for ScoreboardPosition {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: id, data } = i8::mc_deserialize(data)?;
        use ScoreboardPosition::*;
        let res = match id {
            0x00 => Ok(List),
            0x01 => Ok(Sidebar),
            0x02 => Ok(BelowName),
            other => {
                if other >= 3 && other <= 12 {
                    Ok(TeamSpecific(other - 0x03))
                } else {
                    Err(DeserializeErr::CannotUnderstandValue(format!("invalid scoreboard position id {}", id)))
                }
            }
        }?;
        Deserialized::ok(res, data)
    }
}

proto_varint_enum!(EquipmentSlot,
    0x00 :: MainHand,
    0x01 :: OffHand,
    0x02 :: ArmorBoots,
    0x03 :: ArmorLeggings,
    0x04 :: ArmorChestplate,
    0x05 :: ArmorHelmet
);

#[derive(Clone, PartialEq, Debug)]
pub enum ScoreboardObjectiveAction {
    Create(ScoreboardObjectiveSpec),
    Remove,
    UpdateText(ScoreboardObjectiveSpec)
}

proto_varint_enum!(ScoreboardObjectiveKind,
    0x00 :: Integer,
    0x01 :: Hearts
);

__protocol_body_def_helper!(ScoreboardObjectiveSpec {
    text: Chat,
    kind: ScoreboardObjectiveKind
});

impl ScoreboardObjectiveAction {
    pub fn id(&self) -> i8 {
        use ScoreboardObjectiveAction::*;
        match self {
            Create(_) => 0x00,
            Remove => 0x01,
            UpdateText(_) => 0x02
        }
    }
}

impl Serialize for ScoreboardObjectiveAction {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let id = self.id();
        to.serialize_other(&id)?;

        use ScoreboardObjectiveAction::*;
        match self {
            Create(body) => to.serialize_other(body),
            UpdateText(body) => to.serialize_other(body),
            _ => Ok(())
        }
    }
}

impl Deserialize for ScoreboardObjectiveAction {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized{ value: id, data } = i8::mc_deserialize(data)?;
        use ScoreboardObjectiveAction::*;
        match id {
            0x00 => Ok(ScoreboardObjectiveSpec::mc_deserialize(data)?.map(move |body| Create(body))),
            0x01 => Deserialized::ok(Remove, data),
            0x02 => Ok(ScoreboardObjectiveSpec::mc_deserialize(data)?.map(move |body| UpdateText(body))),
            other => Err(DeserializeErr::CannotUnderstandValue(format!("invalid scoreboard objective action id {}", other)))
        }
    }
}

__protocol_body_def_helper!(EntityPropertySpec {
    key: String,
    value: f64,
    modifiers: VarIntCountedArray<EntityPropertyModifierSpec>
});

__protocol_body_def_helper!(EntityPropertyModifierSpec {
    uuid: UUID4,
    amount: f64,
    operation: EntityPropertyModifierOperation
});

proto_byte_enum!(EntityPropertyModifierOperation,
    0x00 :: AddSubtractAmount,
    0x01 :: AddSubtractAmountPercentOfCurrent,
    0x02 :: MultiplyByAmountPercent
);

proto_byte_flag!(EntityEffectFlags,
    0x01 :: ambient,
    0x02 :: show_particles,
    0x04 :: show_icon
);

__protocol_body_def_helper!(TagSpec {
    name: String,
    entries: VarIntCountedArray<VarInt>
});

proto_varint_enum!(ClientStatusAction,
    0x00 :: PerformRespawn,
    0x01 :: RequestStats
);

proto_varint_enum!(ClientChatMode,
    0x00 :: Enabled,
    0x01 :: CommandsOnly,
    0x02 :: Hidden
);

proto_varint_enum!(ClientMainHand,
    0x00 :: Left,
    0x01 :: Right
);

proto_byte_flag!(ClientDisplayedSkinParts,
    0x01 :: cape_enabled,
    0x02 :: jacket_enabled,
    0x04 :: left_sleeve_enabled,
    0x08 :: right_sleeve_enabled,
    0x10 :: left_pants_leg_enabled,
    0x20 :: right_pant_legs_enabled,
    0x40 :: hat_enabled
);

proto_varint_enum!(InventoryOperationMode,
    0x00 :: MouseClick,
    0x01 :: ShiftClick,
    0x02 :: NumberClick,
    0x03 :: MiddleClick,
    0x04 :: DropClick,
    0x05 :: Drag,
    0x06 :: DoubleClick
);

__protocol_body_def_helper!(InteractAtSpec {
    target_x: f32,
    target_y: f32,
    target_z: f32,
    hand: Hand
});

#[derive(Clone, PartialEq, Debug)]
pub enum InteractKind {
    Interact,
    Attack,
    InteractAt(InteractAtSpec)
}

impl InteractKind {
    pub fn id(&self) -> VarInt {
        use InteractKind::*;
        match self {
            Interact => 0x00,
            Attack => 0x01,
            InteractAt(_) => 0x02,
        }.into()
    }
}

impl Serialize for InteractKind {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let id = self.id();
        to.serialize_other(&id)?;

        use InteractKind::*;
        match self {
            InteractAt(body) => to.serialize_other(body),
            _ => Ok(())
        }
    }
}

impl Deserialize for InteractKind {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized{ value: id, data } = VarInt::mc_deserialize(data)?;

        use InteractKind::*;
        match id.0 {
            0x00 => Deserialized::ok(Interact, data),
            0x01 => Deserialized::ok(Attack, data),
            0x02 => Ok(InteractAtSpec::mc_deserialize(data)?.map(move |body| InteractAt(body))),
            other => Err(DeserializeErr::CannotUnderstandValue(format!("invalid entity interact kind id {}", other)))
        }
    }
}

proto_byte_flag!(ClientPlayerAbilities,
    0x01 :: creative,
    0x02 :: flying,
    0x04 :: fly_enabled,
    0x08 :: damaged_disabled
);

proto_varint_enum!(PlayerDiggingStatus,
    0x00 :: Started,
    0x01 :: Cancelled,
    0x02 :: Finished,
    0x03 :: DropStack,
    0x04 :: DropItem,
    0x05 :: ShootArrowOrFishEating,
    0x06 :: SwapItemInHand
);

proto_byte_enum!(DiggingFace,
    0x00 :: Bottom,
    0x01 :: Top,
    0x02 :: North,
    0x03 :: South,
    0x04 :: West,
    0x05 :: East
);

proto_varint_enum!(EntityActionKind,
    0x00 :: StartSneaking,
    0x01 :: StopSneaking,
    0x02 :: LeaveBed,
    0x03 :: StartSprinting,
    0x04 :: StopSprinting,
    0x05 :: StartJumpWithHorse,
    0x06 :: StopJumpWithHorse,
    0x07 :: OpenHorseInventory,
    0x08 :: StartFlyingWithElytra
);

proto_byte_flag!(SteerVehicleFlags,
    0x01 :: jump,
    0x02 :: unmount
);

proto_varint_enum!(ResourcePackStatus,
    0x00 :: Loaded,
    0x01 :: Declined,
    0x02 :: FailedDownload,
    0x03 :: Accepted
);

proto_varint_enum!(CommandBlockMode,
    0x00 :: Sequence,
    0x01 :: Auto,
    0x02 :: Redstone
);

proto_byte_flag!(CommandBlockFlags,
    0x01 :: track_output,
    0x02 :: conditional,
    0x04 :: automatic
);

proto_varint_enum!(UpdateStructureBlockAction,
    0x00 :: UpdateData,
    0x01 :: SaveStructure,
    0x02 :: LoadStructure,
    0x03 :: DetectSize
);

proto_varint_enum!(UpdateStructureBlockMode,
    0x00 :: Save,
    0x01 :: Load,
    0x02 :: Corner,
    0x03 :: Data
);

proto_varint_enum!(UpdateStructureBlockMirror,
    0x00 :: NoMirror,
    0x01 :: LeftRight,
    0x02 :: FrontBack
);

proto_varint_enum!(UpdateStructureBlockRotation,
    0x00 :: NoRotation,
    0x01 :: Clockwise90,
    0x02 :: Clockwise180,
    0x03 :: CounterClockwise90
);

proto_byte_flag!(UpdateStructureBlockFlags,
    0x01 :: ignore_entities,
    0x02 :: show_air,
    0x04 :: show_bounding_box
);

#[derive(Clone, PartialEq, Debug)]
pub struct RecipeSpec {
    pub recipe: Recipe,
    pub id: String
}

#[derive(Clone, PartialEq, Debug)]
pub enum Recipe {
    CraftingShapeless(RecipeCraftingShapelessSpec),
    CraftingShaped(RecipeCraftingShapedSpec),
    CraftingArmorDye,
    CraftingBookCloning,
    CraftingMapCloning,
    CraftingMapExtending,
    CraftingFireworkRocket,
    CraftingFireworkStar,
    CraftingFireworkStarFade,
    CraftingRepairItem,
    CraftingTippedArrow,
    CraftingBannerDuplicate,
    CraftingBannerAddPattern,
    CraftingShieldDecoration,
    CraftingShulkerBoxColoring,
    CraftingSuspiciousStew,
    Smelting(RecipeSmeltingSpec),
    Blasting(RecipeSmeltingSpec),
    Smoking(RecipeSmeltingSpec),
    CampfireCooking(RecipeSmeltingSpec),
    StoneCutting(RecipeStonecuttingSpec)
}

impl Recipe {
    pub fn id(&self) -> String {
        use Recipe::*;
        match self {
            CraftingShapeless(_) => "minecraft:crafting_shapeless",
            CraftingShaped(_) => "minecraft:crafting_shaped",
            CraftingArmorDye => "minecraft:crafting_special_armordye",
            CraftingBookCloning => "minecraft:crafting_special_bookcloning",
            CraftingMapCloning => "minecraft:crafting_special_mapcloning",
            CraftingMapExtending => "minecraft:crafting_special_mapextending",
            CraftingFireworkRocket => "minecraft:crafting_special_firework_rocket",
            CraftingFireworkStar => "minecraft:crafting_special_firework_star",
            CraftingFireworkStarFade => "minecraft:crafting_special_firework_star_fade",
            CraftingRepairItem => "minecraft:crafting_special_repairitem",
            CraftingTippedArrow => "minecraft:crafting_special_tippedarrow",
            CraftingBannerDuplicate => "minecraft:crafting_special_bannerduplicate",
            CraftingBannerAddPattern => "minecraft:crafting_special_banneraddpattern",
            CraftingShieldDecoration => "minecraft:crafting_special_shielddecoration",
            CraftingShulkerBoxColoring => "minecraft:crafting_special_shulkerboxcoloring",
            CraftingSuspiciousStew => "minecraft:crafting_special_suspiciousstew",
            Smelting(_) => "minecraft:smelting",
            Blasting(_) => "minecraft:blasting",
            Smoking(_) => "minecraft:smoking",
            CampfireCooking(_) => "minecraft:campfire_cooking",
            StoneCutting(_) => "minecraft:stonecutting",
        }.to_owned()
    }

    fn serialize_body<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        use Recipe::*;
        match self {
            CraftingShapeless(body) => to.serialize_other(body),
            CraftingShaped(body) => to.serialize_other(body),
            Smelting(body) => to.serialize_other(body),
            Blasting(body) => to.serialize_other(body),
            Smoking(body) => to.serialize_other(body),
            CampfireCooking(body) => to.serialize_other(body),
            StoneCutting(body) => to.serialize_other(body),
            _ => Ok(())
        }
    }
}

impl Serialize for RecipeSpec {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        let _type = self.recipe.id();
        to.serialize_other(&_type)?;
        to.serialize_other(&self.id)?;
        self.recipe.serialize_body(to)
    }
}

impl Deserialize for RecipeSpec {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: _type, data } = String::mc_deserialize(data)?;
        let Deserialized { value: recipe_id, data } = String::mc_deserialize(data)?;

        use Recipe::*;
        Ok(match _type.as_str() {
            "minecraft:crafting_shapeless" => Ok(RecipeCraftingShapelessSpec::mc_deserialize(data)?.map(move |b| CraftingShapeless(b))),
            "minecraft:crafting_shaped" => Ok(RecipeCraftingShapedSpec::mc_deserialize(data)?.map(move |b| CraftingShaped(b))),
            "minecraft:crafting_special_armordye" => Deserialized::ok(CraftingArmorDye, data),
            "minecraft:crafting_special_bookcloning" => Deserialized::ok(CraftingBookCloning, data),
            "minecraft:crafting_special_mapcloning" => Deserialized::ok(CraftingMapCloning, data),
            "minecraft:crafting_special_mapextending" => Deserialized::ok(CraftingMapExtending, data),
            "minecraft:crafting_special_firework_rocket" => Deserialized::ok(CraftingFireworkRocket, data),
            "minecraft:crafting_special_firework_star" => Deserialized::ok(CraftingFireworkStar, data),
            "minecraft:crafting_special_firework_star_fade" => Deserialized::ok(CraftingFireworkStarFade, data),
            "minecraft:crafting_special_repairitem" => Deserialized::ok(CraftingRepairItem, data),
            "minecraft:crafting_special_tippedarrow" => Deserialized::ok(CraftingTippedArrow, data),
            "minecraft:crafting_special_bannerduplicate" => Deserialized::ok(CraftingBannerDuplicate, data),
            "minecraft:crafting_special_banneraddpattern" => Deserialized::ok(CraftingBannerAddPattern, data),
            "minecraft:crafting_special_shielddecoration" => Deserialized::ok(CraftingShieldDecoration, data),
            "minecraft:crafting_special_shulkerboxcoloring" => Deserialized::ok(CraftingShulkerBoxColoring, data),
            "minecraft:crafting_special_suspiciousstew" => Deserialized::ok(CraftingSuspiciousStew, data),
            "minecraft:smelting" => Ok(RecipeSmeltingSpec::mc_deserialize(data)?.map(move |b| Smelting(b))),
            "minecraft:blasting" => Ok(RecipeSmeltingSpec::mc_deserialize(data)?.map(move |b| Blasting(b))),
            "minecraft:smoking" => Ok(RecipeSmeltingSpec::mc_deserialize(data)?.map(move |b| Smoking(b))),
            "minecraft:campfire_cooking" => Ok(RecipeSmeltingSpec::mc_deserialize(data)?.map(move |b| CampfireCooking(b))),
            "minecraft:stonecutting" => Ok(RecipeStonecuttingSpec::mc_deserialize(data)?.map(move |b| StoneCutting(b))),
            other => Err(DeserializeErr::CannotUnderstandValue(format!("invalid crafting recipe kind {:?}", other)))
        }?.map(move |recipe_body| {
            RecipeSpec{ id: recipe_id, recipe: recipe_body }
        }))
    }
}

__protocol_body_def_helper!(RecipeIngredient {
    items: VarIntCountedArray<Option<Slot>>
});

__protocol_body_def_helper!(RecipeCraftingShapelessSpec {
    group: String,
    ingredients: VarIntCountedArray<RecipeIngredient>,
    result: Option<Slot>
});

#[derive(Debug, Clone, PartialEq)]
pub struct RecipeCraftingShapedSpec {
    pub width: VarInt,
    pub height: VarInt,
    pub group: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub result: Option<Slot>,
}

impl Serialize for RecipeCraftingShapedSpec {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_other(&self.width)?;
        to.serialize_other(&self.height)?;
        to.serialize_other(&self.group)?;
        for elem in &self.ingredients {
            to.serialize_other(elem)?;
        }
        to.serialize_other(&self.result)?;
        Ok(())
    }
}
impl Deserialize for RecipeCraftingShapedSpec {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: width, data } = <VarInt>::mc_deserialize(data)?;
        let Deserialized { value: height, data } = <VarInt>::mc_deserialize(data)?;
        let Deserialized { value: group, mut data } = <String>::mc_deserialize(data)?;

        let ingredients_count = width.0 as usize * height.0 as usize;
        let mut ingredients: Vec<RecipeIngredient> = Vec::with_capacity(ingredients_count);
        for _ in 0..ingredients_count {
            let Deserialized { value: elem, data: rest } = RecipeIngredient::mc_deserialize(data)?;
            data = rest;
            ingredients.push(elem);
        }

        let Deserialized { value: result, data } = <Option<Slot>>::mc_deserialize(data)?;

        Deserialized::ok(Self { width, height, group, ingredients, result }, data)
    }
}

__protocol_body_def_helper!(RecipeSmeltingSpec {
    group: String,
    ingredient: RecipeIngredient,
    result: Option<Slot>,
    experience: f32,
    cooking_time: VarInt
});

__protocol_body_def_helper!(RecipeStonecuttingSpec {
    group: String,
    ingredient: RecipeIngredient,
    result: Option<Slot>
});

proto_varint_enum!(RecipeUnlockAction,
    0x00 :: Init,
    0x01 :: Add,
    0x02 :: Remove
);

#[derive(Clone, PartialEq, Debug)]
pub struct ChunkData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub primary_bit_mask: VarInt,
    pub heightmaps: NamedNbtTag,
    pub biomes: Option<[i32; 1024]>,
    pub data: VarIntCountedArray<u8>,
    pub block_entities: Vec<NamedNbtTag>
}

impl Serialize for ChunkData {
    fn mc_serialize<S: Serializer>(&self, to: &mut S) -> SerializeResult {
        to.serialize_other(&self.chunk_x)?;
        to.serialize_other(&self.chunk_z)?;
        let full_chunk = self.biomes.is_some();
        to.serialize_other(&full_chunk)?;
        to.serialize_other(&self.primary_bit_mask)?;
        to.serialize_other(&self.heightmaps)?;

        if full_chunk {
            let biomes = self.biomes.as_ref().unwrap();
            for elem in biomes {
                to.serialize_other(elem)?;
            }
        }

        to.serialize_other(&self.data)?;
        let num_block_entities = VarInt(self.block_entities.len() as i32);
        to.serialize_other(&num_block_entities)?;
        for entity in &self.block_entities {
            to.serialize_other(entity)?;
        }

        Ok(())
    }
}

impl Deserialize for ChunkData {
    fn mc_deserialize(data: &[u8]) -> DeserializeResult<'_, Self> {
        let Deserialized { value: chunk_x, data } = i32::mc_deserialize(data)?;
        let Deserialized { value: chunk_z, data } = i32::mc_deserialize(data)?;
        let Deserialized { value: is_full_chunk, data } = bool::mc_deserialize(data)?;
        let Deserialized { value: primary_bit_mask, data } = VarInt::mc_deserialize(data)?;
        let Deserialized { value: heightmaps, mut data } = NamedNbtTag::mc_deserialize(data)?;
        let biomes = if is_full_chunk {
            let mut biomes: [i32; 1024] = [0i32; 1024];
            for elem in &mut biomes {
                let Deserialized { value, data: rest } = i32::mc_deserialize(data)?;
                data = rest;
                *elem = value;
            }
            Some(biomes)
        } else {
            None
        };
        let Deserialized { value: chunk_data, data } = VarIntCountedArray::<u8>::mc_deserialize(data)?;
        let Deserialized { value: n_block_entities_raw, mut data } = VarInt::mc_deserialize(data)?;
        let n_block_entities = n_block_entities_raw.0 as usize;
        let mut block_entities = Vec::with_capacity(n_block_entities);
        for _ in 0..n_block_entities {
            let Deserialized { value: entity, data: rest } = NamedNbtTag::mc_deserialize(data)?;
            data = rest;
            block_entities.push(entity);
        }

        Deserialized::ok(ChunkData{
            chunk_x,
            chunk_z,
            primary_bit_mask,
            heightmaps,
            biomes,
            data: chunk_data,
            block_entities,
        }, data)
    }
}