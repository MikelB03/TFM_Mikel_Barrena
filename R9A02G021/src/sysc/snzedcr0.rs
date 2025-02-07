#[doc = "Register `SNZEDCR0` reader"]
pub type R = crate::R<Snzedcr0Spec>;
#[doc = "Register `SNZEDCR0` writer"]
pub type W = crate::W<Snzedcr0Spec>;
#[doc = "Last DTC Transmission Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtczred {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
    _1 = 1,
}
impl From<Dtczred> for bool {
    #[inline(always)]
    fn from(variant: Dtczred) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCZRED` reader - Last DTC Transmission Completion Snooze End Enable"]
pub type DtczredR = crate::BitReader<Dtczred>;
impl DtczredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtczred {
        match self.bits {
            false => Dtczred::_0,
            true => Dtczred::_1,
        }
    }
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtczred::_0
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtczred::_1
    }
}
#[doc = "Field `DTCZRED` writer - Last DTC Transmission Completion Snooze End Enable"]
pub type DtczredW<'a, REG> = crate::BitWriter<'a, REG, Dtczred>;
impl<'a, REG> DtczredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtczred::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtczred::_1)
    }
}
#[doc = "Not Last DTC Transmission Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtcnzred {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
    _1 = 1,
}
impl From<Dtcnzred> for bool {
    #[inline(always)]
    fn from(variant: Dtcnzred) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTCNZRED` reader - Not Last DTC Transmission Completion Snooze End Enable"]
pub type DtcnzredR = crate::BitReader<Dtcnzred>;
impl DtcnzredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtcnzred {
        match self.bits {
            false => Dtcnzred::_0,
            true => Dtcnzred::_1,
        }
    }
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dtcnzred::_0
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dtcnzred::_1
    }
}
#[doc = "Field `DTCNZRED` writer - Not Last DTC Transmission Completion Snooze End Enable"]
pub type DtcnzredW<'a, REG> = crate::BitWriter<'a, REG, Dtcnzred>;
impl<'a, REG> DtcnzredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcnzred::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dtcnzred::_1)
    }
}
#[doc = "ADC Compare Mismatch Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adncred {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
    _1 = 1,
}
impl From<Adncred> for bool {
    #[inline(always)]
    fn from(variant: Adncred) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADNCRED` reader - ADC Compare Mismatch Snooze End Enable"]
pub type AdncredR = crate::BitReader<Adncred>;
impl AdncredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adncred {
        match self.bits {
            false => Adncred::_0,
            true => Adncred::_1,
        }
    }
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Adncred::_0
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Adncred::_1
    }
}
#[doc = "Field `ADNCRED` writer - ADC Compare Mismatch Snooze End Enable"]
pub type AdncredW<'a, REG> = crate::BitWriter<'a, REG, Adncred>;
impl<'a, REG> AdncredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Adncred::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Adncred::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(&self) -> DtczredR {
        DtczredR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(&self) -> DtcnzredR {
        DtcnzredR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Compare Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn adncred(&self) -> AdncredR {
        AdncredR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtczred(&mut self) -> DtczredW<Snzedcr0Spec> {
        DtczredW::new(self, 1)
    }
    #[doc = "Bit 2 - Not Last DTC Transmission Completion Snooze End Enable"]
    #[inline(always)]
    pub fn dtcnzred(&mut self) -> DtcnzredW<Snzedcr0Spec> {
        DtcnzredW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC Compare Mismatch Snooze End Enable"]
    #[inline(always)]
    pub fn adncred(&mut self) -> AdncredW<Snzedcr0Spec> {
        AdncredW::new(self, 3)
    }
}
#[doc = "Snooze End Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`snzedcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snzedcr0Spec;
impl crate::RegisterSpec for Snzedcr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snzedcr0::R`](R) reader structure"]
impl crate::Readable for Snzedcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`snzedcr0::W`](W) writer structure"]
impl crate::Writable for Snzedcr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SNZEDCR0 to value 0"]
impl crate::Resettable for Snzedcr0Spec {
    const RESET_VALUE: u8 = 0;
}
