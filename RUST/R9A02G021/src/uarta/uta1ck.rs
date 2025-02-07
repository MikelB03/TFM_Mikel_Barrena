#[doc = "Register `UTA1CK` reader"]
pub type R = crate::R<Uta1ckSpec>;
#[doc = "Register `UTA1CK` writer"]
pub type W = crate::W<Uta1ckSpec>;
#[doc = "UARTA1 operation clock select (fUTA1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ck {
    #[doc = "0: fSEL"]
    _0x0 = 0,
    #[doc = "1: fSEL/2"]
    _0x1 = 1,
    #[doc = "2: fSEL/4"]
    _0x2 = 2,
    #[doc = "3: fSEL/8"]
    _0x3 = 3,
    #[doc = "4: fSEL/16"]
    _0x4 = 4,
    #[doc = "5: fSEL/32"]
    _0x5 = 5,
    #[doc = "6: fSEL/64"]
    _0x6 = 6,
    #[doc = "8: UARTALCLK/UARTASCLK"]
    _0x8 = 8,
    #[doc = "7: Setting prohibited"]
    Others = 7,
}
impl From<Ck> for u8 {
    #[inline(always)]
    fn from(variant: Ck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ck {
    type Ux = u8;
}
impl crate::IsEnum for Ck {}
#[doc = "Field `CK` reader - UARTA1 operation clock select (fUTA1)"]
pub type CkR = crate::FieldReader<Ck>;
impl CkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ck {
        match self.bits {
            0 => Ck::_0x0,
            1 => Ck::_0x1,
            2 => Ck::_0x2,
            3 => Ck::_0x3,
            4 => Ck::_0x4,
            5 => Ck::_0x5,
            6 => Ck::_0x6,
            8 => Ck::_0x8,
            _ => Ck::Others,
        }
    }
    #[doc = "fSEL"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == Ck::_0x0
    }
    #[doc = "fSEL/2"]
    #[inline(always)]
    pub fn is_0x1(&self) -> bool {
        *self == Ck::_0x1
    }
    #[doc = "fSEL/4"]
    #[inline(always)]
    pub fn is_0x2(&self) -> bool {
        *self == Ck::_0x2
    }
    #[doc = "fSEL/8"]
    #[inline(always)]
    pub fn is_0x3(&self) -> bool {
        *self == Ck::_0x3
    }
    #[doc = "fSEL/16"]
    #[inline(always)]
    pub fn is_0x4(&self) -> bool {
        *self == Ck::_0x4
    }
    #[doc = "fSEL/32"]
    #[inline(always)]
    pub fn is_0x5(&self) -> bool {
        *self == Ck::_0x5
    }
    #[doc = "fSEL/64"]
    #[inline(always)]
    pub fn is_0x6(&self) -> bool {
        *self == Ck::_0x6
    }
    #[doc = "UARTALCLK/UARTASCLK"]
    #[inline(always)]
    pub fn is_0x8(&self) -> bool {
        *self == Ck::_0x8
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_others(&self) -> bool {
        matches!(self.variant(), Ck::Others)
    }
}
#[doc = "Field `CK` writer - UARTA1 operation clock select (fUTA1)"]
pub type CkW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ck, crate::Safe>;
impl<'a, REG> CkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fSEL"]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::_0x0)
    }
    #[doc = "fSEL/2"]
    #[inline(always)]
    pub fn _0x1(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::_0x1)
    }
    #[doc = "fSEL/4"]
    #[inline(always)]
    pub fn _0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::_0x2)
    }
    #[doc = "fSEL/8"]
    #[inline(always)]
    pub fn _0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::_0x3)
    }
    #[doc = "fSEL/16"]
    #[inline(always)]
    pub fn _0x4(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::_0x4)
    }
    #[doc = "fSEL/32"]
    #[inline(always)]
    pub fn _0x5(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::_0x5)
    }
    #[doc = "fSEL/64"]
    #[inline(always)]
    pub fn _0x6(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::_0x6)
    }
    #[doc = "UARTALCLK/UARTASCLK"]
    #[inline(always)]
    pub fn _0x8(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::_0x8)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn others(self) -> &'a mut crate::W<REG> {
        self.variant(Ck::Others)
    }
}
#[doc = "UARTA1 clock output function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disables CLKA1 output"]
    _0 = 0,
    #[doc = "1: Enables CLKA1 output"]
    _1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - UARTA1 clock output function enable"]
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
    #[doc = "Disables CLKA1 output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En::_0
    }
    #[doc = "Enables CLKA1 output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En::_1
    }
}
#[doc = "Field `EN` writer - UARTA1 clock output function enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables CLKA1 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En::_0)
    }
    #[doc = "Enables CLKA1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - UARTA1 operation clock select (fUTA1)"]
    #[inline(always)]
    pub fn ck(&self) -> CkR {
        CkR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 7 - UARTA1 clock output function enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - UARTA1 operation clock select (fUTA1)"]
    #[inline(always)]
    pub fn ck(&mut self) -> CkW<Uta1ckSpec> {
        CkW::new(self, 0)
    }
    #[doc = "Bit 7 - UARTA1 clock output function enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Uta1ckSpec> {
        EnW::new(self, 7)
    }
}
#[doc = "UARTA Clock Select Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`uta1ck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uta1ck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uta1ckSpec;
impl crate::RegisterSpec for Uta1ckSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uta1ck::R`](R) reader structure"]
impl crate::Readable for Uta1ckSpec {}
#[doc = "`write(|w| ..)` method takes [`uta1ck::W`](W) writer structure"]
impl crate::Writable for Uta1ckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UTA1CK to value 0"]
impl crate::Resettable for Uta1ckSpec {
    const RESET_VALUE: u8 = 0;
}
