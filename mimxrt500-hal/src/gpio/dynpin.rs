/// Value-level `enum` for disabled configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynDisabled {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for input configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynInput {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for interrupt configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynInterrupt {
    Floating,
    PullDown,
    PullUp,
}

/// Value-level `enum` for output configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynOutput {
    PushPull,
    Readable,
}

/// Value-level `enum` for alternate peripheral function configurations
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynAlternate {
    Func1,
    Func2,
    Func3,
    Func4,
    Func5,
    Func6,
    Func7,
    Func8,
    Func9,
    Func10,
    Func11,
    Func12,
    Func13,
    Func14,
    Func15,
}

/// Value-level `enum` representing pin modes
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynPinMode {
    Disabled(DynDisabled),
    Input(DynInput),
    Interrupt(DynInterrupt),
    Output(DynOutput),
    Alternate(DynAlternate),
}

/// Value-level `struct` representing pin IDs
#[derive(PartialEq, Clone, Copy)]
pub struct DynPinId {
    pub group: DynGroup,
    pub num: u8,
}

/// Value-level `enum` for pin groups
#[derive(PartialEq, Clone, Copy)]
pub enum DynGroup {
    Group0,
    Group1,
    Group2,
    Group3,
    Group4,
    Group5,
}
