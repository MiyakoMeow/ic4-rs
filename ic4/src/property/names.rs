use crate::*;

/*
 * PropertyName
 */

#[derive(Debug, Clone, Copy)]
pub enum PropertyName {
    AcquisitionBurstFrameCount,
    AcquisitionBurstInterval,
    AcquisitionFrameRate,
    AcquisitionMode,
    AcquisitionStart,
    AcquisitionStop,
    ActionDeviceKey,
    ActionGroupKey,
    ActionGroupMask,
    ActionQueueSize,
    ActionSchedulerCancel,
    ActionSchedulerCommit,
    ActionSchedulerInterval,
    ActionSchedulerStatus,
    ActionSchedulerTime,
    ActionSelector,
    AutoFocusRoiEnable,
    AutoFocusRoiHeight,
    AutoFocusRoiLeft,
    AutoFocusRoiTop,
    AutoFocusRoiWidth,
    AutoFunctionsRoiEnable,
    AutoFunctionsRoiHeight,
    AutoFunctionsRoiLeft,
    AutoFunctionsRoiPreset,
    AutoFunctionsRoiTop,
    AutoFunctionsRoiWidth,
    BalanceRatio,
    BalanceRatioSelector,
    BalanceWhiteAuto,
    BalanceWhiteAutoPreset,
    BalanceWhiteMode,
    BalanceWhiteTemperature,
    BalanceWhiteTemperaturePreset,
    BinningHorizontal,
    BinningVertical,
    BlackLevel,
    ChunkBlockId,
    ChunkEnable,
    ChunkExposureTime,
    ChunkGain,
    ChunkImage,
    ChunkImx174FrameId,
    ChunkImx174FrameSet,
    ChunkModeActive,
    ChunkMultiFrameSetFrameId,
    ChunkMultiFrameSetId,
    ChunkSelector,
    ColorTransformationEnable,
    ColorTransformationSelector,
    ColorTransformationValue,
    ColorTransformationValueSelector,
    Contrast,
    DecimationHorizontal,
    DecimationVertical,
    Denoise,
    DeviceEventChannelCount,
    DeviceFactoryReset,
    DeviceFirmwareVersion,
    DeviceLinkHeartbeatTimeout,
    DeviceLinkSelector,
    DeviceModelName,
    DeviceReset,
    DeviceScanType,
    DeviceSerialNumber,
    DeviceSfncVersionMajor,
    DeviceSfncVersionMinor,
    DeviceSfncVersionSubMinor,
    DeviceStreamChannelCount,
    DeviceStreamChannelEndianness,
    DeviceStreamChannelLink,
    DeviceStreamChannelPacketSize,
    DeviceStreamChannelSelector,
    DeviceStreamChannelType,
    DeviceTemperature,
    DeviceTemperatureSelector,
    DeviceTLType,
    DeviceTLVersionMajor,
    DeviceTLVersionMinor,
    DeviceTLVersionSubMinor,
    DeviceType,
    DeviceUserId,
    DeviceVendorName,
    DeviceVersion,
    DisableInfoOverlay,
    EventExposureEnd,
    EventExposureEndFrameId,
    EventExposureEndTimestamp,
    EventFocusMoveCompleted,
    EventFocusMoveCompletedFocus,
    EventFocusMoveCompletedTimestamp,
    EventFrameTriggerMissed,
    EventFrameTriggerMissedTimestamp,
    EventLine1FallingEdge,
    EventLine1FallingEdgeTimestamp,
    EventLine1RisingEdge,
    EventLine1RisingEdgeTimestamp,
    EventNotification,
    EventSelector,
    EventTest,
    EventTestTimestamp,
    EventZoomMoveCompleted,
    EventZoomMoveCompletedTimestamp,
    EventZoomMoveCompletedZoom,
    ExpandOutputRange,
    ExposureAuto,
    ExposureAutoHighlightReduction,
    ExposureAutoHighlighReduction,
    ExposureAutoLowerLimit,
    ExposureAutoReference,
    ExposureAutoUpperLimit,
    ExposureAutoUpperLimitAuto,
    ExposureTime,
    FileAccessBuffer,
    FileAccessLength,
    FileAccessOffset,
    FileOpenMode,
    FileOperationExecute,
    FileOperationResult,
    FileOperationSelector,
    FileOperationStatus,
    FileSelector,
    FileSize,
    Focus,
    FocusAuto,
    Gain,
    GainAuto,
    GainAutoLowerLimit,
    GainAutoUpperLimit,
    GainMode,
    Gamma,
    GevGvspExtendedIdMode,
    GevScpsDoNotFragment,
    GevScpsPacketSize,
    GpIn,
    GpOut,
    Height,
    HeightMax,
    Hue,
    Imx174HardwareWdrEnable,
    Imx174HardwareWdrShutterMode,
    Imx174WdrShutter2,
    ImxLowLatencyTriggerMode,
    InputBits,
    InputFp1ks,
    InputHeight,
    InputWidth,
    Iris,
    IrisAuto,
    IrCutFilterEnable,
    LutEnable,
    LutIndex,
    LutSelector,
    LutValue,
    LutValueAll,
    MultiFrameSetOutputModeCustomGain,
    MultiFrameSetOutputModeEnable,
    MultiFrameSetOutputModeExposureTime0,
    MultiFrameSetOutputModeExposureTime1,
    MultiFrameSetOutputModeExposureTime2,
    MultiFrameSetOutputModeExposureTime3,
    MultiFrameSetOutputModeFrameCount,
    MultiFrameSetOutputModeGain0,
    MultiFrameSetOutputModeGain1,
    MultiFrameSetOutputModeGain2,
    MultiFrameSetOutputModeGain3,
    OffsetAutoCenter,
    OffsetX,
    OffsetY,
    PayloadSize,
    PixelFormat,
    PtpClockAccuracy,
    PtpEnable,
    PtpStatus,
    ReverseX,
    ReverseY,
    Saturation,
    SensorHeight,
    SensorPixelHeight,
    SensorPixelWidth,
    SensorWidth,
    Sharpness,
    SidebandUse,
    SignalDetected,
    SoftwareTransformEnable,
    SourceConnected,
    StrobeDelay,
    StrobeDuration,
    StrobeEnable,
    StrobeOperation,
    StrobePolarity,
    TestEventGenerate,
    TestPendingAck,
    TimestampLatch,
    TimestampLatchString,
    TimestampLatchValue,
    TimestampReset,
    TLParamsLocked,
    ToneMappingEnable,
    ToneMappingGlobalBrightness,
    ToneMappingIntensity,
    TriggerActivation,
    TriggerDebouncer,
    TriggerDelay,
    TriggerDenoise,
    TriggerMask,
    TriggerMode,
    TriggerOperation,
    TriggerOverlap,
    TriggerSelector,
    TriggerSoftware,
    TriggerSource,
    UserSetDefault,
    UserSetLoad,
    UserSetSave,
    UserSetSelector,
    Width,
    WidthMax,
    Zoom,
}

