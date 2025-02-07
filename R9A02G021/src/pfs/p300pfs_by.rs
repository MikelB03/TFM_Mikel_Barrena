#[doc = "Register `P300PFS_BY` reader"]
pub type R = crate::R<P300pfsBySpec>;
#[doc = "Register `P300PFS_BY` writer"]
pub type W = crate::W<P300pfsBySpec>;
#[doc = "Port Output Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Podr {
    #[doc = "0: Low output"]
    _0 = 0,
    #[doc = "1: High output"]
    _1 = 1,
}
impl From<Podr> for bool {
    #[inline(always)]
    fn from(variant: Podr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PODR` reader - Port Output Data"]
pub type PodrR = crate::BitReader<Podr>;
impl PodrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Podr {
        match self.bits {
            false => Podr::_0,
            true => Podr::_1,
        }
    }
    #[doc = "Low output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Podr::_0
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Podr::_1
    }
}
#[doc = "Field `PODR` writer - Port Output Data"]
pub type PodrW<'a, REG> = crate::BitWriter<'a, REG, Podr>;
impl<'a, REG> PodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_0)
    }
    #[doc = "High output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Podr::_1)
    }
}
#[doc = "Pmn State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pidr {
    #[doc = "0: Low level"]
    _0 = 0,
    #[doc = "1: High level"]
    _1 = 1,
}
impl From<Pidr> for bool {
    #[inline(always)]
    fn from(variant: Pidr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDR` reader - Pmn State"]
pub type PidrR = crate::BitReader<Pidr>;
impl PidrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pidr {
        match self.bits {
            false => Pidr::_0,
            true => Pidr::_1,
        }
    }
    #[doc = "Low level"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pidr::_0
    }
    #[doc = "High level"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pidr::_1
    }
}
#[doc = "Port Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdr {
    #[doc = "0: Input (functions as an input pin)"]
    _0 = 0,
    #[doc = "1: Output (functions as an output pin)"]
    _1 = 1,
}
impl From<Pdr> for bool {
    #[inline(always)]
    fn from(variant: Pdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDR` reader - Port Direction"]
pub type PdrR = crate::BitReader<Pdr>;
impl PdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdr {
        match self.bits {
            false => Pdr::_0,
            true => Pdr::_1,
        }
    }
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pdr::_0
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pdr::_1
    }
}
#[doc = "Field `PDR` writer - Port Direction"]
pub type PdrW<'a, REG> = crate::BitWriter<'a, REG, Pdr>;
impl<'a, REG> PdrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input (functions as an input pin)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_0)
    }
    #[doc = "Output (functions as an output pin)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pdr::_1)
    }
}
#[doc = "Pull-up Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcr {
    #[doc = "0: Disable input pull-up"]
    _0 = 0,
    #[doc = "1: Enable input pull-up"]
    _1 = 1,
}
impl From<Pcr> for bool {
    #[inline(always)]
    fn from(variant: Pcr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCR` reader - Pull-up Control"]
pub type PcrR = crate::BitReader<Pcr>;
impl PcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcr {
        match self.bits {
            false => Pcr::_0,
            true => Pcr::_1,
        }
    }
    #[doc = "Disable input pull-up"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pcr::_0
    }
    #[doc = "Enable input pull-up"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pcr::_1
    }
}
#[doc = "Field `PCR` writer - Pull-up Control"]
pub type PcrW<'a, REG> = crate::BitWriter<'a, REG, Pcr>;
impl<'a, REG> PcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable input pull-up"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::_0)
    }
    #[doc = "Enable input pull-up"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pcr::_1)
    }
}
#[doc = "N-Channel Open-Drain Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncodr {
    #[doc = "0: CMOS output"]
    _0 = 0,
    #[doc = "1: NMOS open-drain output"]
    _1 = 1,
}
impl From<Ncodr> for bool {
    #[inline(always)]
    fn from(variant: Ncodr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCODR` reader - N-Channel Open-Drain Control"]
pub type NcodrR = crate::BitReader<Ncodr>;
impl NcodrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncodr {
        match self.bits {
            false => Ncodr::_0,
            true => Ncodr::_1,
        }
    }
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ncodr::_0
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ncodr::_1
    }
}
#[doc = "Field `NCODR` writer - N-Channel Open-Drain Control"]
pub type NcodrW<'a, REG> = crate::BitWriter<'a, REG, Ncodr>;
impl<'a, REG> NcodrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMOS output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ncodr::_0)
    }
    #[doc = "NMOS open-drain output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ncodr::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&self) -> PodrR {
        PodrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pmn State"]
    #[inline(always)]
    pub fn pidr(&self) -> PidrR {
        PidrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&self) -> PdrR {
        PdrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&self) -> PcrR {
        PcrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(&self) -> NcodrR {
        NcodrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Output Data"]
    #[inline(always)]
    pub fn podr(&mut self) -> PodrW<P300pfsBySpec> {
        PodrW::new(self, 0)
    }
    #[doc = "Bit 2 - Port Direction"]
    #[inline(always)]
    pub fn pdr(&mut self) -> PdrW<P300pfsBySpec> {
        PdrW::new(self, 2)
    }
    #[doc = "Bit 4 - Pull-up Control"]
    #[inline(always)]
    pub fn pcr(&mut self) -> PcrW<P300pfsBySpec> {
        PcrW::new(self, 4)
    }
    #[doc = "Bit 6 - N-Channel Open-Drain Control"]
    #[inline(always)]
    pub fn ncodr(&mut self) -> NcodrW<P300pfsBySpec> {
        NcodrW::new(self, 6)
    }
}
#[doc = "Port 300 Pin Function Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`p300pfs_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p300pfs_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P300pfsBySpec;
impl crate::RegisterSpec for P300pfsBySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`p300pfs_by::R`](R) reader structure"]
impl crate::Readable for P300pfsBySpec {}
#[doc = "`write(|w| ..)` method takes [`p300pfs_by::W`](W) writer structure"]
impl crate::Writable for P300pfsBySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets P300PFS_BY to value 0x10"]
impl crate::Resettable for P300pfsBySpec {
    const RESET_VALUE: u8 = 0x10;
}
