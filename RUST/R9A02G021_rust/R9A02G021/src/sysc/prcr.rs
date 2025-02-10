#[doc = "Register `PRCR` reader"]
pub type R = crate::R<PrcrSpec>;
#[doc = "Register `PRCR` writer"]
pub type W = crate::W<PrcrSpec>;
#[doc = "Enable writing to the registers related to the clock generation circuit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prc0 {
    #[doc = "0: Disable writes"]
    _0 = 0,
    #[doc = "1: Enable writes"]
    _1 = 1,
}
impl From<Prc0> for bool {
    #[inline(always)]
    fn from(variant: Prc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRC0` reader - Enable writing to the registers related to the clock generation circuit"]
pub type Prc0R = crate::BitReader<Prc0>;
impl Prc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prc0 {
        match self.bits {
            false => Prc0::_0,
            true => Prc0::_1,
        }
    }
    #[doc = "Disable writes"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prc0::_0
    }
    #[doc = "Enable writes"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prc0::_1
    }
}
#[doc = "Field `PRC0` writer - Enable writing to the registers related to the clock generation circuit"]
pub type Prc0W<'a, REG> = crate::BitWriter<'a, REG, Prc0>;
impl<'a, REG> Prc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable writes"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prc0::_0)
    }
    #[doc = "Enable writes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prc0::_1)
    }
}
#[doc = "Enable writing to the registers related to the low power modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prc1 {
    #[doc = "0: Disable writes"]
    _0 = 0,
    #[doc = "1: Enable writes"]
    _1 = 1,
}
impl From<Prc1> for bool {
    #[inline(always)]
    fn from(variant: Prc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRC1` reader - Enable writing to the registers related to the low power modes"]
pub type Prc1R = crate::BitReader<Prc1>;
impl Prc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prc1 {
        match self.bits {
            false => Prc1::_0,
            true => Prc1::_1,
        }
    }
    #[doc = "Disable writes"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prc1::_0
    }
    #[doc = "Enable writes"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prc1::_1
    }
}
#[doc = "Field `PRC1` writer - Enable writing to the registers related to the low power modes"]
pub type Prc1W<'a, REG> = crate::BitWriter<'a, REG, Prc1>;
impl<'a, REG> Prc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable writes"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prc1::_0)
    }
    #[doc = "Enable writes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prc1::_1)
    }
}
#[doc = "Enable writing to the registers related to the LVD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prc3 {
    #[doc = "0: Disable writes"]
    _0 = 0,
    #[doc = "1: Enable writes"]
    _1 = 1,
}
impl From<Prc3> for bool {
    #[inline(always)]
    fn from(variant: Prc3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRC3` reader - Enable writing to the registers related to the LVD"]
pub type Prc3R = crate::BitReader<Prc3>;
impl Prc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prc3 {
        match self.bits {
            false => Prc3::_0,
            true => Prc3::_1,
        }
    }
    #[doc = "Disable writes"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Prc3::_0
    }
    #[doc = "Enable writes"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prc3::_1
    }
}
#[doc = "Field `PRC3` writer - Enable writing to the registers related to the LVD"]
pub type Prc3W<'a, REG> = crate::BitWriter<'a, REG, Prc3>;
impl<'a, REG> Prc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable writes"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Prc3::_0)
    }
    #[doc = "Enable writes"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prc3::_1)
    }
}
#[doc = "Field `PRKEY` writer - PRC Key Code"]
pub type PrkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable writing to the registers related to the clock generation circuit"]
    #[inline(always)]
    pub fn prc0(&self) -> Prc0R {
        Prc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable writing to the registers related to the low power modes"]
    #[inline(always)]
    pub fn prc1(&self) -> Prc1R {
        Prc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable writing to the registers related to the LVD"]
    #[inline(always)]
    pub fn prc3(&self) -> Prc3R {
        Prc3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable writing to the registers related to the clock generation circuit"]
    #[inline(always)]
    pub fn prc0(&mut self) -> Prc0W<PrcrSpec> {
        Prc0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable writing to the registers related to the low power modes"]
    #[inline(always)]
    pub fn prc1(&mut self) -> Prc1W<PrcrSpec> {
        Prc1W::new(self, 1)
    }
    #[doc = "Bit 3 - Enable writing to the registers related to the LVD"]
    #[inline(always)]
    pub fn prc3(&mut self) -> Prc3W<PrcrSpec> {
        Prc3W::new(self, 3)
    }
    #[doc = "Bits 8:15 - PRC Key Code"]
    #[inline(always)]
    pub fn prkey(&mut self) -> PrkeyW<PrcrSpec> {
        PrkeyW::new(self, 8)
    }
}
#[doc = "Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrcrSpec;
impl crate::RegisterSpec for PrcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`prcr::R`](R) reader structure"]
impl crate::Readable for PrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`prcr::W`](W) writer structure"]
impl crate::Writable for PrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PRCR to value 0"]
impl crate::Resettable for PrcrSpec {
    const RESET_VALUE: u16 = 0;
}
