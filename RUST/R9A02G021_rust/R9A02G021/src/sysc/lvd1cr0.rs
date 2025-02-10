#[doc = "Register `LVD1CR0` reader"]
pub type R = crate::R<Lvd1cr0Spec>;
#[doc = "Register `LVD1CR0` writer"]
pub type W = crate::W<Lvd1cr0Spec>;
#[doc = "Voltage Monitor 1 Interrupt/Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Voltage Monitor 1 Interrupt/Reset Enable"]
pub type RieR = crate::BitReader<Rie>;
impl RieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rie {
        match self.bits {
            false => Rie::_0,
            true => Rie::_1,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rie::_0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rie::_1
    }
}
#[doc = "Field `RIE` writer - Voltage Monitor 1 Interrupt/Reset Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::_1)
    }
}
#[doc = "Voltage Monitor 1 Circuit Comparison Result Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpe {
    #[doc = "0: Disable voltage monitor 1 circuit comparison result output"]
    _0 = 0,
    #[doc = "1: Enable voltage monitor 1 circuit comparison result output"]
    _1 = 1,
}
impl From<Cmpe> for bool {
    #[inline(always)]
    fn from(variant: Cmpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPE` reader - Voltage Monitor 1 Circuit Comparison Result Output Enable"]
pub type CmpeR = crate::BitReader<Cmpe>;
impl CmpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpe {
        match self.bits {
            false => Cmpe::_0,
            true => Cmpe::_1,
        }
    }
    #[doc = "Disable voltage monitor 1 circuit comparison result output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cmpe::_0
    }
    #[doc = "Enable voltage monitor 1 circuit comparison result output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cmpe::_1
    }
}
#[doc = "Field `CMPE` writer - Voltage Monitor 1 Circuit Comparison Result Output Enable"]
pub type CmpeW<'a, REG> = crate::BitWriter<'a, REG, Cmpe>;
impl<'a, REG> CmpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable voltage monitor 1 circuit comparison result output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpe::_0)
    }
    #[doc = "Enable voltage monitor 1 circuit comparison result output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpe::_1)
    }
}
#[doc = "Voltage Monitor 1 Circuit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ri {
    #[doc = "0: Generate voltage monitor 1 interrupt on Vdet1 crossing"]
    _0 = 0,
    #[doc = "1: Enable voltage monitor 1 reset when the voltage falls to and below Vdet1"]
    _1 = 1,
}
impl From<Ri> for bool {
    #[inline(always)]
    fn from(variant: Ri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RI` reader - Voltage Monitor 1 Circuit Mode Select"]
pub type RiR = crate::BitReader<Ri>;
impl RiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ri {
        match self.bits {
            false => Ri::_0,
            true => Ri::_1,
        }
    }
    #[doc = "Generate voltage monitor 1 interrupt on Vdet1 crossing"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ri::_0
    }
    #[doc = "Enable voltage monitor 1 reset when the voltage falls to and below Vdet1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ri::_1
    }
}
#[doc = "Field `RI` writer - Voltage Monitor 1 Circuit Mode Select"]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG, Ri>;
impl<'a, REG> RiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate voltage monitor 1 interrupt on Vdet1 crossing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ri::_0)
    }
    #[doc = "Enable voltage monitor 1 reset when the voltage falls to and below Vdet1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ri::_1)
    }
}
#[doc = "Voltage Monitor 1 Reset Negate Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rn {
    #[doc = "0: Negate after a stabilization time (tLVD1) from the time that VCC > Vdet1 is detected"]
    _0 = 0,
    #[doc = "1: Negate after a stabilization time (tLVD1) from the time that LVD1 reset is asserted"]
    _1 = 1,
}
impl From<Rn> for bool {
    #[inline(always)]
    fn from(variant: Rn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RN` reader - Voltage Monitor 1 Reset Negate Select"]
pub type RnR = crate::BitReader<Rn>;
impl RnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rn {
        match self.bits {
            false => Rn::_0,
            true => Rn::_1,
        }
    }
    #[doc = "Negate after a stabilization time (tLVD1) from the time that VCC > Vdet1 is detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rn::_0
    }
    #[doc = "Negate after a stabilization time (tLVD1) from the time that LVD1 reset is asserted"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rn::_1
    }
}
#[doc = "Field `RN` writer - Voltage Monitor 1 Reset Negate Select"]
pub type RnW<'a, REG> = crate::BitWriter<'a, REG, Rn>;
impl<'a, REG> RnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Negate after a stabilization time (tLVD1) from the time that VCC > Vdet1 is detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rn::_0)
    }
    #[doc = "Negate after a stabilization time (tLVD1) from the time that LVD1 reset is asserted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rn::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor 1 Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(&self) -> CmpeR {
        CmpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage Monitor 1 Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Voltage Monitor 1 Reset Negate Select"]
    #[inline(always)]
    pub fn rn(&self) -> RnR {
        RnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor 1 Interrupt/Reset Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<Lvd1cr0Spec> {
        RieW::new(self, 0)
    }
    #[doc = "Bit 2 - Voltage Monitor 1 Circuit Comparison Result Output Enable"]
    #[inline(always)]
    pub fn cmpe(&mut self) -> CmpeW<Lvd1cr0Spec> {
        CmpeW::new(self, 2)
    }
    #[doc = "Bit 6 - Voltage Monitor 1 Circuit Mode Select"]
    #[inline(always)]
    pub fn ri(&mut self) -> RiW<Lvd1cr0Spec> {
        RiW::new(self, 6)
    }
    #[doc = "Bit 7 - Voltage Monitor 1 Reset Negate Select"]
    #[inline(always)]
    pub fn rn(&mut self) -> RnW<Lvd1cr0Spec> {
        RnW::new(self, 7)
    }
}
#[doc = "Voltage Monitor 1 Circuit Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`lvd1cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lvd1cr0Spec;
impl crate::RegisterSpec for Lvd1cr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvd1cr0::R`](R) reader structure"]
impl crate::Readable for Lvd1cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`lvd1cr0::W`](W) writer structure"]
impl crate::Writable for Lvd1cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LVD1CR0 to value 0x80"]
impl crate::Resettable for Lvd1cr0Spec {
    const RESET_VALUE: u8 = 0x80;
}
