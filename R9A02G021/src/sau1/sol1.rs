#[doc = "Register `SOL1` reader"]
pub type R = crate::R<Sol1Spec>;
#[doc = "Register `SOL1` writer"]
pub type W = crate::W<Sol1Spec>;
#[doc = "Selects inversion of the level of the transmit data of channel 0 in UART mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sol0 {
    #[doc = "0: Communication data is output as is"]
    _0 = 0,
    #[doc = "1: Communication data is inverted and output"]
    _1 = 1,
}
impl From<Sol0> for bool {
    #[inline(always)]
    fn from(variant: Sol0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOL0` reader - Selects inversion of the level of the transmit data of channel 0 in UART mode"]
pub type Sol0R = crate::BitReader<Sol0>;
impl Sol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sol0 {
        match self.bits {
            false => Sol0::_0,
            true => Sol0::_1,
        }
    }
    #[doc = "Communication data is output as is"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Sol0::_0
    }
    #[doc = "Communication data is inverted and output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Sol0::_1
    }
}
#[doc = "Field `SOL0` writer - Selects inversion of the level of the transmit data of channel 0 in UART mode"]
pub type Sol0W<'a, REG> = crate::BitWriter<'a, REG, Sol0>;
impl<'a, REG> Sol0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Communication data is output as is"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Sol0::_0)
    }
    #[doc = "Communication data is inverted and output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Sol0::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Selects inversion of the level of the transmit data of channel 0 in UART mode"]
    #[inline(always)]
    pub fn sol0(&self) -> Sol0R {
        Sol0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects inversion of the level of the transmit data of channel 0 in UART mode"]
    #[inline(always)]
    pub fn sol0(&mut self) -> Sol0W<Sol1Spec> {
        Sol0W::new(self, 0)
    }
}
#[doc = "Serial Output Level Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sol1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sol1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sol1Spec;
impl crate::RegisterSpec for Sol1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sol1::R`](R) reader structure"]
impl crate::Readable for Sol1Spec {}
#[doc = "`write(|w| ..)` method takes [`sol1::W`](W) writer structure"]
impl crate::Writable for Sol1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SOL1 to value 0"]
impl crate::Resettable for Sol1Spec {
    const RESET_VALUE: u16 = 0;
}
