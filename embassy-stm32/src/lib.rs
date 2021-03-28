#![no_std]
#![feature(generic_associated_types)]
#![feature(asm)]
#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
pub use {stm32f4xx_hal as hal, stm32f4xx_hal::stm32 as pac};

#[cfg(any(feature = "stm32l0x1", feature = "stm32l0x2", feature = "stm32l0x3",))]
pub use {stm32l0xx_hal as hal, stm32l0xx_hal::pac};

pub mod fmt;

pub mod exti;
pub mod interrupt;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
pub mod can;

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
pub mod rtc;

unsafe impl embassy_extras::usb::USBInterrupt for interrupt::OTG_FS {}

use core::option::Option;
use hal::prelude::*;
use hal::rcc::Clocks;

macro_rules! peripherals {
    ($($PER:ident,)+) => {
        #[doc = r"All the peripherals"]
        #[allow(non_snake_case)]
        pub struct Peripherals {
            $(
                pub $PER: pac::$PER,
            )+
        }

        static mut GLOBAL_PERIPHERALS: Option<(Peripherals, Clocks)> = None;

        impl Peripherals {
            pub fn take() -> Option<(Peripherals, Clocks)> {
                unsafe { GLOBAL_PERIPHERALS.take() }
            }

            pub unsafe fn set_peripherals(clocks: Clocks) {
                let dp = pac::Peripherals::steal();
                let peripherals = Peripherals {
                    $(
                        $PER: dp.$PER,
                    )+
                };

                GLOBAL_PERIPHERALS.replace((peripherals, clocks));
            }
        }
    };
}

#[cfg(feature = "stm32f446")]
peripherals! {
    DCMI,
    FMC,
    DBGMCU,
    DMA2,
    DMA1,
//    RCC,
    GPIOH,
    GPIOG,
    GPIOF,
    GPIOE,
    GPIOD,
    GPIOC,
    GPIOB,
    GPIOA,
    SYSCFG,
    SPI1,
    SPI2,
    SPI3,
    SPI4,
    ADC1,
    ADC2,
    ADC3,
    USART6,
    USART1,
    USART2,
    USART3,
    DAC,
    I2C3,
    I2C2,
    I2C1,
    IWDG,
    WWDG,
    RTC,
    UART4,
    UART5,
    ADC_COMMON,
    TIM1,
    TIM2,
    TIM8,
//    TIM3,
    TIM4,
    TIM5,
    TIM9,
    TIM12,
    TIM10,
    TIM13,
    TIM14,
    TIM11,
    TIM6,
    TIM7,
    CRC,
    OTG_FS_GLOBAL,
    OTG_FS_HOST,
    OTG_FS_DEVICE,
    OTG_FS_PWRCLK,
    CAN1,
    CAN2,
    FLASH,
    EXTI,
    OTG_HS_GLOBAL,
    OTG_HS_HOST,
    OTG_HS_DEVICE,
    OTG_HS_PWRCLK,
    SAI1,
    SAI2,
    PWR,
    QUADSPI,
    SPDIFRX,
    SDMMC,
    HDMI_CEC,
    FPU,
    STK,
    NVIC_STIR,
    FPU_CPACR,
    SCB_ACTRL,
}

#[cfg(feature = "stm32f405")]
peripherals! {
    RNG,
    DCMI,
    FSMC,
    DBGMCU,
    DMA2,
    DMA1,
//    RCC,
    GPIOI,
    GPIOH,
    GPIOG,
    GPIOF,
    GPIOE,
    GPIOD,
    GPIOC,
    GPIOJ,
    GPIOK,
    GPIOB,
    GPIOA,
    SYSCFG,
    SPI1,
    SPI2,
    SPI3,
    I2S2EXT,
    I2S3EXT,
    SPI4,
    SPI5,
    SPI6,
    SDIO,
    ADC1,
    ADC2,
    ADC3,
    USART6,
    USART1,
    USART2,
    USART3,
    DAC,
    PWR,
    I2C3,
    I2C2,
    I2C1,
    IWDG,
    WWDG,
    RTC,
    UART4,
    UART5,
    UART7,
    UART8,
    ADC_COMMON,
    TIM1,
    TIM8,
    TIM2,
//    TIM3,
    TIM4,
    TIM5,
    TIM9,
    TIM12,
    TIM10,
    TIM13,
    TIM14,
    TIM11,
    TIM6,
    TIM7,
    ETHERNET_MAC,
    ETHERNET_MMC,
    ETHERNET_PTP,
    ETHERNET_DMA,
    CRC,
    OTG_FS_GLOBAL,
    OTG_FS_HOST,
    OTG_FS_DEVICE,
    OTG_FS_PWRCLK,
    CAN1,
    CAN2,
    FLASH,
    EXTI,
    OTG_HS_GLOBAL,
    OTG_HS_HOST,
    OTG_HS_DEVICE,
    OTG_HS_PWRCLK,
    SAI1,
    LTDC,
    HASH,
    CRYP,
    FPU,
    STK,
    NVIC_STIR,
    FPU_CPACR,
    SCB_ACTRL,
}
