#[doc = "Register `MSTPCRC` reader"]
pub type R = crate::R<MstpcrcSpec>;
#[doc = "Register `MSTPCRC` writer"]
pub type W = crate::W<MstpcrcSpec>;
#[doc = "Clock Frequency Accuracy Measurement Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc0 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc0> for bool {
    #[inline(always)]
    fn from(variant: Mstpc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC0` reader - Clock Frequency Accuracy Measurement Circuit Module Stop"]
pub type Mstpc0R = crate::BitReader<Mstpc0>;
impl Mstpc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc0 {
        match self.bits {
            false => Mstpc0::_0,
            true => Mstpc0::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc0::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc0::_1
    }
}
#[doc = "Field `MSTPC0` writer - Clock Frequency Accuracy Measurement Circuit Module Stop"]
pub type Mstpc0W<'a, REG> = crate::BitWriter<'a, REG, Mstpc0>;
impl<'a, REG> Mstpc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc0::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc0::_1)
    }
}
#[doc = "Cyclic Redundancy Check Calculator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc1 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc1> for bool {
    #[inline(always)]
    fn from(variant: Mstpc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC1` reader - Cyclic Redundancy Check Calculator Module Stop"]
pub type Mstpc1R = crate::BitReader<Mstpc1>;
impl Mstpc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc1 {
        match self.bits {
            false => Mstpc1::_0,
            true => Mstpc1::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc1::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc1::_1
    }
}
#[doc = "Field `MSTPC1` writer - Cyclic Redundancy Check Calculator Module Stop"]
pub type Mstpc1W<'a, REG> = crate::BitWriter<'a, REG, Mstpc1>;
impl<'a, REG> Mstpc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc1::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc1::_1)
    }
}
#[doc = "Data Operation Circuit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc13 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc13> for bool {
    #[inline(always)]
    fn from(variant: Mstpc13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC13` reader - Data Operation Circuit Module Stop"]
pub type Mstpc13R = crate::BitReader<Mstpc13>;
impl Mstpc13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc13 {
        match self.bits {
            false => Mstpc13::_0,
            true => Mstpc13::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc13::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc13::_1
    }
}
#[doc = "Field `MSTPC13` writer - Data Operation Circuit Module Stop"]
pub type Mstpc13W<'a, REG> = crate::BitWriter<'a, REG, Mstpc13>;
impl<'a, REG> Mstpc13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc13::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc13::_1)
    }
}
#[doc = "Event Link Controller Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc14 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc14> for bool {
    #[inline(always)]
    fn from(variant: Mstpc14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC14` reader - Event Link Controller Module Stop"]
pub type Mstpc14R = crate::BitReader<Mstpc14>;
impl Mstpc14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc14 {
        match self.bits {
            false => Mstpc14::_0,
            true => Mstpc14::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc14::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc14::_1
    }
}
#[doc = "Field `MSTPC14` writer - Event Link Controller Module Stop"]
pub type Mstpc14W<'a, REG> = crate::BitWriter<'a, REG, Mstpc14>;
impl<'a, REG> Mstpc14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc14::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc14::_1)
    }
}
#[doc = "Remote Control Signal Receiver Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc19 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc19> for bool {
    #[inline(always)]
    fn from(variant: Mstpc19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC19` reader - Remote Control Signal Receiver Module Stop"]
pub type Mstpc19R = crate::BitReader<Mstpc19>;
impl Mstpc19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc19 {
        match self.bits {
            false => Mstpc19::_0,
            true => Mstpc19::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc19::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc19::_1
    }
}
#[doc = "Field `MSTPC19` writer - Remote Control Signal Receiver Module Stop"]
pub type Mstpc19W<'a, REG> = crate::BitWriter<'a, REG, Mstpc19>;
impl<'a, REG> Mstpc19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc19::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc19::_1)
    }
}
#[doc = "8-bit D/A Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc22 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc22> for bool {
    #[inline(always)]
    fn from(variant: Mstpc22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC22` reader - 8-bit D/A Converter Module Stop"]
pub type Mstpc22R = crate::BitReader<Mstpc22>;
impl Mstpc22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc22 {
        match self.bits {
            false => Mstpc22::_0,
            true => Mstpc22::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc22::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc22::_1
    }
}
#[doc = "Field `MSTPC22` writer - 8-bit D/A Converter Module Stop"]
pub type Mstpc22W<'a, REG> = crate::BitWriter<'a, REG, Mstpc22>;
impl<'a, REG> Mstpc22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc22::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc22::_1)
    }
}
#[doc = "True Random Number Generator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpc28 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpc28> for bool {
    #[inline(always)]
    fn from(variant: Mstpc28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPC28` reader - True Random Number Generator Module Stop"]
pub type Mstpc28R = crate::BitReader<Mstpc28>;
impl Mstpc28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpc28 {
        match self.bits {
            false => Mstpc28::_0,
            true => Mstpc28::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpc28::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpc28::_1
    }
}
#[doc = "Field `MSTPC28` writer - True Random Number Generator Module Stop"]
pub type Mstpc28W<'a, REG> = crate::BitWriter<'a, REG, Mstpc28>;
impl<'a, REG> Mstpc28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc28::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpc28::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc0(&self) -> Mstpc0R {
        Mstpc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub fn mstpc1(&self) -> Mstpc1R {
        Mstpc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 13 - Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc13(&self) -> Mstpc13R {
        Mstpc13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Link Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc14(&self) -> Mstpc14R {
        Mstpc14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Remote Control Signal Receiver Module Stop"]
    #[inline(always)]
    pub fn mstpc19(&self) -> Mstpc19R {
        Mstpc19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - 8-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpc22(&self) -> Mstpc22R {
        Mstpc22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - True Random Number Generator Module Stop"]
    #[inline(always)]
    pub fn mstpc28(&self) -> Mstpc28R {
        Mstpc28R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Frequency Accuracy Measurement Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc0(&mut self) -> Mstpc0W<MstpcrcSpec> {
        Mstpc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Cyclic Redundancy Check Calculator Module Stop"]
    #[inline(always)]
    pub fn mstpc1(&mut self) -> Mstpc1W<MstpcrcSpec> {
        Mstpc1W::new(self, 1)
    }
    #[doc = "Bit 13 - Data Operation Circuit Module Stop"]
    #[inline(always)]
    pub fn mstpc13(&mut self) -> Mstpc13W<MstpcrcSpec> {
        Mstpc13W::new(self, 13)
    }
    #[doc = "Bit 14 - Event Link Controller Module Stop"]
    #[inline(always)]
    pub fn mstpc14(&mut self) -> Mstpc14W<MstpcrcSpec> {
        Mstpc14W::new(self, 14)
    }
    #[doc = "Bit 19 - Remote Control Signal Receiver Module Stop"]
    #[inline(always)]
    pub fn mstpc19(&mut self) -> Mstpc19W<MstpcrcSpec> {
        Mstpc19W::new(self, 19)
    }
    #[doc = "Bit 22 - 8-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpc22(&mut self) -> Mstpc22W<MstpcrcSpec> {
        Mstpc22W::new(self, 22)
    }
    #[doc = "Bit 28 - True Random Number Generator Module Stop"]
    #[inline(always)]
    pub fn mstpc28(&mut self) -> Mstpc28W<MstpcrcSpec> {
        Mstpc28W::new(self, 28)
    }
}
#[doc = "Module Stop Control Register C\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstpcrcSpec;
impl crate::RegisterSpec for MstpcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcrc::R`](R) reader structure"]
impl crate::Readable for MstpcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`mstpcrc::W`](W) writer structure"]
impl crate::Writable for MstpcrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTPCRC to value 0xffff_ffff"]
impl crate::Resettable for MstpcrcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
