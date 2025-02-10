#[doc = "Register `SS1` reader"]
pub type R = crate::R<Ss1Spec>;
#[doc = "Register `SS1` writer"]
pub type W = crate::W<Ss1Spec>;
#[doc = "Operation start trigger of channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ss {
    #[doc = "0: No trigger operation"]
    _0 = 0,
    #[doc = "1: Set the SE1.SE\\[n\\]
bit to 1 to place the channel in communications waiting state"]
    _1 = 1,
}
impl From<Ss> for u8 {
    #[inline(always)]
    fn from(variant: Ss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ss {
    type Ux = u8;
}
impl crate::IsEnum for Ss {}
#[doc = "Field `SS` reader - Operation start trigger of channel n"]
pub type SsR = crate::FieldReader<Ss>;
impl SsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ss> {
        match self.bits {
            0 => Some(Ss::_0),
            1 => Some(Ss::_1),
            _ => None,
        }
    }
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ss::_0
    }
    #[doc = "Set the SE1.SE\\[n\\]
bit to 1 to place the channel in communications waiting state"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ss::_1
    }
}
#[doc = "Field `SS` writer - Operation start trigger of channel n"]
pub type SsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ss>;
impl<'a, REG> SsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No trigger operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ss::_0)
    }
    #[doc = "Set the SE1.SE\\[n\\]
bit to 1 to place the channel in communications waiting state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ss::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation start trigger of channel n"]
    #[inline(always)]
    pub fn ss(&self) -> SsR {
        SsR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation start trigger of channel n"]
    #[inline(always)]
    pub fn ss(&mut self) -> SsW<Ss1Spec> {
        SsW::new(self, 0)
    }
}
#[doc = "Serial Channel Start Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ss1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ss1Spec;
impl crate::RegisterSpec for Ss1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ss1::R`](R) reader structure"]
impl crate::Readable for Ss1Spec {}
#[doc = "`write(|w| ..)` method takes [`ss1::W`](W) writer structure"]
impl crate::Writable for Ss1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SS1 to value 0"]
impl crate::Resettable for Ss1Spec {
    const RESET_VALUE: u16 = 0;
}
