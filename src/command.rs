use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Command {
    Execute,
    CloseWindow,
    SwapTags,
    SoftReload,
    HardReload,
    GotoTag,
    MoveWindowUp,
    MoveWindowDown,
    FocusWindowUp,
    FocusWindowDown,
    FocusWorkspaceNext,
    FocusWorkspacePrevious,
    MoveToTag,
    MoveToLastWorkspace,
    MouseMoveWindow,
    NextLayout,
    PreviousLayout,
}
