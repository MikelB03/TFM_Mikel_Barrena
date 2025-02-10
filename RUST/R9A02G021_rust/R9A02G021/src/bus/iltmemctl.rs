#[doc = "Register `ILTMEMCTL` reader"]
pub type R = crate::R<IltmemctlSpec>;
#[doc = "Register `ILTMEMCTL` writer"]
pub type W = crate::W<IltmemctlSpec>;
#[doc = "Illicit Memory Access Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iltmemen {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<Iltmemen> for bool {
    #[inline(always)]
    fn from(variant: Iltmemen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILTMEMEN` reader - Illicit Memory Access Detection Enable"]
pub type IltmemenR = crate::BitReader<Iltmemen>;
impl IltmemenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iltmemen {
        match self.bits {
            false => Iltmemen::_0,
            true => Iltmemen::_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Iltmemen::_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Iltmemen::_1
    }
}
#[doc = "Field `ILTMEMEN` writer - Illicit Memory Access Detection Enable"]
pub type IltmemenW<'a, REG> = crate::BitWriter<'a, REG, Iltmemen>;
impl<'a, REG> IltmemenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Iltmemen::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Iltmemen::_1)
    }
}
#[doc = "Field `KEY` reader - Key Code"]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - Key Code"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - Illicit Memory Access Detection Enable"]
    #[inline(always)]
    pub fn iltmemen(&self) -> IltmemenR {
        IltmemenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Illicit Memory Access Detection Enable"]
    #[inline(always)]
    pub fn iltmemen(&mut self) -> IltmemenW<IltmemctlSpec> {
        IltmemenW::new(self, 2)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<IltmemctlSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Illicit Memory Access Detection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iltmemctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iltmemctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IltmemctlSpec;
impl crate::RegisterSpec for IltmemctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iltmemctl::R`](R) reader structure"]
impl crate::Readable for IltmemctlSpec {}
#[doc = "`write(|w| ..)` method takes [`iltmemctl::W`](W) writer structure"]
impl crate::Writable for IltmemctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ILTMEMCTL to value 0"]
impl crate::Resettable for IltmemctlSpec {
    const RESET_VALUE: u16 = 0;
}
