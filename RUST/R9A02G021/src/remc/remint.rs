#[doc = "Register `REMINT` reader"]
pub type R = crate::R<RemintSpec>;
#[doc = "Register `REMINT` writer"]
pub type W = crate::W<RemintSpec>;
#[doc = "Compare Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpint {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Cpint> for bool {
    #[inline(always)]
    fn from(variant: Cpint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPINT` reader - Compare Match Interrupt Enable"]
pub type CpintR = crate::BitReader<Cpint>;
impl CpintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpint {
        match self.bits {
            false => Cpint::_0,
            true => Cpint::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpint::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpint::_1
    }
}
#[doc = "Field `CPINT` writer - Compare Match Interrupt Enable"]
pub type CpintW<'a, REG> = crate::BitWriter<'a, REG, Cpint>;
impl<'a, REG> CpintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpint::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpint::_1)
    }
}
#[doc = "Receive Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reint {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Reint> for bool {
    #[inline(always)]
    fn from(variant: Reint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REINT` reader - Receive Error Interrupt Enable"]
pub type ReintR = crate::BitReader<Reint>;
impl ReintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reint {
        match self.bits {
            false => Reint::_0,
            true => Reint::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Reint::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Reint::_1
    }
}
#[doc = "Field `REINT` writer - Receive Error Interrupt Enable"]
pub type ReintW<'a, REG> = crate::BitWriter<'a, REG, Reint>;
impl<'a, REG> ReintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Reint::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Reint::_1)
    }
}
#[doc = "Data Receiving Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drint {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Drint> for bool {
    #[inline(always)]
    fn from(variant: Drint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRINT` reader - Data Receiving Interrupt Enable"]
pub type DrintR = crate::BitReader<Drint>;
impl DrintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drint {
        match self.bits {
            false => Drint::_0,
            true => Drint::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Drint::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Drint::_1
    }
}
#[doc = "Field `DRINT` writer - Data Receiving Interrupt Enable"]
pub type DrintW<'a, REG> = crate::BitWriter<'a, REG, Drint>;
impl<'a, REG> DrintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Drint::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Drint::_1)
    }
}
#[doc = "Receive Buffer Full Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bfulint {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Bfulint> for bool {
    #[inline(always)]
    fn from(variant: Bfulint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BFULINT` reader - Receive Buffer Full Interrupt Enable"]
pub type BfulintR = crate::BitReader<Bfulint>;
impl BfulintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bfulint {
        match self.bits {
            false => Bfulint::_0,
            true => Bfulint::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bfulint::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bfulint::_1
    }
}
#[doc = "Field `BFULINT` writer - Receive Buffer Full Interrupt Enable"]
pub type BfulintW<'a, REG> = crate::BitWriter<'a, REG, Bfulint>;
impl<'a, REG> BfulintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bfulint::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bfulint::_1)
    }
}
#[doc = "Header Pattern Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdint {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Hdint> for bool {
    #[inline(always)]
    fn from(variant: Hdint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDINT` reader - Header Pattern Match Interrupt Enable"]
pub type HdintR = crate::BitReader<Hdint>;
impl HdintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdint {
        match self.bits {
            false => Hdint::_0,
            true => Hdint::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Hdint::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Hdint::_1
    }
}
#[doc = "Field `HDINT` writer - Header Pattern Match Interrupt Enable"]
pub type HdintW<'a, REG> = crate::BitWriter<'a, REG, Hdint>;
impl<'a, REG> HdintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdint::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdint::_1)
    }
}
#[doc = "Data 0 Pattern or Data 1 Pattern Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dint {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Dint> for bool {
    #[inline(always)]
    fn from(variant: Dint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINT` reader - Data 0 Pattern or Data 1 Pattern Match Interrupt Enable"]
pub type DintR = crate::BitReader<Dint>;
impl DintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dint {
        match self.bits {
            false => Dint::_0,
            true => Dint::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dint::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dint::_1
    }
}
#[doc = "Field `DINT` writer - Data 0 Pattern or Data 1 Pattern Match Interrupt Enable"]
pub type DintW<'a, REG> = crate::BitWriter<'a, REG, Dint>;
impl<'a, REG> DintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dint::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dint::_1)
    }
}
#[doc = "Special Data Pattern Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdint {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Sdint> for bool {
    #[inline(always)]
    fn from(variant: Sdint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDINT` reader - Special Data Pattern Match Interrupt Enable"]
pub type SdintR = crate::BitReader<Sdint>;
impl SdintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdint {
        match self.bits {
            false => Sdint::_0,
            true => Sdint::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sdint::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sdint::_1
    }
}
#[doc = "Field `SDINT` writer - Special Data Pattern Match Interrupt Enable"]
pub type SdintW<'a, REG> = crate::BitWriter<'a, REG, Sdint>;
impl<'a, REG> SdintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sdint::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sdint::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Match Interrupt Enable"]
    #[inline(always)]
    pub fn cpint(&self) -> CpintR {
        CpintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn reint(&self) -> ReintR {
        ReintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Receiving Interrupt Enable"]
    #[inline(always)]
    pub fn drint(&self) -> DrintR {
        DrintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn bfulint(&self) -> BfulintR {
        BfulintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Header Pattern Match Interrupt Enable"]
    #[inline(always)]
    pub fn hdint(&self) -> HdintR {
        HdintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data 0 Pattern or Data 1 Pattern Match Interrupt Enable"]
    #[inline(always)]
    pub fn dint(&self) -> DintR {
        DintR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Special Data Pattern Match Interrupt Enable"]
    #[inline(always)]
    pub fn sdint(&self) -> SdintR {
        SdintR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Match Interrupt Enable"]
    #[inline(always)]
    pub fn cpint(&mut self) -> CpintW<RemintSpec> {
        CpintW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn reint(&mut self) -> ReintW<RemintSpec> {
        ReintW::new(self, 1)
    }
    #[doc = "Bit 2 - Data Receiving Interrupt Enable"]
    #[inline(always)]
    pub fn drint(&mut self) -> DrintW<RemintSpec> {
        DrintW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn bfulint(&mut self) -> BfulintW<RemintSpec> {
        BfulintW::new(self, 3)
    }
    #[doc = "Bit 4 - Header Pattern Match Interrupt Enable"]
    #[inline(always)]
    pub fn hdint(&mut self) -> HdintW<RemintSpec> {
        HdintW::new(self, 4)
    }
    #[doc = "Bit 5 - Data 0 Pattern or Data 1 Pattern Match Interrupt Enable"]
    #[inline(always)]
    pub fn dint(&mut self) -> DintW<RemintSpec> {
        DintW::new(self, 5)
    }
    #[doc = "Bit 7 - Special Data Pattern Match Interrupt Enable"]
    #[inline(always)]
    pub fn sdint(&mut self) -> SdintW<RemintSpec> {
        SdintW::new(self, 7)
    }
}
#[doc = "Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`remint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemintSpec;
impl crate::RegisterSpec for RemintSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`remint::R`](R) reader structure"]
impl crate::Readable for RemintSpec {}
#[doc = "`write(|w| ..)` method takes [`remint::W`](W) writer structure"]
impl crate::Writable for RemintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REMINT to value 0"]
impl crate::Resettable for RemintSpec {
    const RESET_VALUE: u8 = 0;
}
