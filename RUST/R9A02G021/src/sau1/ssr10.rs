#[doc = "Register `SSR10` reader"]
pub type R = crate::R<Ssr10Spec>;
#[doc = "Overrun error detection flag of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovf {
    #[doc = "0: No error occurs"]
    _0 = 0,
    #[doc = "1: An error occurs"]
    _1 = 1,
}
impl From<Ovf> for bool {
    #[inline(always)]
    fn from(variant: Ovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF` reader - Overrun error detection flag of channel n"]
pub type OvfR = crate::BitReader<Ovf>;
impl OvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovf {
        match self.bits {
            false => Ovf::_0,
            true => Ovf::_1,
        }
    }
    #[doc = "No error occurs"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovf::_0
    }
    #[doc = "An error occurs"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovf::_1
    }
}
#[doc = "Parity or ACK error detection flag of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pef {
    #[doc = "0: No error occurs"]
    _0 = 0,
    #[doc = "1: Parity error occurs (during UART reception) or ACK is not detected (during I2C transmission)"]
    _1 = 1,
}
impl From<Pef> for bool {
    #[inline(always)]
    fn from(variant: Pef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEF` reader - Parity or ACK error detection flag of channel n"]
pub type PefR = crate::BitReader<Pef>;
impl PefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pef {
        match self.bits {
            false => Pef::_0,
            true => Pef::_1,
        }
    }
    #[doc = "No error occurs"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pef::_0
    }
    #[doc = "Parity error occurs (during UART reception) or ACK is not detected (during I2C transmission)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pef::_1
    }
}
#[doc = "Flag indicating the state of the buffer register for channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bff {
    #[doc = "0: Valid data is not stored in the SDRmn register"]
    _0 = 0,
    #[doc = "1: Valid data is stored in the SDRmn register"]
    _1 = 1,
}
impl From<Bff> for bool {
    #[inline(always)]
    fn from(variant: Bff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFF` reader - Flag indicating the state of the buffer register for channel n"]
pub type BffR = crate::BitReader<Bff>;
impl BffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bff {
        match self.bits {
            false => Bff::_0,
            true => Bff::_1,
        }
    }
    #[doc = "Valid data is not stored in the SDRmn register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bff::_0
    }
    #[doc = "Valid data is stored in the SDRmn register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bff::_1
    }
}
#[doc = "Flag indicating the state of communications for channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsf {
    #[doc = "0: Communication is stopped or suspended"]
    _0 = 0,
    #[doc = "1: Communication is in progress"]
    _1 = 1,
}
impl From<Tsf> for bool {
    #[inline(always)]
    fn from(variant: Tsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSF` reader - Flag indicating the state of communications for channel n"]
pub type TsfR = crate::BitReader<Tsf>;
impl TsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsf {
        match self.bits {
            false => Tsf::_0,
            true => Tsf::_1,
        }
    }
    #[doc = "Communication is stopped or suspended"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tsf::_0
    }
    #[doc = "Communication is in progress"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tsf::_1
    }
}
impl R {
    #[doc = "Bit 0 - Overrun error detection flag of channel n"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity or ACK error detection flag of channel n"]
    #[inline(always)]
    pub fn pef(&self) -> PefR {
        PefR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Flag indicating the state of the buffer register for channel n"]
    #[inline(always)]
    pub fn bff(&self) -> BffR {
        BffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Flag indicating the state of communications for channel n"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Serial Status Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ssr10Spec;
impl crate::RegisterSpec for Ssr10Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssr10::R`](R) reader structure"]
impl crate::Readable for Ssr10Spec {}
#[doc = "`reset()` method sets SSR10 to value 0"]
impl crate::Resettable for Ssr10Spec {
    const RESET_VALUE: u16 = 0;
}
