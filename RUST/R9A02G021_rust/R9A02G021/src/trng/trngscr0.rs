#[doc = "Register `TRNGSCR0` reader"]
pub type R = crate::R<Trngscr0Spec>;
#[doc = "Register `TRNGSCR0` writer"]
pub type W = crate::W<Trngscr0Spec>;
#[doc = "Trigger to start generating a random number seed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum St {
    #[doc = "0: The trigger is inactive."]
    _0 = 0,
    #[doc = "1: Starts generation of a random number seed."]
    _1 = 1,
}
impl From<St> for bool {
    #[inline(always)]
    fn from(variant: St) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ST` reader - Trigger to start generating a random number seed"]
pub type StR = crate::BitReader<St>;
impl StR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> St {
        match self.bits {
            false => St::_0,
            true => St::_1,
        }
    }
    #[doc = "The trigger is inactive."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == St::_0
    }
    #[doc = "Starts generation of a random number seed."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == St::_1
    }
}
#[doc = "Field `ST` writer - Trigger to start generating a random number seed"]
pub type StW<'a, REG> = crate::BitWriter<'a, REG, St>;
impl<'a, REG> StW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The trigger is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(St::_0)
    }
    #[doc = "Starts generation of a random number seed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(St::_1)
    }
}
#[doc = "Control over operation of the true random number generator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Stops the true random number generator."]
    _0 = 0,
    #[doc = "1: Enables the true random number generator."]
    _1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Control over operation of the true random number generator"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::_0,
            true => En::_1,
        }
    }
    #[doc = "Stops the true random number generator."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En::_0
    }
    #[doc = "Enables the true random number generator."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En::_1
    }
}
#[doc = "Field `EN` writer - Control over operation of the true random number generator"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stops the true random number generator."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En::_0)
    }
    #[doc = "Enables the true random number generator."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En::_1)
    }
}
#[doc = "Random number seed generation status flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdy {
    #[doc = "0: A random number seed has not been generated or four rounds of reading from the TRNGSDR register have been completed."]
    _0 = 0,
    #[doc = "1: A random number seed has been generated."]
    _1 = 1,
}
impl From<Rdy> for bool {
    #[inline(always)]
    fn from(variant: Rdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDY` reader - Random number seed generation status flag"]
pub type RdyR = crate::BitReader<Rdy>;
impl RdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdy {
        match self.bits {
            false => Rdy::_0,
            true => Rdy::_1,
        }
    }
    #[doc = "A random number seed has not been generated or four rounds of reading from the TRNGSDR register have been completed."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rdy::_0
    }
    #[doc = "A random number seed has been generated."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rdy::_1
    }
}
#[doc = "Field `RDY` writer - Random number seed generation status flag"]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG, Rdy>;
impl<'a, REG> RdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A random number seed has not been generated or four rounds of reading from the TRNGSDR register have been completed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdy::_0)
    }
    #[doc = "A random number seed has been generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdy::_1)
    }
}
impl R {
    #[doc = "Bit 2 - Trigger to start generating a random number seed"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control over operation of the true random number generator"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Random number seed generation status flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Trigger to start generating a random number seed"]
    #[inline(always)]
    pub fn st(&mut self) -> StW<Trngscr0Spec> {
        StW::new(self, 2)
    }
    #[doc = "Bit 3 - Control over operation of the true random number generator"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Trngscr0Spec> {
        EnW::new(self, 3)
    }
    #[doc = "Bit 7 - Random number seed generation status flag"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<Trngscr0Spec> {
        RdyW::new(self, 7)
    }
}
#[doc = "Random Number Seed Command Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`trngscr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trngscr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Trngscr0Spec;
impl crate::RegisterSpec for Trngscr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`trngscr0::R`](R) reader structure"]
impl crate::Readable for Trngscr0Spec {}
#[doc = "`write(|w| ..)` method takes [`trngscr0::W`](W) writer structure"]
impl crate::Writable for Trngscr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TRNGSCR0 to value 0"]
impl crate::Resettable for Trngscr0Spec {
    const RESET_VALUE: u8 = 0;
}
