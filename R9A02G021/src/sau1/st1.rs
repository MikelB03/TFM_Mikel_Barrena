#[doc = "Register `ST1` reader"]
pub type R = crate::R<St1Spec>;
#[doc = "Register `ST1` writer"]
pub type W = crate::W<St1Spec>;
#[doc = "Operation stop trigger of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum St {
    #[doc = "0: No trigger operation"]
    _0 = 0,
    #[doc = "1: Clears the SE1.SE\\[n\\]
bit to 0 and stops the communication operation"]
    _1 = 1,
}
impl From<St> for u8 {
    #[inline(always)]
    fn from(variant: St) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for St {
    type Ux = u8;
}
impl crate::IsEnum for St {}
#[doc = "Field `ST` reader - Operation stop trigger of channel n"]
pub type StR = crate::FieldReader<St>;
impl StR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<St> {
        match self.bits {
            0 => Some(St::_0),
            1 => Some(St::_1),
            _ => None,
        }
    }
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == St::_0
    }
    #[doc = "Clears the SE1.SE\\[n\\]
bit to 0 and stops the communication operation"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == St::_1
    }
}
#[doc = "Field `ST` writer - Operation stop trigger of channel n"]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 2, St>;
impl<'a, REG> StW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(St::_0)
    }
    #[doc = "Clears the SE1.SE\\[n\\]
bit to 0 and stops the communication operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(St::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation stop trigger of channel n"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation stop trigger of channel n"]
    #[inline(always)]
    pub fn st(&mut self) -> StW<St1Spec> {
        StW::new(self, 0)
    }
}
#[doc = "Serial Channel Stop Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`st1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`st1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct St1Spec;
impl crate::RegisterSpec for St1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`st1::R`](R) reader structure"]
impl crate::Readable for St1Spec {}
#[doc = "`write(|w| ..)` method takes [`st1::W`](W) writer structure"]
impl crate::Writable for St1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ST1 to value 0"]
impl crate::Resettable for St1Spec {
    const RESET_VALUE: u16 = 0;
}
