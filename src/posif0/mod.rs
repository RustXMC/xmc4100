use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Service Request Processing configuration"]
    pub pconf: PCONF,
    #[doc = "0x04 - Service Request Processing Suspend Config"]
    pub psus: PSUS,
    #[doc = "0x08 - Service Request Processing Run Bit Set"]
    pub pruns: PRUNS,
    #[doc = "0x0c - Service Request Processing Run Bit Clear"]
    pub prunc: PRUNC,
    #[doc = "0x10 - Service Request Processing Run Bit Status"]
    pub prun: PRUN,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - Module Identification register"]
    pub midr: MIDR,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - Hall Sensor Patterns"]
    pub halp: HALP,
    #[doc = "0x34 - Hall Sensor Shadow Patterns"]
    pub halps: HALPS,
    _reserved2: [u8; 8usize],
    #[doc = "0x40 - Multi-Channel Pattern"]
    pub mcm: MCM,
    #[doc = "0x44 - Multi-Channel Shadow Pattern"]
    pub mcsm: MCSM,
    #[doc = "0x48 - Multi-Channel Pattern Control set"]
    pub mcms: MCMS,
    #[doc = "0x4c - Multi-Channel Pattern Control clear"]
    pub mcmc: MCMC,
    #[doc = "0x50 - Multi-Channel Pattern Control flag"]
    pub mcmf: MCMF,
    _reserved3: [u8; 12usize],
    #[doc = "0x60 - Quadrature Decoder Control"]
    pub qdc: QDC,
    _reserved4: [u8; 12usize],
    #[doc = "0x70 - Service Request Processing Interrupt Flags"]
    pub pflg: PFLG,
    #[doc = "0x74 - Service Request Processing Interrupt Enable"]
    pub pflge: PFLGE,
    #[doc = "0x78 - Service Request Processing Interrupt Set"]
    pub spflg: SPFLG,
    #[doc = "0x7c - Service Request Processing Interrupt Clear"]
    pub rpflg: RPFLG,
    _reserved5: [u8; 128usize],
    #[doc = "0x100 - POSIF Debug register"]
    pub pdbg: PDBG,
}
#[doc = "Service Request Processing configuration"]
pub struct PCONF {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing configuration"]
pub mod pconf;
#[doc = "Service Request Processing Suspend Config"]
pub struct PSUS {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing Suspend Config"]
pub mod psus;
#[doc = "Service Request Processing Run Bit Set"]
pub struct PRUNS {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing Run Bit Set"]
pub mod pruns;
#[doc = "Service Request Processing Run Bit Clear"]
pub struct PRUNC {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing Run Bit Clear"]
pub mod prunc;
#[doc = "Service Request Processing Run Bit Status"]
pub struct PRUN {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing Run Bit Status"]
pub mod prun;
#[doc = "Module Identification register"]
pub struct MIDR {
    register: VolatileCell<u32>,
}
#[doc = "Module Identification register"]
pub mod midr;
#[doc = "Hall Sensor Patterns"]
pub struct HALP {
    register: VolatileCell<u32>,
}
#[doc = "Hall Sensor Patterns"]
pub mod halp;
#[doc = "Hall Sensor Shadow Patterns"]
pub struct HALPS {
    register: VolatileCell<u32>,
}
#[doc = "Hall Sensor Shadow Patterns"]
pub mod halps;
#[doc = "Multi-Channel Pattern"]
pub struct MCM {
    register: VolatileCell<u32>,
}
#[doc = "Multi-Channel Pattern"]
pub mod mcm;
#[doc = "Multi-Channel Shadow Pattern"]
pub struct MCSM {
    register: VolatileCell<u32>,
}
#[doc = "Multi-Channel Shadow Pattern"]
pub mod mcsm;
#[doc = "Multi-Channel Pattern Control set"]
pub struct MCMS {
    register: VolatileCell<u32>,
}
#[doc = "Multi-Channel Pattern Control set"]
pub mod mcms;
#[doc = "Multi-Channel Pattern Control clear"]
pub struct MCMC {
    register: VolatileCell<u32>,
}
#[doc = "Multi-Channel Pattern Control clear"]
pub mod mcmc;
#[doc = "Multi-Channel Pattern Control flag"]
pub struct MCMF {
    register: VolatileCell<u32>,
}
#[doc = "Multi-Channel Pattern Control flag"]
pub mod mcmf;
#[doc = "Quadrature Decoder Control"]
pub struct QDC {
    register: VolatileCell<u32>,
}
#[doc = "Quadrature Decoder Control"]
pub mod qdc;
#[doc = "Service Request Processing Interrupt Flags"]
pub struct PFLG {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing Interrupt Flags"]
pub mod pflg;
#[doc = "Service Request Processing Interrupt Enable"]
pub struct PFLGE {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing Interrupt Enable"]
pub mod pflge;
#[doc = "Service Request Processing Interrupt Set"]
pub struct SPFLG {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing Interrupt Set"]
pub mod spflg;
#[doc = "Service Request Processing Interrupt Clear"]
pub struct RPFLG {
    register: VolatileCell<u32>,
}
#[doc = "Service Request Processing Interrupt Clear"]
pub mod rpflg;
#[doc = "POSIF Debug register"]
pub struct PDBG {
    register: VolatileCell<u32>,
}
#[doc = "POSIF Debug register"]
pub mod pdbg;