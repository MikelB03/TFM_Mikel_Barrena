#[doc = "Register `FASR` reader"]
pub type R = crate::R<FasrSpec>;
#[doc = "Register `FASR` writer"]
pub type W = crate::W<FasrSpec>;
#[doc = "Extra Area Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exs {
    #[doc = "0: User area or data area"]
    _0 = 0,
    #[doc = "1: Extra area"]
    _1 = 1,
}
impl From<Exs> for bool {
    #[inline(always)]
    fn from(variant: Exs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXS` reader - Extra Area Select"]
pub type ExsR = crate::BitReader<Exs>;
impl ExsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exs {
        match self.bits {
            false => Exs::_0,
            true => Exs::_1,
        }
    }
    #[doc = "User area or data area"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Exs::_0
    }
    #[doc = "Extra area"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Exs::_1
    }
}
#[doc = "Field `EXS` writer - Extra Area Select"]
pub type ExsW<'a, REG> = crate::BitWriter<'a, REG, Exs>;
impl<'a, REG> ExsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "User area or data area"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Exs::_0)
    }
    #[doc = "Extra area"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Exs::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Extra Area Select"]
    #[inline(always)]
    pub fn exs(&self) -> ExsR {
        ExsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Extra Area Select"]
    #[inline(always)]
    pub fn exs(&mut self) -> ExsW<FasrSpec> {
        ExsW::new(self, 0)
    }
}
#[doc = "Flash Area Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fasr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fasr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FasrSpec;
impl crate::RegisterSpec for FasrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fasr::R`](R) reader structure"]
impl crate::Readable for FasrSpec {}
#[doc = "`write(|w| ..)` method takes [`fasr::W`](W) writer structure"]
impl crate::Writable for FasrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FASR to value 0"]
impl crate::Resettable for FasrSpec {
    const RESET_VALUE: u8 = 0;
}
