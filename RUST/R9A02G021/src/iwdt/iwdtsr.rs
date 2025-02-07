#[doc = "Register `IWDTSR` reader"]
pub type R = crate::R<IwdtsrSpec>;
#[doc = "Register `IWDTSR` writer"]
pub type W = crate::W<IwdtsrSpec>;
#[doc = "Field `CNTVAL` reader - Down-counter Value"]
pub type CntvalR = crate::FieldReader<u16>;
#[doc = "Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Undff {
    #[doc = "0: No underflow occurred"]
    _0 = 0,
    #[doc = "1: Underflow occurred"]
    _1 = 1,
}
impl From<Undff> for bool {
    #[inline(always)]
    fn from(variant: Undff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNDFF` reader - Underflow Flag"]
pub type UndffR = crate::BitReader<Undff>;
impl UndffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Undff {
        match self.bits {
            false => Undff::_0,
            true => Undff::_1,
        }
    }
    #[doc = "No underflow occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Undff::_0
    }
    #[doc = "Underflow occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Undff::_1
    }
}
#[doc = "Field `UNDFF` writer - Underflow Flag"]
pub type UndffW<'a, REG> = crate::BitWriter<'a, REG, Undff>;
impl<'a, REG> UndffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No underflow occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Undff::_0)
    }
    #[doc = "Underflow occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Undff::_1)
    }
}
#[doc = "Refresh Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refef {
    #[doc = "0: No refresh error occurred"]
    _0 = 0,
    #[doc = "1: Refresh error occurred"]
    _1 = 1,
}
impl From<Refef> for bool {
    #[inline(always)]
    fn from(variant: Refef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFEF` reader - Refresh Error Flag"]
pub type RefefR = crate::BitReader<Refef>;
impl RefefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refef {
        match self.bits {
            false => Refef::_0,
            true => Refef::_1,
        }
    }
    #[doc = "No refresh error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Refef::_0
    }
    #[doc = "Refresh error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Refef::_1
    }
}
#[doc = "Field `REFEF` writer - Refresh Error Flag"]
pub type RefefW<'a, REG> = crate::BitWriter<'a, REG, Refef>;
impl<'a, REG> RefefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No refresh error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Refef::_0)
    }
    #[doc = "Refresh error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Refef::_1)
    }
}
impl R {
    #[doc = "Bits 0:13 - Down-counter Value"]
    #[inline(always)]
    pub fn cntval(&self) -> CntvalR {
        CntvalR::new(self.bits & 0x3fff)
    }
    #[doc = "Bit 14 - Underflow Flag"]
    #[inline(always)]
    pub fn undff(&self) -> UndffR {
        UndffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Refresh Error Flag"]
    #[inline(always)]
    pub fn refef(&self) -> RefefR {
        RefefR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Underflow Flag"]
    #[inline(always)]
    pub fn undff(&mut self) -> UndffW<IwdtsrSpec> {
        UndffW::new(self, 14)
    }
    #[doc = "Bit 15 - Refresh Error Flag"]
    #[inline(always)]
    pub fn refef(&mut self) -> RefefW<IwdtsrSpec> {
        RefefW::new(self, 15)
    }
}
#[doc = "IWDT Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iwdtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IwdtsrSpec;
impl crate::RegisterSpec for IwdtsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`iwdtsr::R`](R) reader structure"]
impl crate::Readable for IwdtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`iwdtsr::W`](W) writer structure"]
impl crate::Writable for IwdtsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets IWDTSR to value 0"]
impl crate::Resettable for IwdtsrSpec {
    const RESET_VALUE: u16 = 0;
}
