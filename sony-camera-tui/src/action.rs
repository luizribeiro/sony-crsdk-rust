/// Actions represent all possible user intents and system events
#[derive(Debug, Clone)]
pub enum Action {
    // Lifecycle
    Quit,
    Tick,
    FlushPendingProperty,

    // Navigation
    Back,
    ShowHelp,
    HideHelp,

    // Discovery screen
    SelectNextCamera,
    SelectPrevCamera,
    ConnectToSelected,
    StartScan,
    ShowManualConnect,

    // Dashboard
    SelectNextDashboardProperty,
    SelectPrevDashboardProperty,
    AdjustPropertyUp,
    AdjustPropertyDown,
    Capture,
    HalfPressShutter,
    StartRecording,
    StopRecording,
    ShowPropertyEditor,
    ShowEventsExpanded,
    Disconnect,

    // Property editor
    PropertyEditorNext,         // Move down in current focus area (j/down)
    PropertyEditorPrev,         // Move up in current focus area (k/up)
    PropertyEditorNextCategory, // Go to next category (Tab)
    PropertyEditorPrevCategory, // Go to previous category (Shift+Tab)
    PropertyEditorValueNext,    // Change to next value (l/right)
    PropertyEditorValuePrev,    // Change to prev value (h/left)
    TogglePropertyPin,          // Toggle pin to quick settings (*)
    OpenPropertyInEditor,       // Jump to property in editor (o)
    PropertyEditorOpenValues,   // Open value selector (o in property editor)
    PropertyEditorApplyValue,   // Apply selected value (Enter in Values focus)
    TogglePropertyInfo,         // Toggle property info panel (i)
    ShowPropertySearch,         // Open property search modal (/)

    // Events log
    ScrollEventsUp,
    ScrollEventsDown,
    ScrollEventsToTop,
    ScrollEventsToBottom,
    ClearEvents,

    // Modals
    ModalClose,
    ModalConfirm,
    ModalNextField,
    ModalPrevField,
    ModalToggleCheckbox,
    ModalInputChar(char),
    ModalInputBackspace,
    ModalInputDelete,
    ModalInputLeft,
    ModalInputRight,
    ModalSelectNext,
    ModalSelectPrev,
}
