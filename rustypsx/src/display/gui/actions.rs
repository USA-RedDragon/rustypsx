pub enum GuiAction {
    Exit,
    TogglePause,
    Restart,
    ClearError,
    StepCycles(u32),
    StepFrames(u32),
    SetBreakpoint(u32),
    RemoveBreakpoint(u32),
}
