#[doc = "Register `LPOPT` reader"]
pub type R = crate::R<LpoptSpec>;
#[doc = "Register `LPOPT` writer"]
pub type W = crate::W<LpoptSpec>;
#[doc = "BPF Clock Disable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpfclkdis {
    #[doc = "0: Flash register R/W clock operates as normal"]
    _0 = 0,
    #[doc = "1: Flash register R/W clock stops"]
    _1 = 1,
}
impl From<Bpfclkdis> for bool {
    #[inline(always)]
    fn from(variant: Bpfclkdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFCLKDIS` reader - BPF Clock Disable Control"]
pub type BpfclkdisR = crate::BitReader<Bpfclkdis>;
impl BpfclkdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpfclkdis {
        match self.bits {
            false => Bpfclkdis::_0,
            true => Bpfclkdis::_1,
        }
    }
    #[doc = "Flash register R/W clock operates as normal"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Bpfclkdis::_0
    }
    #[doc = "Flash register R/W clock stops"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Bpfclkdis::_1
    }
}
#[doc = "Field `BPFCLKDIS` writer - BPF Clock Disable Control"]
pub type BpfclkdisW<'a, REG> = crate::BitWriter<'a, REG, Bpfclkdis>;
impl<'a, REG> BpfclkdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash register R/W clock operates as normal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpfclkdis::_0)
    }
    #[doc = "Flash register R/W clock stops"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpfclkdis::_1)
    }
}
#[doc = "Lower Power Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpopten {
    #[doc = "0: All lower power counter measure disable"]
    _0 = 0,
    #[doc = "1: All lower power counter measure enable"]
    _1 = 1,
}
impl From<Lpopten> for bool {
    #[inline(always)]
    fn from(variant: Lpopten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOPTEN` reader - Lower Power Operation Enable"]
pub type LpoptenR = crate::BitReader<Lpopten>;
impl LpoptenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpopten {
        match self.bits {
            false => Lpopten::_0,
            true => Lpopten::_1,
        }
    }
    #[doc = "All lower power counter measure disable"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lpopten::_0
    }
    #[doc = "All lower power counter measure enable"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lpopten::_1
    }
}
#[doc = "Field `LPOPTEN` writer - Lower Power Operation Enable"]
pub type LpoptenW<'a, REG> = crate::BitWriter<'a, REG, Lpopten>;
impl<'a, REG> LpoptenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All lower power counter measure disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpopten::_0)
    }
    #[doc = "All lower power counter measure enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpopten::_1)
    }
}
impl R {
    #[doc = "Bit 3 - BPF Clock Disable Control"]
    #[inline(always)]
    pub fn bpfclkdis(&self) -> BpfclkdisR {
        BpfclkdisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Lower Power Operation Enable"]
    #[inline(always)]
    pub fn lpopten(&self) -> LpoptenR {
        LpoptenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - BPF Clock Disable Control"]
    #[inline(always)]
    pub fn bpfclkdis(&mut self) -> BpfclkdisW<LpoptSpec> {
        BpfclkdisW::new(self, 3)
    }
    #[doc = "Bit 7 - Lower Power Operation Enable"]
    #[inline(always)]
    pub fn lpopten(&mut self) -> LpoptenW<LpoptSpec> {
        LpoptenW::new(self, 7)
    }
}
#[doc = "Lower Power Operation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpopt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpopt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpoptSpec;
impl crate::RegisterSpec for LpoptSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lpopt::R`](R) reader structure"]
impl crate::Readable for LpoptSpec {}
#[doc = "`write(|w| ..)` method takes [`lpopt::W`](W) writer structure"]
impl crate::Writable for LpoptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LPOPT to value 0x40"]
impl crate::Resettable for LpoptSpec {
    const RESET_VALUE: u8 = 0x40;
}
