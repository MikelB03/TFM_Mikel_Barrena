#[doc = "Register `ASISA%s` reader"]
pub type R = crate::R<AsisaSpec>;
#[doc = "Overrun error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovea {
    #[doc = "0: No error has occurred"]
    _0 = 0,
    #[doc = "1: An error has occurred"]
    _1 = 1,
}
impl From<Ovea> for bool {
    #[inline(always)]
    fn from(variant: Ovea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVEA` reader - Overrun error flag"]
pub type OveaR = crate::BitReader<Ovea>;
impl OveaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovea {
        match self.bits {
            false => Ovea::_0,
            true => Ovea::_1,
        }
    }
    #[doc = "No error has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovea::_0
    }
    #[doc = "An error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovea::_1
    }
}
#[doc = "Framing error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fea {
    #[doc = "0: No error has occurred"]
    _0 = 0,
    #[doc = "1: An error has occurred"]
    _1 = 1,
}
impl From<Fea> for bool {
    #[inline(always)]
    fn from(variant: Fea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEA` reader - Framing error flag"]
pub type FeaR = crate::BitReader<Fea>;
impl FeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fea {
        match self.bits {
            false => Fea::_0,
            true => Fea::_1,
        }
    }
    #[doc = "No error has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fea::_0
    }
    #[doc = "An error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fea::_1
    }
}
#[doc = "Parity error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pea {
    #[doc = "0: No error has occurred"]
    _0 = 0,
    #[doc = "1: An error has occurred"]
    _1 = 1,
}
impl From<Pea> for bool {
    #[inline(always)]
    fn from(variant: Pea) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEA` reader - Parity error flag"]
pub type PeaR = crate::BitReader<Pea>;
impl PeaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pea {
        match self.bits {
            false => Pea::_0,
            true => Pea::_1,
        }
    }
    #[doc = "No error has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pea::_0
    }
    #[doc = "An error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pea::_1
    }
}
#[doc = "Transmit shift register data flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txsfa {
    #[doc = "0: Data is not being transmitted"]
    _0 = 0,
    #[doc = "1: Data is being transmitted"]
    _1 = 1,
}
impl From<Txsfa> for bool {
    #[inline(always)]
    fn from(variant: Txsfa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSFA` reader - Transmit shift register data flag"]
pub type TxsfaR = crate::BitReader<Txsfa>;
impl TxsfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txsfa {
        match self.bits {
            false => Txsfa::_0,
            true => Txsfa::_1,
        }
    }
    #[doc = "Data is not being transmitted"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txsfa::_0
    }
    #[doc = "Data is being transmitted"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txsfa::_1
    }
}
#[doc = "Transmit buffer data flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txbfa {
    #[doc = "0: No valid data exists in the TXBAn register"]
    _0 = 0,
    #[doc = "1: Valid data exists in the TXBAn register"]
    _1 = 1,
}
impl From<Txbfa> for bool {
    #[inline(always)]
    fn from(variant: Txbfa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXBFA` reader - Transmit buffer data flag"]
pub type TxbfaR = crate::BitReader<Txbfa>;
impl TxbfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txbfa {
        match self.bits {
            false => Txbfa::_0,
            true => Txbfa::_1,
        }
    }
    #[doc = "No valid data exists in the TXBAn register"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txbfa::_0
    }
    #[doc = "Valid data exists in the TXBAn register"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txbfa::_1
    }
}
impl R {
    #[doc = "Bit 0 - Overrun error flag"]
    #[inline(always)]
    pub fn ovea(&self) -> OveaR {
        OveaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error flag"]
    #[inline(always)]
    pub fn fea(&self) -> FeaR {
        FeaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error flag"]
    #[inline(always)]
    pub fn pea(&self) -> PeaR {
        PeaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit shift register data flag"]
    #[inline(always)]
    pub fn txsfa(&self) -> TxsfaR {
        TxsfaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit buffer data flag"]
    #[inline(always)]
    pub fn txbfa(&self) -> TxbfaR {
        TxbfaR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`asisa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsisaSpec;
impl crate::RegisterSpec for AsisaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`asisa::R`](R) reader structure"]
impl crate::Readable for AsisaSpec {}
#[doc = "`reset()` method sets ASISA%s to value 0"]
impl crate::Resettable for AsisaSpec {
    const RESET_VALUE: u8 = 0;
}
