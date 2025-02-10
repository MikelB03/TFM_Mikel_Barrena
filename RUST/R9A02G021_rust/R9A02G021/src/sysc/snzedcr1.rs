#[doc = "Register `SNZEDCR1` reader"]
pub type R = crate::R<Snzedcr1Spec>;
#[doc = "Register `SNZEDCR1` writer"]
pub type W = crate::W<Snzedcr1Spec>;
#[doc = "TML32 Interrupt Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tmloied {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
    _1 = 1,
}
impl From<Tmloied> for bool {
    #[inline(always)]
    fn from(variant: Tmloied) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMLOIED` reader - TML32 Interrupt Snooze End Enable"]
pub type TmloiedR = crate::BitReader<Tmloied>;
impl TmloiedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tmloied {
        match self.bits {
            false => Tmloied::_0,
            true => Tmloied::_1,
        }
    }
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tmloied::_0
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tmloied::_1
    }
}
#[doc = "Field `TMLOIED` writer - TML32 Interrupt Snooze End Enable"]
pub type TmloiedW<'a, REG> = crate::BitWriter<'a, REG, Tmloied>;
impl<'a, REG> TmloiedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tmloied::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tmloied::_1)
    }
}
#[doc = "REMC No-Interrupt Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Remcncred {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
    _1 = 1,
}
impl From<Remcncred> for bool {
    #[inline(always)]
    fn from(variant: Remcncred) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REMCNCRED` reader - REMC No-Interrupt Completion Snooze End Enable"]
pub type RemcncredR = crate::BitReader<Remcncred>;
impl RemcncredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Remcncred {
        match self.bits {
            false => Remcncred::_0,
            true => Remcncred::_1,
        }
    }
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Remcncred::_0
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Remcncred::_1
    }
}
#[doc = "Field `REMCNCRED` writer - REMC No-Interrupt Completion Snooze End Enable"]
pub type RemcncredW<'a, REG> = crate::BitWriter<'a, REG, Remcncred>;
impl<'a, REG> RemcncredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Remcncred::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Remcncred::_1)
    }
}
#[doc = "SAU0 No-Interrupt Error Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sau0ncred {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
    _1 = 1,
}
impl From<Sau0ncred> for bool {
    #[inline(always)]
    fn from(variant: Sau0ncred) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAU0NCRED` reader - SAU0 No-Interrupt Error Completion Snooze End Enable"]
pub type Sau0ncredR = crate::BitReader<Sau0ncred>;
impl Sau0ncredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sau0ncred {
        match self.bits {
            false => Sau0ncred::_0,
            true => Sau0ncred::_1,
        }
    }
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sau0ncred::_0
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sau0ncred::_1
    }
}
#[doc = "Field `SAU0NCRED` writer - SAU0 No-Interrupt Error Completion Snooze End Enable"]
pub type Sau0ncredW<'a, REG> = crate::BitWriter<'a, REG, Sau0ncred>;
impl<'a, REG> Sau0ncredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sau0ncred::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sau0ncred::_1)
    }
}
#[doc = "SAU1 No-Interrupt Error Completion Snooze End Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sau1ncred {
    #[doc = "0: Disable the snooze end request"]
    _0 = 0,
    #[doc = "1: Enable the snooze end request"]
    _1 = 1,
}
impl From<Sau1ncred> for bool {
    #[inline(always)]
    fn from(variant: Sau1ncred) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAU1NCRED` reader - SAU1 No-Interrupt Error Completion Snooze End Enable"]
pub type Sau1ncredR = crate::BitReader<Sau1ncred>;
impl Sau1ncredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sau1ncred {
        match self.bits {
            false => Sau1ncred::_0,
            true => Sau1ncred::_1,
        }
    }
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sau1ncred::_0
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sau1ncred::_1
    }
}
#[doc = "Field `SAU1NCRED` writer - SAU1 No-Interrupt Error Completion Snooze End Enable"]
pub type Sau1ncredW<'a, REG> = crate::BitWriter<'a, REG, Sau1ncred>;
impl<'a, REG> Sau1ncredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the snooze end request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sau1ncred::_0)
    }
    #[doc = "Enable the snooze end request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sau1ncred::_1)
    }
}
impl R {
    #[doc = "Bit 0 - TML32 Interrupt Snooze End Enable"]
    #[inline(always)]
    pub fn tmloied(&self) -> TmloiedR {
        TmloiedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REMC No-Interrupt Completion Snooze End Enable"]
    #[inline(always)]
    pub fn remcncred(&self) -> RemcncredR {
        RemcncredR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - SAU0 No-Interrupt Error Completion Snooze End Enable"]
    #[inline(always)]
    pub fn sau0ncred(&self) -> Sau0ncredR {
        Sau0ncredR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAU1 No-Interrupt Error Completion Snooze End Enable"]
    #[inline(always)]
    pub fn sau1ncred(&self) -> Sau1ncredR {
        Sau1ncredR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TML32 Interrupt Snooze End Enable"]
    #[inline(always)]
    pub fn tmloied(&mut self) -> TmloiedW<Snzedcr1Spec> {
        TmloiedW::new(self, 0)
    }
    #[doc = "Bit 1 - REMC No-Interrupt Completion Snooze End Enable"]
    #[inline(always)]
    pub fn remcncred(&mut self) -> RemcncredW<Snzedcr1Spec> {
        RemcncredW::new(self, 1)
    }
    #[doc = "Bit 4 - SAU0 No-Interrupt Error Completion Snooze End Enable"]
    #[inline(always)]
    pub fn sau0ncred(&mut self) -> Sau0ncredW<Snzedcr1Spec> {
        Sau0ncredW::new(self, 4)
    }
    #[doc = "Bit 5 - SAU1 No-Interrupt Error Completion Snooze End Enable"]
    #[inline(always)]
    pub fn sau1ncred(&mut self) -> Sau1ncredW<Snzedcr1Spec> {
        Sau1ncredW::new(self, 5)
    }
}
#[doc = "Snooze End Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`snzedcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Snzedcr1Spec;
impl crate::RegisterSpec for Snzedcr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snzedcr1::R`](R) reader structure"]
impl crate::Readable for Snzedcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`snzedcr1::W`](W) writer structure"]
impl crate::Writable for Snzedcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SNZEDCR1 to value 0"]
impl crate::Resettable for Snzedcr1Spec {
    const RESET_VALUE: u8 = 0;
}
