#[doc = "Register `MSTPCRD` reader"]
pub type R = crate::R<MstpcrdSpec>;
#[doc = "Register `MSTPCRD` writer"]
pub type W = crate::W<MstpcrdSpec>;
#[doc = "32-bit Interval Timer Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd9 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd9> for bool {
    #[inline(always)]
    fn from(variant: Mstpd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD9` reader - 32-bit Interval Timer Module Stop"]
pub type Mstpd9R = crate::BitReader<Mstpd9>;
impl Mstpd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd9 {
        match self.bits {
            false => Mstpd9::_0,
            true => Mstpd9::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd9::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd9::_1
    }
}
#[doc = "Field `MSTPD9` writer - 32-bit Interval Timer Module Stop"]
pub type Mstpd9W<'a, REG> = crate::BitWriter<'a, REG, Mstpd9>;
impl<'a, REG> Mstpd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd9::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd9::_1)
    }
}
#[doc = "Timer Array Unit Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd10 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd10> for bool {
    #[inline(always)]
    fn from(variant: Mstpd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD10` reader - Timer Array Unit Module Stop"]
pub type Mstpd10R = crate::BitReader<Mstpd10>;
impl Mstpd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd10 {
        match self.bits {
            false => Mstpd10::_0,
            true => Mstpd10::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd10::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd10::_1
    }
}
#[doc = "Field `MSTPD10` writer - Timer Array Unit Module Stop"]
pub type Mstpd10W<'a, REG> = crate::BitWriter<'a, REG, Mstpd10>;
impl<'a, REG> Mstpd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd10::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd10::_1)
    }
}
#[doc = "12-bit A/D Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd16 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd16> for bool {
    #[inline(always)]
    fn from(variant: Mstpd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD16` reader - 12-bit A/D Converter Module Stop"]
pub type Mstpd16R = crate::BitReader<Mstpd16>;
impl Mstpd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd16 {
        match self.bits {
            false => Mstpd16::_0,
            true => Mstpd16::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd16::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd16::_1
    }
}
#[doc = "Field `MSTPD16` writer - 12-bit A/D Converter Module Stop"]
pub type Mstpd16W<'a, REG> = crate::BitWriter<'a, REG, Mstpd16>;
impl<'a, REG> Mstpd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd16::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd16::_1)
    }
}
#[doc = "8-bit D/A Converter Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd20 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd20> for bool {
    #[inline(always)]
    fn from(variant: Mstpd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD20` reader - 8-bit D/A Converter Module Stop"]
pub type Mstpd20R = crate::BitReader<Mstpd20>;
impl Mstpd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd20 {
        match self.bits {
            false => Mstpd20::_0,
            true => Mstpd20::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd20::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd20::_1
    }
}
#[doc = "Field `MSTPD20` writer - 8-bit D/A Converter Module Stop"]
pub type Mstpd20W<'a, REG> = crate::BitWriter<'a, REG, Mstpd20>;
impl<'a, REG> Mstpd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd20::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd20::_1)
    }
}
#[doc = "Comparator Module Stop\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpd28 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpd28> for bool {
    #[inline(always)]
    fn from(variant: Mstpd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPD28` reader - Comparator Module Stop"]
pub type Mstpd28R = crate::BitReader<Mstpd28>;
impl Mstpd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpd28 {
        match self.bits {
            false => Mstpd28::_0,
            true => Mstpd28::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpd28::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpd28::_1
    }
}
#[doc = "Field `MSTPD28` writer - Comparator Module Stop"]
pub type Mstpd28W<'a, REG> = crate::BitWriter<'a, REG, Mstpd28>;
impl<'a, REG> Mstpd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd28::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpd28::_1)
    }
}
impl R {
    #[doc = "Bit 9 - 32-bit Interval Timer Module Stop"]
    #[inline(always)]
    pub fn mstpd9(&self) -> Mstpd9R {
        Mstpd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer Array Unit Module Stop"]
    #[inline(always)]
    pub fn mstpd10(&self) -> Mstpd10R {
        Mstpd10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - 12-bit A/D Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd16(&self) -> Mstpd16R {
        Mstpd16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 8-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd20(&self) -> Mstpd20R {
        Mstpd20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 28 - Comparator Module Stop"]
    #[inline(always)]
    pub fn mstpd28(&self) -> Mstpd28R {
        Mstpd28R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - 32-bit Interval Timer Module Stop"]
    #[inline(always)]
    pub fn mstpd9(&mut self) -> Mstpd9W<MstpcrdSpec> {
        Mstpd9W::new(self, 9)
    }
    #[doc = "Bit 10 - Timer Array Unit Module Stop"]
    #[inline(always)]
    pub fn mstpd10(&mut self) -> Mstpd10W<MstpcrdSpec> {
        Mstpd10W::new(self, 10)
    }
    #[doc = "Bit 16 - 12-bit A/D Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd16(&mut self) -> Mstpd16W<MstpcrdSpec> {
        Mstpd16W::new(self, 16)
    }
    #[doc = "Bit 20 - 8-bit D/A Converter Module Stop"]
    #[inline(always)]
    pub fn mstpd20(&mut self) -> Mstpd20W<MstpcrdSpec> {
        Mstpd20W::new(self, 20)
    }
    #[doc = "Bit 28 - Comparator Module Stop"]
    #[inline(always)]
    pub fn mstpd28(&mut self) -> Mstpd28W<MstpcrdSpec> {
        Mstpd28W::new(self, 28)
    }
}
#[doc = "Module Stop Control Register D\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstpcrdSpec;
impl crate::RegisterSpec for MstpcrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcrd::R`](R) reader structure"]
impl crate::Readable for MstpcrdSpec {}
#[doc = "`write(|w| ..)` method takes [`mstpcrd::W`](W) writer structure"]
impl crate::Writable for MstpcrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTPCRD to value 0xffff_ffff"]
impl crate::Resettable for MstpcrdSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
