pub struct Controller{
    pub pump_controller: PumpControllerState
}

impl Controller {
    pub fn new() -> Self {
        Controller { pump_controller: PumpControllerState::Init(PumpController::new()) }
    }
}

struct PumpController<S>{
    state: S
}

struct Init {}

struct Waiting{}

struct Running{}

struct NoWater{}


impl PumpController<Init>{
    fn new() -> Self {
        PumpController { state: Init {} }
    }
}

impl From<PumpController<Init>> for PumpController<Waiting> {
    fn from(state: PumpController<Init>) -> PumpController<Waiting> {
        PumpController { state: Waiting {} }
    }
}

impl From<PumpController<Waiting>> for PumpController<Running> {
    fn from(state: PumpController<Waiting>) -> PumpController<Running> {
        PumpController { state: Running {} }
    }
}

impl From<PumpController<Waiting>> for PumpController<NoWater> {
    fn from(state: PumpController<Waiting>) -> PumpController<NoWater> {
        PumpController { state: NoWater {} }
    }
}

impl From<PumpController<Running>> for PumpController<Waiting> {
    fn from(state: PumpController<Running>) -> PumpController<Waiting> {
        PumpController { state: Waiting {} }
    }
}

impl From<PumpController<Running>> for PumpController<NoWater> {
    fn from(state: PumpController<Running>) -> PumpController<NoWater> {
        PumpController { state: NoWater {} }
    }
}

impl From<PumpController<NoWater>> for PumpController<Waiting> {
    fn from(state: PumpController<NoWater>) -> PumpController<Waiting> {
        PumpController { state: Waiting {} }
    }
}

pub enum PumpControllerState {
    Init(PumpController<Init>),
    Waiting(PumpController<Waiting>),
    Running(PumpController<Running>),
    NoWater(PumpController<NoWater>),
}

impl PumpControllerState {
    pub fn step(mut self) -> Self {
        self = match self {
            PumpControllerState::Init(state) => PumpControllerState::Waiting(state.into()),
            PumpControllerState::Waiting(state) => PumpControllerState::Running(state.into()),
            PumpControllerState::Running(state) => PumpControllerState::Waiting(state.into()),
            PumpControllerState::NoWater(state) => PumpControllerState::Waiting(state.into()),
        };
        self
    }

    pub fn to_no_water(mut self) -> Self {
        self = match self {
            PumpControllerState::Waiting(state) => PumpControllerState::NoWater(state.into()),
            PumpControllerState::Running(state) => PumpControllerState::NoWater(state.into()),
            PumpControllerState::Init(state) => PumpControllerState::Waiting(state.into()),
            PumpControllerState::NoWater(state) => PumpControllerState::Waiting(state.into()),
        };
        self
    }
}