impl Default for PropertyName {
    fn default() -> Self {
        Self::Contrast
    }
}

impl PropertyName {
    fn as_bytes(&self) -> &[u8] {
        use PropertyName::*;
        match self {
            AcquisitionBurstFrameCount => &ic4_sys::IC4_PROPID_ACQUISITION_BURST_FRAME_COUNT[..],
            AcquisitionBurstInterval => &ic4_sys::IC4_PROPID_ACQUISITION_BURST_INTERVAL[..],
            AcquisitionFrameRate => &ic4_sys::IC4_PROPID_ACQUISITION_FRAME_RATE[..],
            AcquisitionMode => &ic4_sys::IC4_PROPID_ACQUISITION_MODE[..],
            AcquisitionStart => &ic4_sys::IC4_PROPID_ACQUISITION_START[..],
            AcquisitionStop => &ic4_sys::IC4_PROPID_ACQUISITION_STOP[..],
            ActionDeviceKey => &ic4_sys::IC4_PROPID_ACTION_DEVICE_KEY[..],
            ActionGroupKey => &ic4_sys::IC4_PROPID_ACTION_GROUP_KEY[..],
            ActionGroupMask => &ic4_sys::IC4_PROPID_ACTION_GROUP_MASK[..],
            ActionQueueSize => &ic4_sys::IC4_PROPID_ACTION_QUEUE_SIZE[..],
            ActionSchedulerCancel => &ic4_sys::IC4_PROPID_ACTION_SCHEDULER_CANCEL[..],
            ActionSchedulerCommit => &ic4_sys::IC4_PROPID_ACTION_SCHEDULER_COMMIT[..],
            ActionSchedulerInterval => &ic4_sys::IC4_PROPID_ACTION_SCHEDULER_INTERVAL[..],
            ActionSchedulerStatus => &ic4_sys::IC4_PROPID_ACTION_SCHEDULER_STATUS[..],
            ActionSchedulerTime => &ic4_sys::IC4_PROPID_ACTION_SCHEDULER_TIME[..],
            ActionSelector => &ic4_sys::IC4_PROPID_ACTION_SELECTOR[..],
            AutoFocusRoiEnable => &ic4_sys::IC4_PROPID_AUTO_FOCUS_ROI_ENABLE[..],
            AutoFocusRoiHeight => &ic4_sys::IC4_PROPID_AUTO_FOCUS_ROI_HEIGHT[..],
            AutoFocusRoiLeft => &ic4_sys::IC4_PROPID_AUTO_FOCUS_ROI_LEFT[..],
            AutoFocusRoiTop => &ic4_sys::IC4_PROPID_AUTO_FOCUS_ROI_TOP[..],
            AutoFocusRoiWidth => &ic4_sys::IC4_PROPID_AUTO_FOCUS_ROI_WIDTH[..],
            AutoFunctionsRoiEnable => &ic4_sys::IC4_PROPID_AUTO_FUNCTIONS_ROI_ENABLE[..],
            AutoFunctionsRoiHeight => &ic4_sys::IC4_PROPID_AUTO_FUNCTIONS_ROI_HEIGHT[..],
            AutoFunctionsRoiLeft => &ic4_sys::IC4_PROPID_AUTO_FUNCTIONS_ROI_LEFT[..],
            AutoFunctionsRoiPreset => &ic4_sys::IC4_PROPID_AUTO_FUNCTIONS_ROI_PRESET[..],
            AutoFunctionsRoiTop => &ic4_sys::IC4_PROPID_AUTO_FUNCTIONS_ROI_TOP[..],
            AutoFunctionsRoiWidth => &ic4_sys::IC4_PROPID_AUTO_FUNCTIONS_ROI_WIDTH[..],
            BalanceRatio => &ic4_sys::IC4_PROPID_BALANCE_RATIO[..],
            BalanceRatioSelector => &ic4_sys::IC4_PROPID_BALANCE_RATIO_SELECTOR[..],
            BalanceWhiteAuto => &ic4_sys::IC4_PROPID_BALANCE_WHITE_AUTO[..],
            BalanceWhiteAutoPreset => &ic4_sys::IC4_PROPID_BALANCE_WHITE_AUTO_PRESET[..],
            BalanceWhiteMode => &ic4_sys::IC4_PROPID_BALANCE_WHITE_MODE[..],
            BalanceWhiteTemperature => &ic4_sys::IC4_PROPID_BALANCE_WHITE_TEMPERATURE[..],
            BalanceWhiteTemperaturePreset => {
                &ic4_sys::IC4_PROPID_BALANCE_WHITE_TEMPERATURE_PRESET[..]
            }
            BinningHorizontal => &ic4_sys::IC4_PROPID_BINNING_HORIZONTAL[..],
            BinningVertical => &ic4_sys::IC4_PROPID_BINNING_VERTICAL[..],
            BlackLevel => &ic4_sys::IC4_PROPID_BLACK_LEVEL[..],
            ChunkBlockId => &ic4_sys::IC4_PROPID_CHUNK_BLOCK_ID[..],
            ChunkEnable => &ic4_sys::IC4_PROPID_CHUNK_ENABLE[..],
            ChunkExposureTime => &ic4_sys::IC4_PROPID_CHUNK_EXPOSURE_TIME[..],
            ChunkGain => &ic4_sys::IC4_PROPID_CHUNK_GAIN[..],
            ChunkImage => &ic4_sys::IC4_PROPID_CHUNK_IMAGE[..],
            ChunkImx174FrameId => &ic4_sys::IC4_PROPID_CHUNK_IMX174_FRAME_ID[..],
            ChunkImx174FrameSet => &ic4_sys::IC4_PROPID_CHUNK_IMX174_FRAME_SET[..],
            ChunkModeActive => &ic4_sys::IC4_PROPID_CHUNK_MODE_ACTIVE[..],
            ChunkMultiFrameSetFrameId => &ic4_sys::IC4_PROPID_CHUNK_MULTI_FRAME_SET_FRAME_ID[..],
            ChunkMultiFrameSetId => &ic4_sys::IC4_PROPID_CHUNK_MULTI_FRAME_SET_ID[..],
            ChunkSelector => &ic4_sys::IC4_PROPID_CHUNK_SELECTOR[..],
            ColorTransformationEnable => &ic4_sys::IC4_PROPID_COLOR_TRANSFORMATION_ENABLE[..],
            ColorTransformationSelector => &ic4_sys::IC4_PROPID_COLOR_TRANSFORMATION_SELECTOR[..],
            ColorTransformationValue => &ic4_sys::IC4_PROPID_COLOR_TRANSFORMATION_VALUE[..],
            ColorTransformationValueSelector => {
                &ic4_sys::IC4_PROPID_COLOR_TRANSFORMATION_VALUE_SELECTOR[..]
            }
            Contrast => &ic4_sys::IC4_PROPID_CONTRAST[..],
            DecimationHorizontal => &ic4_sys::IC4_PROPID_DECIMATION_HORIZONTAL[..],
            DecimationVertical => &ic4_sys::IC4_PROPID_DECIMATION_VERTICAL[..],
            Denoise => &ic4_sys::IC4_PROPID_DENOISE[..],
            DeviceEventChannelCount => &ic4_sys::IC4_PROPID_DEVICE_EVENT_CHANNEL_COUNT[..],
            DeviceFactoryReset => &ic4_sys::IC4_PROPID_DEVICE_FACTORY_RESET[..],
            DeviceFirmwareVersion => &ic4_sys::IC4_PROPID_DEVICE_FIRMWARE_VERSION[..],
            DeviceLinkHeartbeatTimeout => &ic4_sys::IC4_PROPID_DEVICE_LINK_HEARTBEAT_TIMEOUT[..],
            DeviceLinkSelector => &ic4_sys::IC4_PROPID_DEVICE_LINK_SELECTOR[..],
            DeviceModelName => &ic4_sys::IC4_PROPID_DEVICE_MODEL_NAME[..],
            DeviceReset => &ic4_sys::IC4_PROPID_DEVICE_RESET[..],
            DeviceScanType => &ic4_sys::IC4_PROPID_DEVICE_SCAN_TYPE[..],
            DeviceSerialNumber => &ic4_sys::IC4_PROPID_DEVICE_SERIAL_NUMBER[..],
            DeviceSfncVersionMajor => &ic4_sys::IC4_PROPID_DEVICE_SFNC_VERSION_MAJOR[..],
            DeviceSfncVersionMinor => &ic4_sys::IC4_PROPID_DEVICE_SFNC_VERSION_MINOR[..],
            DeviceSfncVersionSubMinor => &ic4_sys::IC4_PROPID_DEVICE_SFNC_VERSION_SUB_MINOR[..],
            DeviceStreamChannelCount => &ic4_sys::IC4_PROPID_DEVICE_STREAM_CHANNEL_COUNT[..],
            DeviceStreamChannelEndianness => {
                &ic4_sys::IC4_PROPID_DEVICE_STREAM_CHANNEL_ENDIANNESS[..]
            }
            DeviceStreamChannelLink => &ic4_sys::IC4_PROPID_DEVICE_STREAM_CHANNEL_LINK[..],
            DeviceStreamChannelPacketSize => {
                &ic4_sys::IC4_PROPID_DEVICE_STREAM_CHANNEL_PACKET_SIZE[..]
            }
            DeviceStreamChannelSelector => &ic4_sys::IC4_PROPID_DEVICE_STREAM_CHANNEL_SELECTOR[..],
            DeviceStreamChannelType => &ic4_sys::IC4_PROPID_DEVICE_STREAM_CHANNEL_TYPE[..],
            DeviceTemperature => &ic4_sys::IC4_PROPID_DEVICE_TEMPERATURE[..],
            DeviceTemperatureSelector => &ic4_sys::IC4_PROPID_DEVICE_TEMPERATURE_SELECTOR[..],
            DeviceTLType => &ic4_sys::IC4_PROPID_DEVICE_TL_TYPE[..],
            DeviceTLVersionMajor => &ic4_sys::IC4_PROPID_DEVICE_TL_VERSION_MAJOR[..],
            DeviceTLVersionMinor => &ic4_sys::IC4_PROPID_DEVICE_TL_VERSION_MINOR[..],
            DeviceTLVersionSubMinor => &ic4_sys::IC4_PROPID_DEVICE_TL_VERSION_SUB_MINOR[..],
            DeviceType => &ic4_sys::IC4_PROPID_DEVICE_TYPE[..],
            DeviceUserId => &ic4_sys::IC4_PROPID_DEVICE_USER_ID[..],
            DeviceVendorName => &ic4_sys::IC4_PROPID_DEVICE_VENDOR_NAME[..],
            DeviceVersion => &ic4_sys::IC4_PROPID_DEVICE_VERSION[..],
            DisableInfoOverlay => &ic4_sys::IC4_PROPID_DISABLE_INFO_OVERLAY[..],
            EventExposureEnd => &ic4_sys::IC4_PROPID_EVENT_EXPOSURE_END[..],
            EventExposureEndFrameId => &ic4_sys::IC4_PROPID_EVENT_EXPOSURE_END_FRAME_ID[..],
            EventExposureEndTimestamp => &ic4_sys::IC4_PROPID_EVENT_EXPOSURE_END_TIMESTAMP[..],
            EventFocusMoveCompleted => &ic4_sys::IC4_PROPID_EVENT_FOCUS_MOVE_COMPLETED[..],
            EventFocusMoveCompletedFocus => {
                &ic4_sys::IC4_PROPID_EVENT_FOCUS_MOVE_COMPLETED_FOCUS[..]
            }
            EventFocusMoveCompletedTimestamp => {
                &ic4_sys::IC4_PROPID_EVENT_FOCUS_MOVE_COMPLETED_TIMESTAMP[..]
            }
            EventFrameTriggerMissed => &ic4_sys::IC4_PROPID_EVENT_FRAME_TRIGGER_MISSED[..],
            EventFrameTriggerMissedTimestamp => {
                &ic4_sys::IC4_PROPID_EVENT_FRAME_TRIGGER_MISSED_TIMESTAMP[..]
            }
            EventLine1FallingEdge => &ic4_sys::IC4_PROPID_EVENT_LINE1_FALLING_EDGE[..],
            EventLine1FallingEdgeTimestamp => {
                &ic4_sys::IC4_PROPID_EVENT_LINE1_FALLING_EDGE_TIMESTAMP[..]
            }
            EventLine1RisingEdge => &ic4_sys::IC4_PROPID_EVENT_LINE1_RISING_EDGE[..],
            EventLine1RisingEdgeTimestamp => {
                &ic4_sys::IC4_PROPID_EVENT_LINE1_RISING_EDGE_TIMESTAMP[..]
            }
            EventNotification => &ic4_sys::IC4_PROPID_EVENT_NOTIFICATION[..],
            EventSelector => &ic4_sys::IC4_PROPID_EVENT_SELECTOR[..],
            EventTest => &ic4_sys::IC4_PROPID_EVENT_TEST[..],
            EventTestTimestamp => &ic4_sys::IC4_PROPID_EVENT_TEST_TIMESTAMP[..],
            EventZoomMoveCompleted => &ic4_sys::IC4_PROPID_EVENT_ZOOM_MOVE_COMPLETED[..],
            EventZoomMoveCompletedTimestamp => {
                &ic4_sys::IC4_PROPID_EVENT_ZOOM_MOVE_COMPLETED_TIMESTAMP[..]
            }
            EventZoomMoveCompletedZoom => &ic4_sys::IC4_PROPID_EVENT_ZOOM_MOVE_COMPLETED_ZOOM[..],
            ExpandOutputRange => &ic4_sys::IC4_PROPID_EXPAND_OUTPUT_RANGE[..],
            ExposureAuto => &ic4_sys::IC4_PROPID_EXPOSURE_AUTO[..],
            ExposureAutoHighlightReduction => {
                &ic4_sys::IC4_PROPID_EXPOSURE_AUTO_HIGHLIGHT_REDUCTION[..]
            }
            ExposureAutoHighlighReduction => {
                &ic4_sys::IC4_PROPID_EXPOSURE_AUTO_HIGHLIGH_REDUCTION[..]
            }
            ExposureAutoLowerLimit => &ic4_sys::IC4_PROPID_EXPOSURE_AUTO_LOWER_LIMIT[..],
            ExposureAutoReference => &ic4_sys::IC4_PROPID_EXPOSURE_AUTO_REFERENCE[..],
            ExposureAutoUpperLimit => &ic4_sys::IC4_PROPID_EXPOSURE_AUTO_UPPER_LIMIT[..],
            ExposureAutoUpperLimitAuto => &ic4_sys::IC4_PROPID_EXPOSURE_AUTO_UPPER_LIMIT_AUTO[..],
            ExposureTime => &ic4_sys::IC4_PROPID_EXPOSURE_TIME[..],
            FileAccessBuffer => &ic4_sys::IC4_PROPID_FILE_ACCESS_BUFFER[..],
            FileAccessLength => &ic4_sys::IC4_PROPID_FILE_ACCESS_LENGTH[..],
            FileAccessOffset => &ic4_sys::IC4_PROPID_FILE_ACCESS_OFFSET[..],
            FileOpenMode => &ic4_sys::IC4_PROPID_FILE_OPEN_MODE[..],
            FileOperationExecute => &ic4_sys::IC4_PROPID_FILE_OPERATION_EXECUTE[..],
            FileOperationResult => &ic4_sys::IC4_PROPID_FILE_OPERATION_RESULT[..],
            FileOperationSelector => &ic4_sys::IC4_PROPID_FILE_OPERATION_SELECTOR[..],
            FileOperationStatus => &ic4_sys::IC4_PROPID_FILE_OPERATION_STATUS[..],
            FileSelector => &ic4_sys::IC4_PROPID_FILE_SELECTOR[..],
            FileSize => &ic4_sys::IC4_PROPID_FILE_SIZE[..],
            Focus => &ic4_sys::IC4_PROPID_FOCUS[..],
            FocusAuto => &ic4_sys::IC4_PROPID_FOCUS_AUTO[..],
            Gain => &ic4_sys::IC4_PROPID_GAIN[..],
            GainAuto => &ic4_sys::IC4_PROPID_GAIN_AUTO[..],
            GainAutoLowerLimit => &ic4_sys::IC4_PROPID_GAIN_AUTO_LOWER_LIMIT[..],
            GainAutoUpperLimit => &ic4_sys::IC4_PROPID_GAIN_AUTO_UPPER_LIMIT[..],
            GainMode => &ic4_sys::IC4_PROPID_GAIN_MODE[..],
            Gamma => &ic4_sys::IC4_PROPID_GAMMA[..],
            GevGvspExtendedIdMode => &ic4_sys::IC4_PROPID_GEV_GVSP_EXTENDED_ID_MODE[..],
            GevScpsDoNotFragment => &ic4_sys::IC4_PROPID_GEV_SCPS_DO_NOT_FRAGMENT[..],
            GevScpsPacketSize => &ic4_sys::IC4_PROPID_GEV_SCPS_PACKET_SIZE[..],
            GpIn => &ic4_sys::IC4_PROPID_GP_IN[..],
            GpOut => &ic4_sys::IC4_PROPID_GP_OUT[..],
            Height => &ic4_sys::IC4_PROPID_HEIGHT[..],
            HeightMax => &ic4_sys::IC4_PROPID_HEIGHT_MAX[..],
            Hue => &ic4_sys::IC4_PROPID_HUE[..],
            Imx174HardwareWdrEnable => &ic4_sys::IC4_PROPID_IMX174_HARDWARE_WDR_ENABLE[..],
            Imx174HardwareWdrShutterMode => {
                &ic4_sys::IC4_PROPID_IMX174_HARDWARE_WDR_SHUTTER_MODE[..]
            }
            Imx174WdrShutter2 => &ic4_sys::IC4_PROPID_IMX174_WDR_SHUTTER2[..],
            ImxLowLatencyTriggerMode => &ic4_sys::IC4_PROPID_IMX_LOW_LATENCY_TRIGGER_MODE[..],
            InputBits => &ic4_sys::IC4_PROPID_INPUT_BITS[..],
            InputFp1ks => &ic4_sys::IC4_PROPID_INPUT_FP1KS[..],
            InputHeight => &ic4_sys::IC4_PROPID_INPUT_HEIGHT[..],
            InputWidth => &ic4_sys::IC4_PROPID_INPUT_WIDTH[..],
            Iris => &ic4_sys::IC4_PROPID_IRIS[..],
            IrisAuto => &ic4_sys::IC4_PROPID_IRIS_AUTO[..],
            IrCutFilterEnable => &ic4_sys::IC4_PROPID_IR_CUT_FILTER_ENABLE[..],
            LutEnable => &ic4_sys::IC4_PROPID_LUT_ENABLE[..],
            LutIndex => &ic4_sys::IC4_PROPID_LUT_INDEX[..],
            LutSelector => &ic4_sys::IC4_PROPID_LUT_SELECTOR[..],
            LutValue => &ic4_sys::IC4_PROPID_LUT_VALUE[..],
            LutValueAll => &ic4_sys::IC4_PROPID_LUT_VALUE_ALL[..],
            MultiFrameSetOutputModeCustomGain => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_CUSTOM_GAIN[..]
            }
            MultiFrameSetOutputModeEnable => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_ENABLE[..]
            }
            MultiFrameSetOutputModeExposureTime0 => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_EXPOSURE_TIME0[..]
            }
            MultiFrameSetOutputModeExposureTime1 => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_EXPOSURE_TIME1[..]
            }
            MultiFrameSetOutputModeExposureTime2 => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_EXPOSURE_TIME2[..]
            }
            MultiFrameSetOutputModeExposureTime3 => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_EXPOSURE_TIME3[..]
            }
            MultiFrameSetOutputModeFrameCount => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_FRAME_COUNT[..]
            }
            MultiFrameSetOutputModeGain0 => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_GAIN0[..]
            }
            MultiFrameSetOutputModeGain1 => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_GAIN1[..]
            }
            MultiFrameSetOutputModeGain2 => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_GAIN2[..]
            }
            MultiFrameSetOutputModeGain3 => {
                &ic4_sys::IC4_PROPID_MULTI_FRAME_SET_OUTPUT_MODE_GAIN3[..]
            }
            OffsetAutoCenter => &ic4_sys::IC4_PROPID_OFFSET_AUTO_CENTER[..],
            OffsetX => &ic4_sys::IC4_PROPID_OFFSET_X[..],
            OffsetY => &ic4_sys::IC4_PROPID_OFFSET_Y[..],
            PayloadSize => &ic4_sys::IC4_PROPID_PAYLOAD_SIZE[..],
            PixelFormat => &ic4_sys::IC4_PROPID_PIXEL_FORMAT[..],
            PtpClockAccuracy => &ic4_sys::IC4_PROPID_PTP_CLOCK_ACCURACY[..],
            PtpEnable => &ic4_sys::IC4_PROPID_PTP_ENABLE[..],
            PtpStatus => &ic4_sys::IC4_PROPID_PTP_STATUS[..],
            ReverseX => &ic4_sys::IC4_PROPID_REVERSE_X[..],
            ReverseY => &ic4_sys::IC4_PROPID_REVERSE_Y[..],
            Saturation => &ic4_sys::IC4_PROPID_SATURATION[..],
            SensorHeight => &ic4_sys::IC4_PROPID_SENSOR_HEIGHT[..],
            SensorPixelHeight => &ic4_sys::IC4_PROPID_SENSOR_PIXEL_HEIGHT[..],
            SensorPixelWidth => &ic4_sys::IC4_PROPID_SENSOR_PIXEL_WIDTH[..],
            SensorWidth => &ic4_sys::IC4_PROPID_SENSOR_WIDTH[..],
            Sharpness => &ic4_sys::IC4_PROPID_SHARPNESS[..],
            SidebandUse => &ic4_sys::IC4_PROPID_SIDEBAND_USE[..],
            SignalDetected => &ic4_sys::IC4_PROPID_SIGNAL_DETECTED[..],
            SoftwareTransformEnable => &ic4_sys::IC4_PROPID_SOFTWARE_TRANSFORM_ENABLE[..],
            SourceConnected => &ic4_sys::IC4_PROPID_SOURCE_CONNECTED[..],
            StrobeDelay => &ic4_sys::IC4_PROPID_STROBE_DELAY[..],
            StrobeDuration => &ic4_sys::IC4_PROPID_STROBE_DURATION[..],
            StrobeEnable => &ic4_sys::IC4_PROPID_STROBE_ENABLE[..],
            StrobeOperation => &ic4_sys::IC4_PROPID_STROBE_OPERATION[..],
            StrobePolarity => &ic4_sys::IC4_PROPID_STROBE_POLARITY[..],
            TestEventGenerate => &ic4_sys::IC4_PROPID_TEST_EVENT_GENERATE[..],
            TestPendingAck => &ic4_sys::IC4_PROPID_TEST_PENDING_ACK[..],
            TimestampLatch => &ic4_sys::IC4_PROPID_TIMESTAMP_LATCH[..],
            TimestampLatchString => &ic4_sys::IC4_PROPID_TIMESTAMP_LATCH_STRING[..],
            TimestampLatchValue => &ic4_sys::IC4_PROPID_TIMESTAMP_LATCH_VALUE[..],
            TimestampReset => &ic4_sys::IC4_PROPID_TIMESTAMP_RESET[..],
            TLParamsLocked => &ic4_sys::IC4_PROPID_TL_PARAMS_LOCKED[..],
            ToneMappingEnable => &ic4_sys::IC4_PROPID_TONE_MAPPING_ENABLE[..],
            ToneMappingGlobalBrightness => &ic4_sys::IC4_PROPID_TONE_MAPPING_GLOBAL_BRIGHTNESS[..],
            ToneMappingIntensity => &ic4_sys::IC4_PROPID_TONE_MAPPING_INTENSITY[..],
            TriggerActivation => &ic4_sys::IC4_PROPID_TRIGGER_ACTIVATION[..],
            TriggerDebouncer => &ic4_sys::IC4_PROPID_TRIGGER_DEBOUNCER[..],
            TriggerDelay => &ic4_sys::IC4_PROPID_TRIGGER_DELAY[..],
            TriggerDenoise => &ic4_sys::IC4_PROPID_TRIGGER_DENOISE[..],
            TriggerMask => &ic4_sys::IC4_PROPID_TRIGGER_MASK[..],
            TriggerMode => &ic4_sys::IC4_PROPID_TRIGGER_MODE[..],
            TriggerOperation => &ic4_sys::IC4_PROPID_TRIGGER_OPERATION[..],
            TriggerOverlap => &ic4_sys::IC4_PROPID_TRIGGER_OVERLAP[..],
            TriggerSelector => &ic4_sys::IC4_PROPID_TRIGGER_SELECTOR[..],
            TriggerSoftware => &ic4_sys::IC4_PROPID_TRIGGER_SOFTWARE[..],
            TriggerSource => &ic4_sys::IC4_PROPID_TRIGGER_SOURCE[..],
            UserSetDefault => &ic4_sys::IC4_PROPID_USER_SET_DEFAULT[..],
            UserSetLoad => &ic4_sys::IC4_PROPID_USER_SET_LOAD[..],
            UserSetSave => &ic4_sys::IC4_PROPID_USER_SET_SAVE[..],
            UserSetSelector => &ic4_sys::IC4_PROPID_USER_SET_SELECTOR[..],
            Width => &ic4_sys::IC4_PROPID_WIDTH[..],
            WidthMax => &ic4_sys::IC4_PROPID_WIDTH_MAX[..],
            Zoom => &ic4_sys::IC4_PROPID_ZOOM[..],
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub fn as_ptr(&self) -> *const c_char {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes()).as_ptr() }
    }
}
