#[doc = "Register `SOE1` reader"]
pub type R = crate::R<Soe1Spec>;
#[doc = "Register `SOE1` writer"]
pub type W = crate::W<Soe1Spec>;
#[doc = "Serial output enable or stop of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Soe {
    #[doc = "0: Stops output by serial communication operation"]
    _0 = 0,
    #[doc = "1: Enables output by serial communication operation"]
    _1 = 1,
}
impl From<Soe> for u8 {
    #[inline(always)]
    fn from(variant: Soe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Soe {
    type Ux = u8;
}
impl crate::IsEnum for Soe {}
#[doc = "Field `SOE` reader - Serial output enable or stop of channel n"]
pub type SoeR = crate::FieldReader<Soe>;
impl SoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Soe> {
        match self.bits {
            0 => Some(Soe::_0),
            1 => Some(Soe::_1),
            _ => None,
        }
    }
    #[doc = "Stops output by serial communication operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Soe::_0
    }
    #[doc = "Enables output by serial communication operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Soe::_1
    }
}
#[doc = "Field `SOE` writer - Serial output enable or stop of channel n"]
pub type SoeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Soe>;
impl<'a, REG> SoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stops output by serial communication operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Soe::_0)
    }
    #[doc = "Enables output by serial communication operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Soe::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Serial output enable or stop of channel n"]
    #[inline(always)]
    pub fn soe(&self) -> SoeR {
        SoeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial output enable or stop of channel n"]
    #[inline(always)]
    pub fn soe(&mut self) -> SoeW<Soe1Spec> {
        SoeW::new(self, 0)
    }
}
#[doc = "Serial Output Enable Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`soe1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`soe1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Soe1Spec;
impl crate::RegisterSpec for Soe1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`soe1::R`](R) reader structure"]
impl crate::Readable for Soe1Spec {}
#[doc = "`write(|w| ..)` method takes [`soe1::W`](W) writer structure"]
impl crate::Writable for Soe1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SOE1 to value 0"]
impl crate::Resettable for Soe1Spec {
    const RESET_VALUE: u16 = 0;
}
