pub enum GuiAction {
    Exit,
    TogglePause,
    Restart,
    ClearError,
    StepCycles(u32),
    StepFrames(u32),
    SetBreakpoint(u16),
    RemoveBreakpoint(u16),
}
