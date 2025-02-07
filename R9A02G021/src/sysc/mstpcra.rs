#[doc = "Register `MSTPCRA` reader"]
pub type R = crate::R<MstpcraSpec>;
#[doc = "Register `MSTPCRA` writer"]
pub type W = crate::W<MstpcraSpec>;
#[doc = "DTC Module Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstpa22 {
    #[doc = "0: Cancel the module-stop state"]
    _0 = 0,
    #[doc = "1: Enter the module-stop state"]
    _1 = 1,
}
impl From<Mstpa22> for bool {
    #[inline(always)]
    fn from(variant: Mstpa22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTPA22` reader - DTC Module Stop"]
pub type Mstpa22R = crate::BitReader<Mstpa22>;
impl Mstpa22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstpa22 {
        match self.bits {
            false => Mstpa22::_0,
            true => Mstpa22::_1,
        }
    }
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Mstpa22::_0
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Mstpa22::_1
    }
}
#[doc = "Field `MSTPA22` writer - DTC Module Stop"]
pub type Mstpa22W<'a, REG> = crate::BitWriter<'a, REG, Mstpa22>;
impl<'a, REG> Mstpa22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cancel the module-stop state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa22::_0)
    }
    #[doc = "Enter the module-stop state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstpa22::_1)
    }
}
impl R {
    #[doc = "Bit 22 - DTC Module Stop"]
    #[inline(always)]
    pub fn mstpa22(&self) -> Mstpa22R {
        Mstpa22R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - DTC Module Stop"]
    #[inline(always)]
    pub fn mstpa22(&mut self) -> Mstpa22W<MstpcraSpec> {
        Mstpa22W::new(self, 22)
    }
}
#[doc = "Module Stop Control Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`mstpcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstpcraSpec;
impl crate::RegisterSpec for MstpcraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mstpcra::R`](R) reader structure"]
impl crate::Readable for MstpcraSpec {}
#[doc = "`write(|w| ..)` method takes [`mstpcra::W`](W) writer structure"]
impl crate::Writable for MstpcraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTPCRA to value 0xffbf_ffff"]
impl crate::Resettable for MstpcraSpec {
    const RESET_VALUE: u32 = 0xffbf_ffff;
}
