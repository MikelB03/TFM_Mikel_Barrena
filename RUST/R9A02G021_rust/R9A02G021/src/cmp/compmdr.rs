#[doc = "Register `COMPMDR` reader"]
pub type R = crate::R<CompmdrSpec>;
#[doc = "Register `COMPMDR` writer"]
pub type W = crate::W<CompmdrSpec>;
#[doc = "Comparator 0 operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0enb {
    #[doc = "0: Comparator 0 operation disabled"]
    _0 = 0,
    #[doc = "1: Comparator 0 operation enabled"]
    _1 = 1,
}
impl From<C0enb> for bool {
    #[inline(always)]
    fn from(variant: C0enb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0ENB` reader - Comparator 0 operation enable"]
pub type C0enbR = crate::BitReader<C0enb>;
impl C0enbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0enb {
        match self.bits {
            false => C0enb::_0,
            true => C0enb::_1,
        }
    }
    #[doc = "Comparator 0 operation disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0enb::_0
    }
    #[doc = "Comparator 0 operation enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0enb::_1
    }
}
#[doc = "Field `C0ENB` writer - Comparator 0 operation enable"]
pub type C0enbW<'a, REG> = crate::BitWriter<'a, REG, C0enb>;
impl<'a, REG> C0enbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 0 operation disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0enb::_0)
    }
    #[doc = "Comparator 0 operation enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0enb::_1)
    }
}
#[doc = "Selection of comparator 0 reference voltage range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0lvl {
    #[doc = "0: 0 to VCC - 1.4 V"]
    _0 = 0,
    #[doc = "1: 1.4 V to VCC"]
    _1 = 1,
}
impl From<C0lvl> for bool {
    #[inline(always)]
    fn from(variant: C0lvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0LVL` reader - Selection of comparator 0 reference voltage range"]
pub type C0lvlR = crate::BitReader<C0lvl>;
impl C0lvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0lvl {
        match self.bits {
            false => C0lvl::_0,
            true => C0lvl::_1,
        }
    }
    #[doc = "0 to VCC - 1.4 V"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0lvl::_0
    }
    #[doc = "1.4 V to VCC"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0lvl::_1
    }
}
#[doc = "Field `C0LVL` writer - Selection of comparator 0 reference voltage range"]
pub type C0lvlW<'a, REG> = crate::BitWriter<'a, REG, C0lvl>;
impl<'a, REG> C0lvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0 to VCC - 1.4 V"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0lvl::_0)
    }
    #[doc = "1.4 V to VCC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0lvl::_1)
    }
}
#[doc = "Selection of comparator 0 reference voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0vrf {
    #[doc = "0: Supply through the IVREF0 pin"]
    _0 = 0,
    #[doc = "1: Supply through the internal reference voltage"]
    _1 = 1,
}
impl From<C0vrf> for bool {
    #[inline(always)]
    fn from(variant: C0vrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0VRF` reader - Selection of comparator 0 reference voltage"]
pub type C0vrfR = crate::BitReader<C0vrf>;
impl C0vrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0vrf {
        match self.bits {
            false => C0vrf::_0,
            true => C0vrf::_1,
        }
    }
    #[doc = "Supply through the IVREF0 pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0vrf::_0
    }
    #[doc = "Supply through the internal reference voltage"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0vrf::_1
    }
}
#[doc = "Field `C0VRF` writer - Selection of comparator 0 reference voltage"]
pub type C0vrfW<'a, REG> = crate::BitWriter<'a, REG, C0vrf>;
impl<'a, REG> C0vrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Supply through the IVREF0 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C0vrf::_0)
    }
    #[doc = "Supply through the internal reference voltage"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C0vrf::_1)
    }
}
#[doc = "Comparator 0 monitor flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0mon {
    #[doc = "0: IVCMP0 < comparator 0 reference voltage (IVREF0 or internal reference voltage)"]
    _0 = 0,
    #[doc = "1: IVCMP0 > comparator 0 reference voltage (IVREF0 or internal reference voltage)"]
    _1 = 1,
}
impl From<C0mon> for bool {
    #[inline(always)]
    fn from(variant: C0mon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0MON` reader - Comparator 0 monitor flag"]
pub type C0monR = crate::BitReader<C0mon>;
impl C0monR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C0mon {
        match self.bits {
            false => C0mon::_0,
            true => C0mon::_1,
        }
    }
    #[doc = "IVCMP0 < comparator 0 reference voltage (IVREF0 or internal reference voltage)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C0mon::_0
    }
    #[doc = "IVCMP0 > comparator 0 reference voltage (IVREF0 or internal reference voltage)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C0mon::_1
    }
}
#[doc = "Comparator 1 operation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1enb {
    #[doc = "0: Comparator 1 operation disabled"]
    _0 = 0,
    #[doc = "1: Comparator 1 operation enabled"]
    _1 = 1,
}
impl From<C1enb> for bool {
    #[inline(always)]
    fn from(variant: C1enb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1ENB` reader - Comparator 1 operation enable"]
pub type C1enbR = crate::BitReader<C1enb>;
impl C1enbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1enb {
        match self.bits {
            false => C1enb::_0,
            true => C1enb::_1,
        }
    }
    #[doc = "Comparator 1 operation disabled"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1enb::_0
    }
    #[doc = "Comparator 1 operation enabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1enb::_1
    }
}
#[doc = "Field `C1ENB` writer - Comparator 1 operation enable"]
pub type C1enbW<'a, REG> = crate::BitWriter<'a, REG, C1enb>;
impl<'a, REG> C1enbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Comparator 1 operation disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1enb::_0)
    }
    #[doc = "Comparator 1 operation enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1enb::_1)
    }
}
#[doc = "Selection of comparator 1 reference voltage range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1lvl {
    #[doc = "0: 0 to VCC - 1.4 V"]
    _0 = 0,
    #[doc = "1: 1.4 V to VCC"]
    _1 = 1,
}
impl From<C1lvl> for bool {
    #[inline(always)]
    fn from(variant: C1lvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1LVL` reader - Selection of comparator 1 reference voltage range"]
pub type C1lvlR = crate::BitReader<C1lvl>;
impl C1lvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1lvl {
        match self.bits {
            false => C1lvl::_0,
            true => C1lvl::_1,
        }
    }
    #[doc = "0 to VCC - 1.4 V"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1lvl::_0
    }
    #[doc = "1.4 V to VCC"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1lvl::_1
    }
}
#[doc = "Field `C1LVL` writer - Selection of comparator 1 reference voltage range"]
pub type C1lvlW<'a, REG> = crate::BitWriter<'a, REG, C1lvl>;
impl<'a, REG> C1lvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0 to VCC - 1.4 V"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1lvl::_0)
    }
    #[doc = "1.4 V to VCC"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1lvl::_1)
    }
}
#[doc = "Selection of comparator 1 reference voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1vrf {
    #[doc = "0: Supply through the IVREF1 pin"]
    _0 = 0,
    #[doc = "1: Supply through D/A converter output 0"]
    _1 = 1,
}
impl From<C1vrf> for bool {
    #[inline(always)]
    fn from(variant: C1vrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1VRF` reader - Selection of comparator 1 reference voltage"]
pub type C1vrfR = crate::BitReader<C1vrf>;
impl C1vrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1vrf {
        match self.bits {
            false => C1vrf::_0,
            true => C1vrf::_1,
        }
    }
    #[doc = "Supply through the IVREF1 pin"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1vrf::_0
    }
    #[doc = "Supply through D/A converter output 0"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1vrf::_1
    }
}
#[doc = "Field `C1VRF` writer - Selection of comparator 1 reference voltage"]
pub type C1vrfW<'a, REG> = crate::BitWriter<'a, REG, C1vrf>;
impl<'a, REG> C1vrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Supply through the IVREF1 pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(C1vrf::_0)
    }
    #[doc = "Supply through D/A converter output 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(C1vrf::_1)
    }
}
#[doc = "Comparator 1 monitor flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1mon {
    #[doc = "0: IVCMP1 < comparator 1 reference voltage (IVREF1 or D/A converter output 0)"]
    _0 = 0,
    #[doc = "1: IVCMP1 > comparator 1 reference voltage (IVREF1 or D/A converter output 0)"]
    _1 = 1,
}
impl From<C1mon> for bool {
    #[inline(always)]
    fn from(variant: C1mon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1MON` reader - Comparator 1 monitor flag"]
pub type C1monR = crate::BitReader<C1mon>;
impl C1monR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> C1mon {
        match self.bits {
            false => C1mon::_0,
            true => C1mon::_1,
        }
    }
    #[doc = "IVCMP1 < comparator 1 reference voltage (IVREF1 or D/A converter output 0)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == C1mon::_0
    }
    #[doc = "IVCMP1 > comparator 1 reference voltage (IVREF1 or D/A converter output 0)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == C1mon::_1
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 operation enable"]
    #[inline(always)]
    pub fn c0enb(&self) -> C0enbR {
        C0enbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selection of comparator 0 reference voltage range"]
    #[inline(always)]
    pub fn c0lvl(&self) -> C0lvlR {
        C0lvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selection of comparator 0 reference voltage"]
    #[inline(always)]
    pub fn c0vrf(&self) -> C0vrfR {
        C0vrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 0 monitor flag"]
    #[inline(always)]
    pub fn c0mon(&self) -> C0monR {
        C0monR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator 1 operation enable"]
    #[inline(always)]
    pub fn c1enb(&self) -> C1enbR {
        C1enbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selection of comparator 1 reference voltage range"]
    #[inline(always)]
    pub fn c1lvl(&self) -> C1lvlR {
        C1lvlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selection of comparator 1 reference voltage"]
    #[inline(always)]
    pub fn c1vrf(&self) -> C1vrfR {
        C1vrfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 monitor flag"]
    #[inline(always)]
    pub fn c1mon(&self) -> C1monR {
        C1monR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 operation enable"]
    #[inline(always)]
    pub fn c0enb(&mut self) -> C0enbW<CompmdrSpec> {
        C0enbW::new(self, 0)
    }
    #[doc = "Bit 1 - Selection of comparator 0 reference voltage range"]
    #[inline(always)]
    pub fn c0lvl(&mut self) -> C0lvlW<CompmdrSpec> {
        C0lvlW::new(self, 1)
    }
    #[doc = "Bit 2 - Selection of comparator 0 reference voltage"]
    #[inline(always)]
    pub fn c0vrf(&mut self) -> C0vrfW<CompmdrSpec> {
        C0vrfW::new(self, 2)
    }
    #[doc = "Bit 4 - Comparator 1 operation enable"]
    #[inline(always)]
    pub fn c1enb(&mut self) -> C1enbW<CompmdrSpec> {
        C1enbW::new(self, 4)
    }
    #[doc = "Bit 5 - Selection of comparator 1 reference voltage range"]
    #[inline(always)]
    pub fn c1lvl(&mut self) -> C1lvlW<CompmdrSpec> {
        C1lvlW::new(self, 5)
    }
    #[doc = "Bit 6 - Selection of comparator 1 reference voltage"]
    #[inline(always)]
    pub fn c1vrf(&mut self) -> C1vrfW<CompmdrSpec> {
        C1vrfW::new(self, 6)
    }
}
#[doc = "Comparator Mode Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`compmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompmdrSpec;
impl crate::RegisterSpec for CompmdrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`compmdr::R`](R) reader structure"]
impl crate::Readable for CompmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`compmdr::W`](W) writer structure"]
impl crate::Writable for CompmdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets COMPMDR to value 0"]
impl crate::Resettable for CompmdrSpec {
    const RESET_VALUE: u8 = 0;
}
