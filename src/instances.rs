use crate::Tsp;
use anyhow::Result;

/// This enum represents built in instances of the symmetric travelling salesman problem.
#[derive(Debug, strum::EnumString, strum::AsRefStr, strum::Display, strum::EnumIter)]
pub enum Instances {
    A280,
    ALI535,
    ATT48,
    ATT532,
    BAYG29,
    BAYS29,
    BERLIN52,
    BIER127,
    BRAZIL58,
    BRD14051,
    BRG180,
    BURMA14,
    CH130,
    CH150,
    D198,
    D493,
    D657,
    D1291,
    D1655,
    D2103,
    D15112,
    D18512,
    DANTZIG42,
    DSJ1000,
    EIL51,
    EIL76,
    EIL101,
    FL417,
    FL1400,
    FL1577,
    FL3795,
    FNL4461,
    FRI26,
    GIL262,
    GR17,
    GR21,
    GR24,
    GR48,
    GR96,
    GR120,
    GR137,
    GR202,
    GR229,
    GR431,
    GR666,
    HK48,
    KROA100,
    KROA150,
    KROA200,
    KROB100,
    KROB150,
    KROB200,
    KROC100,
    KROD100,
    KROE100,
    LIN105,
    LIN318,
    LINHP318,
    NRW1379,
    P654,
    PA561,
    PCB442,
    PCB1173,
    PCB3038,
    PLA7397,
    PLA33810,
    PLA85900,
    PR76,
    PR107,
    PR124,
    PR136,
    PR144,
    PR152,
    PR226,
    PR264,
    PR299,
    PR439,
    PR1002,
    PR2392,
    RAT99,
    RAT195,
    RAT575,
    RAT783,
    RD100,
    RD400,
    RL1304,
    RL1323,
    RL1889,
    RL5915,
    RL5934,
    RL11849,
    SI175,
    SI535,
    SI1032,
    ST70,
    SWISS42,
    TS225,
    TSP225,
    U159,
    U574,
    U724,
    U1060,
    U1432,
    U1817,
    U2152,
    U2319,
    ULYSSES16,
    ULYSSES22,
    USA13509,
    VM1084,
    VM1748,
}

impl Instances {
    /// Tries to load the built-in instance.
    ///
    /// In theory, all instances should be possible to load.
    /// Thus, only [Instances::load] is public.
    fn try_load(&self) -> Result<Tsp> {
        let name = self.as_ref().to_lowercase();

        let data = crate::FILES
            .get_file(format!("{}.tsp", name))
            .unwrap()
            .contents_utf8()
            .unwrap();

        let opt = crate::FILES
            .get_file(format!("{}.opt.tour", name))
            .map(|file| file.contents_utf8().unwrap());

        Tsp::try_parse(data, opt)
    }

    /// Loads the built-in instance.
    pub fn load(&self) -> Tsp {
        self.try_load().expect("Error while constructing instance")
    }
}
