#[doc = "Register `IC` writer"]
pub type W = crate::W<IcSpec>;
#[doc = "Field `CLR0` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR1` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR2` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR3` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR4` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR5` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR6` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR7` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR8` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR9` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR10` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR11` writer - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
pub type Clr11W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr0(&mut self) -> Clr0W<IcSpec> {
        Clr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr1(&mut self) -> Clr1W<IcSpec> {
        Clr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr2(&mut self) -> Clr2W<IcSpec> {
        Clr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr3(&mut self) -> Clr3W<IcSpec> {
        Clr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr4(&mut self) -> Clr4W<IcSpec> {
        Clr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr5(&mut self) -> Clr5W<IcSpec> {
        Clr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr6(&mut self) -> Clr6W<IcSpec> {
        Clr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr7(&mut self) -> Clr7W<IcSpec> {
        Clr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr8(&mut self) -> Clr8W<IcSpec> {
        Clr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr9(&mut self) -> Clr9W<IcSpec> {
        Clr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr10(&mut self) -> Clr10W<IcSpec> {
        Clr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects interrupt on pin x to be cleared (x = 0 to 11). Clears the interrupt edge detection logic. This register is write-only. The synchronizer between the GPIO and the NVIC blocks causes a delay of 2 clocks. It is recommended to add two NOPs after the clear of the interrupt edge detection logic before the exit of the interrupt service routine. 0 = No effect. 1 = Clears edge detection logic for pin PIOn_x."]
    #[inline(always)]
    #[must_use]
    pub fn clr11(&mut self) -> Clr11W<IcSpec> {
        Clr11W::new(self, 11)
    }
}
#[doc = "Interrupt clear register for port n\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSpec;
impl crate::RegisterSpec for IcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ic::W`](W) writer structure"]
impl crate::Writable for IcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IcSpec {
    const RESET_VALUE: u32 = 0;
}
