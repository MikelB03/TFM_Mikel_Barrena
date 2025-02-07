#[doc = "Register `DAEXOUT` reader"]
pub type R = crate::R<DaexoutSpec>;
#[doc = "Register `DAEXOUT` writer"]
pub type W = crate::W<DaexoutSpec>;
#[doc = "D/A External Pin Output Enable 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daexo0 {
    #[doc = "0: DA0 output to DACOUT0 pin is disabled."]
    _0 = 0,
    #[doc = "1: DA0 output to DACOUT0 pin is enabled."]
    _1 = 1,
}
impl From<Daexo0> for bool {
    #[inline(always)]
    fn from(variant: Daexo0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAEXO0` reader - D/A External Pin Output Enable 0"]
pub type Daexo0R = crate::BitReader<Daexo0>;
impl Daexo0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daexo0 {
        match self.bits {
            false => Daexo0::_0,
            true => Daexo0::_1,
        }
    }
    #[doc = "DA0 output to DACOUT0 pin is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daexo0::_0
    }
    #[doc = "DA0 output to DACOUT0 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daexo0::_1
    }
}
#[doc = "Field `DAEXO0` writer - D/A External Pin Output Enable 0"]
pub type Daexo0W<'a, REG> = crate::BitWriter<'a, REG, Daexo0>;
impl<'a, REG> Daexo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DA0 output to DACOUT0 pin is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daexo0::_0)
    }
    #[doc = "DA0 output to DACOUT0 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daexo0::_1)
    }
}
#[doc = "D/A External Pin Output Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Daexo1 {
    #[doc = "0: DA1 output to DACOUT1 pin is disabled."]
    _0 = 0,
    #[doc = "1: DA1 output to DACOUT1 pin is enabled."]
    _1 = 1,
}
impl From<Daexo1> for bool {
    #[inline(always)]
    fn from(variant: Daexo1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAEXO1` reader - D/A External Pin Output Enable 1"]
pub type Daexo1R = crate::BitReader<Daexo1>;
impl Daexo1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Daexo1 {
        match self.bits {
            false => Daexo1::_0,
            true => Daexo1::_1,
        }
    }
    #[doc = "DA1 output to DACOUT1 pin is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Daexo1::_0
    }
    #[doc = "DA1 output to DACOUT1 pin is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Daexo1::_1
    }
}
#[doc = "Field `DAEXO1` writer - D/A External Pin Output Enable 1"]
pub type Daexo1W<'a, REG> = crate::BitWriter<'a, REG, Daexo1>;
impl<'a, REG> Daexo1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DA1 output to DACOUT1 pin is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Daexo1::_0)
    }
    #[doc = "DA1 output to DACOUT1 pin is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Daexo1::_1)
    }
}
impl R {
    #[doc = "Bit 6 - D/A External Pin Output Enable 0"]
    #[inline(always)]
    pub fn daexo0(&self) -> Daexo0R {
        Daexo0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D/A External Pin Output Enable 1"]
    #[inline(always)]
    pub fn daexo1(&self) -> Daexo1R {
        Daexo1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - D/A External Pin Output Enable 0"]
    #[inline(always)]
    pub fn daexo0(&mut self) -> Daexo0W<DaexoutSpec> {
        Daexo0W::new(self, 6)
    }
    #[doc = "Bit 7 - D/A External Pin Output Enable 1"]
    #[inline(always)]
    pub fn daexo1(&mut self) -> Daexo1W<DaexoutSpec> {
        Daexo1W::new(self, 7)
    }
}
#[doc = "D/A External Output Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daexout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daexout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaexoutSpec;
impl crate::RegisterSpec for DaexoutSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`daexout::R`](R) reader structure"]
impl crate::Readable for DaexoutSpec {}
#[doc = "`write(|w| ..)` method takes [`daexout::W`](W) writer structure"]
impl crate::Writable for DaexoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DAEXOUT to value 0"]
impl crate::Resettable for DaexoutSpec {
    const RESET_VALUE: u16 = 0;
}
