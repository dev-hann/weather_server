struct WeatherCurrent {
    x: u32,
    y: u32,
    PTY: u32,
    REH: u32,
    RN1: u32,
    T1H: u32,
    UUU: u32,
    VVV: u32,
    VEC: u32,
    WSD: u32,
}

impl WeatherCurrent {
    pub fn new(
        x: u32,
        y: u32,
        PTY: u32,
        REH: u32,
        RN1: u32,
        T1H: u32,
        UUU: u32,
        VVV: u32,
        VEC: u32,
        WSD: u32,
    ) -> Self {
        Self {
            x,
            y,
            PTY,
            REH,
            RN1,
            T1H,
            UUU,
            VVV,
            VEC,
            WSD,
        }
    }
}
