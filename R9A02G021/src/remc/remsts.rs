#[doc = "Register `REMSTS` reader"]
pub type R = crate::R<RemstsSpec>;
#[doc = "Register `REMSTS` writer"]
pub type W = crate::W<RemstsSpec>;
#[doc = "Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpflg {
    #[doc = "0: Mismatch"]
    _0 = 0,
    #[doc = "1: Match"]
    _1 = 1,
}
impl From<Cpflg> for bool {
    #[inline(always)]
    fn from(variant: Cpflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPFLG` reader - Compare Match Flag"]
pub type CpflgR = crate::BitReader<Cpflg>;
impl CpflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpflg {
        match self.bits {
            false => Cpflg::_0,
            true => Cpflg::_1,
        }
    }
    #[doc = "Mismatch"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpflg::_0
    }
    #[doc = "Match"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpflg::_1
    }
}
#[doc = "Receive Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reflg {
    #[doc = "0: No error has occurred"]
    _0 = 0,
    #[doc = "1: An error has occurred"]
    _1 = 1,
}
impl From<Reflg> for bool {
    #[inline(always)]
    fn from(variant: Reflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFLG` reader - Receive Error Flag"]
pub type ReflgR = crate::BitReader<Reflg>;
impl ReflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reflg {
        match self.bits {
            false => Reflg::_0,
            true => Reflg::_1,
        }
    }
    #[doc = "No error has occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reflg::_0
    }
    #[doc = "An error has occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reflg::_1
    }
}
#[doc = "Data Receiving Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drflg {
    #[doc = "0: Waiting for data reception"]
    _0 = 0,
    #[doc = "1: Data is being received"]
    _1 = 1,
}
impl From<Drflg> for bool {
    #[inline(always)]
    fn from(variant: Drflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRFLG` reader - Data Receiving Flag"]
pub type DrflgR = crate::BitReader<Drflg>;
impl DrflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drflg {
        match self.bits {
            false => Drflg::_0,
            true => Drflg::_1,
        }
    }
    #[doc = "Waiting for data reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drflg::_0
    }
    #[doc = "Data is being received"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drflg::_1
    }
}
#[doc = "Receive Buffer Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfulflg {
    #[doc = "0: Receive buffer is empty"]
    _0 = 0,
    #[doc = "1: Receive buffer is full (64 bits received)"]
    _1 = 1,
}
impl From<Bfulflg> for bool {
    #[inline(always)]
    fn from(variant: Bfulflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFULFLG` reader - Receive Buffer Full Flag"]
pub type BfulflgR = crate::BitReader<Bfulflg>;
impl BfulflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfulflg {
        match self.bits {
            false => Bfulflg::_0,
            true => Bfulflg::_1,
        }
    }
    #[doc = "Receive buffer is empty"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfulflg::_0
    }
    #[doc = "Receive buffer is full (64 bits received)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfulflg::_1
    }
}
#[doc = "Field `BFULFLG` writer - Receive Buffer Full Flag"]
pub type BfulflgW<'a, REG> = crate::BitWriter<'a, REG, Bfulflg>;
impl<'a, REG> BfulflgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive buffer is empty"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfulflg::_0)
    }
    #[doc = "Receive buffer is full (64 bits received)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfulflg::_1)
    }
}
#[doc = "Header Pattern Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdflg {
    #[doc = "0: Mismatch"]
    _0 = 0,
    #[doc = "1: Match"]
    _1 = 1,
}
impl From<Hdflg> for bool {
    #[inline(always)]
    fn from(variant: Hdflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDFLG` reader - Header Pattern Match Flag"]
pub type HdflgR = crate::BitReader<Hdflg>;
impl HdflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdflg {
        match self.bits {
            false => Hdflg::_0,
            true => Hdflg::_1,
        }
    }
    #[doc = "Mismatch"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hdflg::_0
    }
    #[doc = "Match"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hdflg::_1
    }
}
#[doc = "Data 0 Pattern Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D0flg {
    #[doc = "0: Mismatch"]
    _0 = 0,
    #[doc = "1: Match"]
    _1 = 1,
}
impl From<D0flg> for bool {
    #[inline(always)]
    fn from(variant: D0flg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D0FLG` reader - Data 0 Pattern Match Flag"]
pub type D0flgR = crate::BitReader<D0flg>;
impl D0flgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D0flg {
        match self.bits {
            false => D0flg::_0,
            true => D0flg::_1,
        }
    }
    #[doc = "Mismatch"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == D0flg::_0
    }
    #[doc = "Match"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == D0flg::_1
    }
}
#[doc = "Data 1 Pattern Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D1flg {
    #[doc = "0: Mismatch"]
    _0 = 0,
    #[doc = "1: Match"]
    _1 = 1,
}
impl From<D1flg> for bool {
    #[inline(always)]
    fn from(variant: D1flg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D1FLG` reader - Data 1 Pattern Match Flag"]
pub type D1flgR = crate::BitReader<D1flg>;
impl D1flgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> D1flg {
        match self.bits {
            false => D1flg::_0,
            true => D1flg::_1,
        }
    }
    #[doc = "Mismatch"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == D1flg::_0
    }
    #[doc = "Match"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == D1flg::_1
    }
}
#[doc = "Special Data Pattern Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdflg {
    #[doc = "0: Mismatch"]
    _0 = 0,
    #[doc = "1: Match"]
    _1 = 1,
}
impl From<Sdflg> for bool {
    #[inline(always)]
    fn from(variant: Sdflg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDFLG` reader - Special Data Pattern Match Flag"]
pub type SdflgR = crate::BitReader<Sdflg>;
impl SdflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdflg {
        match self.bits {
            false => Sdflg::_0,
            true => Sdflg::_1,
        }
    }
    #[doc = "Mismatch"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdflg::_0
    }
    #[doc = "Match"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdflg::_1
    }
}
impl R {
    #[doc = "Bit 0 - Compare Match Flag"]
    #[inline(always)]
    pub fn cpflg(&self) -> CpflgR {
        CpflgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Error Flag"]
    #[inline(always)]
    pub fn reflg(&self) -> ReflgR {
        ReflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Receiving Flag"]
    #[inline(always)]
    pub fn drflg(&self) -> DrflgR {
        DrflgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn bfulflg(&self) -> BfulflgR {
        BfulflgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Header Pattern Match Flag"]
    #[inline(always)]
    pub fn hdflg(&self) -> HdflgR {
        HdflgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data 0 Pattern Match Flag"]
    #[inline(always)]
    pub fn d0flg(&self) -> D0flgR {
        D0flgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data 1 Pattern Match Flag"]
    #[inline(always)]
    pub fn d1flg(&self) -> D1flgR {
        D1flgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Special Data Pattern Match Flag"]
    #[inline(always)]
    pub fn sdflg(&self) -> SdflgR {
        SdflgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive Buffer Full Flag"]
    #[inline(always)]
    pub fn bfulflg(&mut self) -> BfulflgW<RemstsSpec> {
        BfulflgW::new(self, 3)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemstsSpec;
impl crate::RegisterSpec for RemstsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remsts::R`](R) reader structure"]
impl crate::Readable for RemstsSpec {}
#[doc = "`write(|w| ..)` method takes [`remsts::W`](W) writer structure"]
impl crate::Writable for RemstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REMSTS to value 0"]
impl crate::Resettable for RemstsSpec {
    const RESET_VALUE: u8 = 0;
}
