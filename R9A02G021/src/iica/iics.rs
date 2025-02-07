#[doc = "Register `IICS%s` reader"]
pub type R = crate::R<IicsSpec>;
#[doc = "Detection of stop condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spd {
    #[doc = "0: Stop condition was not detected."]
    _0 = 0,
    #[doc = "1: Stop condition was detected. Communication of the master device is terminated and the bus is released."]
    _1 = 1,
}
impl From<Spd> for bool {
    #[inline(always)]
    fn from(variant: Spd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPD` reader - Detection of stop condition"]
pub type SpdR = crate::BitReader<Spd>;
impl SpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spd {
        match self.bits {
            false => Spd::_0,
            true => Spd::_1,
        }
    }
    #[doc = "Stop condition was not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Spd::_0
    }
    #[doc = "Stop condition was detected. Communication of the master device is terminated and the bus is released."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Spd::_1
    }
}
#[doc = "Detection of start condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Std {
    #[doc = "0: Start condition was not detected."]
    _0 = 0,
    #[doc = "1: Start condition was detected. This indicates that the address transfer period is in effect."]
    _1 = 1,
}
impl From<Std> for bool {
    #[inline(always)]
    fn from(variant: Std) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STD` reader - Detection of start condition"]
pub type StdR = crate::BitReader<Std>;
impl StdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Std {
        match self.bits {
            false => Std::_0,
            true => Std::_1,
        }
    }
    #[doc = "Start condition was not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Std::_0
    }
    #[doc = "Start condition was detected. This indicates that the address transfer period is in effect."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Std::_1
    }
}
#[doc = "Detection of acknowledge (ACK)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackd {
    #[doc = "0: Acknowledge was not detected."]
    _0 = 0,
    #[doc = "1: Acknowledge was detected."]
    _1 = 1,
}
impl From<Ackd> for bool {
    #[inline(always)]
    fn from(variant: Ackd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKD` reader - Detection of acknowledge (ACK)"]
pub type AckdR = crate::BitReader<Ackd>;
impl AckdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackd {
        match self.bits {
            false => Ackd::_0,
            true => Ackd::_1,
        }
    }
    #[doc = "Acknowledge was not detected."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ackd::_0
    }
    #[doc = "Acknowledge was detected."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ackd::_1
    }
}
#[doc = "Detection of transmit and receive status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trc {
    #[doc = "0: Receive status (other than transmit status). The SDAAn line is set for high impedance."]
    _0 = 0,
    #[doc = "1: Transmit status. The value in the SOn latch is enabled for output to the SDAAn line (valid starting at the falling edge of the first byte's ninth clock)."]
    _1 = 1,
}
impl From<Trc> for bool {
    #[inline(always)]
    fn from(variant: Trc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRC` reader - Detection of transmit and receive status"]
pub type TrcR = crate::BitReader<Trc>;
impl TrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trc {
        match self.bits {
            false => Trc::_0,
            true => Trc::_1,
        }
    }
    #[doc = "Receive status (other than transmit status). The SDAAn line is set for high impedance."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Trc::_0
    }
    #[doc = "Transmit status. The value in the SOn latch is enabled for output to the SDAAn line (valid starting at the falling edge of the first byte's ninth clock)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Trc::_1
    }
}
#[doc = "Detection of matching addresses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coi {
    #[doc = "0: Addresses do not match."]
    _0 = 0,
    #[doc = "1: Addresses match. Or, the all address match function is enabled."]
    _1 = 1,
}
impl From<Coi> for bool {
    #[inline(always)]
    fn from(variant: Coi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COI` reader - Detection of matching addresses"]
pub type CoiR = crate::BitReader<Coi>;
impl CoiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Coi {
        match self.bits {
            false => Coi::_0,
            true => Coi::_1,
        }
    }
    #[doc = "Addresses do not match."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Coi::_0
    }
    #[doc = "Addresses match. Or, the all address match function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Coi::_1
    }
}
#[doc = "Detection of extension code reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exc {
    #[doc = "0: Extension code was not received."]
    _0 = 0,
    #[doc = "1: Extension code was received. Or, the all address match function is enabled."]
    _1 = 1,
}
impl From<Exc> for bool {
    #[inline(always)]
    fn from(variant: Exc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXC` reader - Detection of extension code reception"]
pub type ExcR = crate::BitReader<Exc>;
impl ExcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exc {
        match self.bits {
            false => Exc::_0,
            true => Exc::_1,
        }
    }
    #[doc = "Extension code was not received."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exc::_0
    }
    #[doc = "Extension code was received. Or, the all address match function is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exc::_1
    }
}
#[doc = "Detection of arbitration loss\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ald {
    #[doc = "0: This status means either that there was no arbitration, or that the arbitration result was a win."]
    _0 = 0,
    #[doc = "1: This status indicates the arbitration result was a loss. The MSTS bit is cleared."]
    _1 = 1,
}
impl From<Ald> for bool {
    #[inline(always)]
    fn from(variant: Ald) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALD` reader - Detection of arbitration loss"]
pub type AldR = crate::BitReader<Ald>;
impl AldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ald {
        match self.bits {
            false => Ald::_0,
            true => Ald::_1,
        }
    }
    #[doc = "This status means either that there was no arbitration, or that the arbitration result was a win."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ald::_0
    }
    #[doc = "This status indicates the arbitration result was a loss. The MSTS bit is cleared."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ald::_1
    }
}
#[doc = "Master status check flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msts {
    #[doc = "0: Slave device status or communication standby status"]
    _0 = 0,
    #[doc = "1: Master device communication status"]
    _1 = 1,
}
impl From<Msts> for bool {
    #[inline(always)]
    fn from(variant: Msts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTS` reader - Master status check flag"]
pub type MstsR = crate::BitReader<Msts>;
impl MstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msts {
        match self.bits {
            false => Msts::_0,
            true => Msts::_1,
        }
    }
    #[doc = "Slave device status or communication standby status"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Msts::_0
    }
    #[doc = "Master device communication status"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Msts::_1
    }
}
impl R {
    #[doc = "Bit 0 - Detection of stop condition"]
    #[inline(always)]
    pub fn spd(&self) -> SpdR {
        SpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Detection of start condition"]
    #[inline(always)]
    pub fn std(&self) -> StdR {
        StdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Detection of acknowledge (ACK)"]
    #[inline(always)]
    pub fn ackd(&self) -> AckdR {
        AckdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Detection of transmit and receive status"]
    #[inline(always)]
    pub fn trc(&self) -> TrcR {
        TrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Detection of matching addresses"]
    #[inline(always)]
    pub fn coi(&self) -> CoiR {
        CoiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Detection of extension code reception"]
    #[inline(always)]
    pub fn exc(&self) -> ExcR {
        ExcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Detection of arbitration loss"]
    #[inline(always)]
    pub fn ald(&self) -> AldR {
        AldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master status check flag"]
    #[inline(always)]
    pub fn msts(&self) -> MstsR {
        MstsR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "IICA Status Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`iics::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IicsSpec;
impl crate::RegisterSpec for IicsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iics::R`](R) reader structure"]
impl crate::Readable for IicsSpec {}
#[doc = "`reset()` method sets IICS%s to value 0"]
impl crate::Resettable for IicsSpec {
    const RESET_VALUE: u8 = 0;
}
