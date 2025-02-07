#[doc = "Register `FENTRYR` reader"]
pub type R = crate::R<FentryrSpec>;
#[doc = "Register `FENTRYR` writer"]
pub type W = crate::W<FentryrSpec>;
#[doc = "Code Flash P/E Mode Entry 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fentry0 {
    #[doc = "0: The code flash is the read mode"]
    _0 = 0,
    #[doc = "1: The code flash is the P/E mode."]
    _1 = 1,
}
impl From<Fentry0> for bool {
    #[inline(always)]
    fn from(variant: Fentry0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FENTRY0` reader - Code Flash P/E Mode Entry 0"]
pub type Fentry0R = crate::BitReader<Fentry0>;
impl Fentry0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fentry0 {
        match self.bits {
            false => Fentry0::_0,
            true => Fentry0::_1,
        }
    }
    #[doc = "The code flash is the read mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fentry0::_0
    }
    #[doc = "The code flash is the P/E mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fentry0::_1
    }
}
#[doc = "Field `FENTRY0` writer - Code Flash P/E Mode Entry 0"]
pub type Fentry0W<'a, REG> = crate::BitWriter<'a, REG, Fentry0>;
impl<'a, REG> Fentry0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The code flash is the read mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fentry0::_0)
    }
    #[doc = "The code flash is the P/E mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fentry0::_1)
    }
}
#[doc = "Data Flash P/E Mode Entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fentryd {
    #[doc = "0: The data flash is the read mode"]
    _0 = 0,
    #[doc = "1: The data flash is the P/E mode."]
    _1 = 1,
}
impl From<Fentryd> for bool {
    #[inline(always)]
    fn from(variant: Fentryd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FENTRYD` reader - Data Flash P/E Mode Entry"]
pub type FentrydR = crate::BitReader<Fentryd>;
impl FentrydR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fentryd {
        match self.bits {
            false => Fentryd::_0,
            true => Fentryd::_1,
        }
    }
    #[doc = "The data flash is the read mode"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fentryd::_0
    }
    #[doc = "The data flash is the P/E mode."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fentryd::_1
    }
}
#[doc = "Field `FENTRYD` writer - Data Flash P/E Mode Entry"]
pub type FentrydW<'a, REG> = crate::BitWriter<'a, REG, Fentryd>;
impl<'a, REG> FentrydW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The data flash is the read mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fentryd::_0)
    }
    #[doc = "The data flash is the P/E mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fentryd::_1)
    }
}
#[doc = "Field `FEKEY` writer - Key Code"]
pub type FekeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Code Flash P/E Mode Entry 0"]
    #[inline(always)]
    pub fn fentry0(&self) -> Fentry0R {
        Fentry0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Data Flash P/E Mode Entry"]
    #[inline(always)]
    pub fn fentryd(&self) -> FentrydR {
        FentrydR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Code Flash P/E Mode Entry 0"]
    #[inline(always)]
    pub fn fentry0(&mut self) -> Fentry0W<FentryrSpec> {
        Fentry0W::new(self, 0)
    }
    #[doc = "Bit 7 - Data Flash P/E Mode Entry"]
    #[inline(always)]
    pub fn fentryd(&mut self) -> FentrydW<FentryrSpec> {
        FentrydW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Key Code"]
    #[inline(always)]
    pub fn fekey(&mut self) -> FekeyW<FentryrSpec> {
        FekeyW::new(self, 8)
    }
}
#[doc = "Flash P/E Mode Entry Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fentryr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fentryr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FentryrSpec;
impl crate::RegisterSpec for FentryrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fentryr::R`](R) reader structure"]
impl crate::Readable for FentryrSpec {}
#[doc = "`write(|w| ..)` method takes [`fentryr::W`](W) writer structure"]
impl crate::Writable for FentryrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FENTRYR to value 0"]
impl crate::Resettable for FentryrSpec {
    const RESET_VALUE: u16 = 0;
}
