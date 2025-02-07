#[doc = "Register `SO1` reader"]
pub type R = crate::R<So1Spec>;
#[doc = "Register `SO1` writer"]
pub type W = crate::W<So1Spec>;
#[doc = "Serial data output of channel n\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum So {
    #[doc = "0: Serial data output value is 0"]
    _0 = 0,
    #[doc = "1: Serial data output value is 1"]
    _1 = 1,
}
impl From<So> for u8 {
    #[inline(always)]
    fn from(variant: So) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for So {
    type Ux = u8;
}
impl crate::IsEnum for So {}
#[doc = "Field `SO` reader - Serial data output of channel n"]
pub type SoR = crate::FieldReader<So>;
impl SoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<So> {
        match self.bits {
            0 => Some(So::_0),
            1 => Some(So::_1),
            _ => None,
        }
    }
    #[doc = "Serial data output value is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == So::_0
    }
    #[doc = "Serial data output value is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == So::_1
    }
}
#[doc = "Field `SO` writer - Serial data output of channel n"]
pub type SoW<'a, REG> = crate::FieldWriter<'a, REG, 2, So>;
impl<'a, REG> SoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial data output value is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(So::_0)
    }
    #[doc = "Serial data output value is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(So::_1)
    }
}
#[doc = "Serial clock output of channel n\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cko {
    #[doc = "0: Serial clock output value is 0"]
    _0 = 0,
    #[doc = "1: Serial clock output value is 1"]
    _1 = 1,
}
impl From<Cko> for u8 {
    #[inline(always)]
    fn from(variant: Cko) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cko {
    type Ux = u8;
}
impl crate::IsEnum for Cko {}
#[doc = "Field `CKO` reader - Serial clock output of channel n"]
pub type CkoR = crate::FieldReader<Cko>;
impl CkoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cko> {
        match self.bits {
            0 => Some(Cko::_0),
            1 => Some(Cko::_1),
            _ => None,
        }
    }
    #[doc = "Serial clock output value is 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cko::_0
    }
    #[doc = "Serial clock output value is 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cko::_1
    }
}
#[doc = "Field `CKO` writer - Serial clock output of channel n"]
pub type CkoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cko>;
impl<'a, REG> CkoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Serial clock output value is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cko::_0)
    }
    #[doc = "Serial clock output value is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cko::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Serial data output of channel n"]
    #[inline(always)]
    pub fn so(&self) -> SoR {
        SoR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Serial clock output of channel n"]
    #[inline(always)]
    pub fn cko(&self) -> CkoR {
        CkoR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Serial data output of channel n"]
    #[inline(always)]
    pub fn so(&mut self) -> SoW<So1Spec> {
        SoW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Serial clock output of channel n"]
    #[inline(always)]
    pub fn cko(&mut self) -> CkoW<So1Spec> {
        CkoW::new(self, 8)
    }
}
#[doc = "Serial Output Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`so1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`so1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct So1Spec;
impl crate::RegisterSpec for So1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`so1::R`](R) reader structure"]
impl crate::Readable for So1Spec {}
#[doc = "`write(|w| ..)` method takes [`so1::W`](W) writer structure"]
impl crate::Writable for So1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SO1 to value 0x0f0f"]
impl crate::Resettable for So1Spec {
    const RESET_VALUE: u16 = 0x0f0f;
}
