#[doc = "Register `BUSMCNTDAT` reader"]
pub type R = crate::R<BusmcntdatSpec>;
#[doc = "Register `BUSMCNTDAT` writer"]
pub type W = crate::W<BusmcntdatSpec>;
#[doc = "Ignore Error Responses\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ieres {
    #[doc = "0: Bus errors are reported"]
    _0 = 0,
    #[doc = "1: Bus errors are not reported"]
    _1 = 1,
}
impl From<Ieres> for bool {
    #[inline(always)]
    fn from(variant: Ieres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IERES` reader - Ignore Error Responses"]
pub type IeresR = crate::BitReader<Ieres>;
impl IeresR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ieres {
        match self.bits {
            false => Ieres::_0,
            true => Ieres::_1,
        }
    }
    #[doc = "Bus errors are reported"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ieres::_0
    }
    #[doc = "Bus errors are not reported"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ieres::_1
    }
}
#[doc = "Field `IERES` writer - Ignore Error Responses"]
pub type IeresW<'a, REG> = crate::BitWriter<'a, REG, Ieres>;
impl<'a, REG> IeresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus errors are reported"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ieres::_0)
    }
    #[doc = "Bus errors are not reported"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ieres::_1)
    }
}
impl R {
    #[doc = "Bit 15 - Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(&self) -> IeresR {
        IeresR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Ignore Error Responses"]
    #[inline(always)]
    pub fn ieres(&mut self) -> IeresW<BusmcntdatSpec> {
        IeresW::new(self, 15)
    }
}
#[doc = "Control Register DAT\n\nYou can [`read`](crate::Reg::read) this register and get [`busmcntdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busmcntdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusmcntdatSpec;
impl crate::RegisterSpec for BusmcntdatSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`busmcntdat::R`](R) reader structure"]
impl crate::Readable for BusmcntdatSpec {}
#[doc = "`write(|w| ..)` method takes [`busmcntdat::W`](W) writer structure"]
impl crate::Writable for BusmcntdatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BUSMCNTDAT to value 0"]
impl crate::Resettable for BusmcntdatSpec {
    const RESET_VALUE: u16 = 0;
}
