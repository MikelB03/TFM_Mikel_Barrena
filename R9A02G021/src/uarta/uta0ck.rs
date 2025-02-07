#[doc = "Register `UTA0CK` reader"]
pub type R = crate::R<Uta0ckSpec>;
#[doc = "Register `UTA0CK` writer"]
pub type W = crate::W<Uta0ckSpec>;
#[doc = "UARTA0 operation clock select (fUTA0)\n\nValue on reset: 0"]
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
#[doc = "Field `CK` reader - UARTA0 operation clock select (fUTA0)"]
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
#[doc = "Field `CK` writer - UARTA0 operation clock select (fUTA0)"]
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
#[doc = "fSEL clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Stop"]
    _00 = 0,
    #[doc = "1: UARTAMCLK"]
    _01 = 1,
    #[doc = "2: UARTAHCLK"]
    _10 = 2,
    #[doc = "3: UARTAMOCLK"]
    _11 = 3,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - fSEL clock select"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            0 => Sel::_00,
            1 => Sel::_01,
            2 => Sel::_10,
            3 => Sel::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == Sel::_00
    }
    #[doc = "UARTAMCLK"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == Sel::_01
    }
    #[doc = "UARTAHCLK"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Sel::_10
    }
    #[doc = "UARTAMOCLK"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Sel::_11
    }
}
#[doc = "Field `SEL` writer - fSEL clock select"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sel, crate::Safe>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_00)
    }
    #[doc = "UARTAMCLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_01)
    }
    #[doc = "UARTAHCLK"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_10)
    }
    #[doc = "UARTAMOCLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::_11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Disables CLKA0 output"]
    _0 = 0,
    #[doc = "1: Enables CLKA0 output"]
    _1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - "]
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
    #[doc = "Disables CLKA0 output"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == En::_0
    }
    #[doc = "Enables CLKA0 output"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == En::_1
    }
}
#[doc = "Field `EN` writer - "]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables CLKA0 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(En::_0)
    }
    #[doc = "Enables CLKA0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(En::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - UARTA0 operation clock select (fUTA0)"]
    #[inline(always)]
    pub fn ck(&self) -> CkR {
        CkR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - fSEL clock select"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - UARTA0 operation clock select (fUTA0)"]
    #[inline(always)]
    pub fn ck(&mut self) -> CkW<Uta0ckSpec> {
        CkW::new(self, 0)
    }
    #[doc = "Bits 4:5 - fSEL clock select"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<Uta0ckSpec> {
        SelW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Uta0ckSpec> {
        EnW::new(self, 7)
    }
}
#[doc = "UARTA Clock Select Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`uta0ck::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uta0ck::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uta0ckSpec;
impl crate::RegisterSpec for Uta0ckSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uta0ck::R`](R) reader structure"]
impl crate::Readable for Uta0ckSpec {}
#[doc = "`write(|w| ..)` method takes [`uta0ck::W`](W) writer structure"]
impl crate::Writable for Uta0ckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UTA0CK to value 0"]
impl crate::Resettable for Uta0ckSpec {
    const RESET_VALUE: u8 = 0;
}
