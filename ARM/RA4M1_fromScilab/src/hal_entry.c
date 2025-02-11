#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>


#include "hal_data.h"
#include "UART_config.h"
#include "ICU_config.h"
#include "_frenoEmergencia.h"

/*include necesarios funcionalidades*/
#include "r_sci_uart_cfg.h"
#include "r_ioport_cfg.h"
#include "r_icu_cfg.h"

/* Boolean flag to determine switch is pressed or not.*/
extern volatile bool g_sw_S1_press;
volatile double speed = 0;
double emergencyBreak;
volatile int blink = 0;

FSP_CPP_HEADER
void R_BSP_WarmStart(bsp_warm_start_event_t event);
FSP_CPP_FOOTER

/*******************************************************************************************************************//**
 * main() is generated by the RA Configuration editor and is used to generate threads if an RTOS is used.  This function
 * is called by main() when no RTOS is used.
 **********************************************************************************************************************/
void hal_entry(void)
{
    /* TODO: add your own code here */
    /*Declaracion variables*/
    fsp_err_t led, uart, icu;

    /*Inicializaciones*/
    led = R_IOPORT_Open(&g_ioport_ctrl, &g_bsp_pin_cfg);
    uart = init_uart();
    icu = icu_init();

    if (icu == FSP_SUCCESS) {
        icu_enable();  // Habilita la interrupción
    }

    while(1){

        if (true == g_sw_S1_press){
            speed = 15;  //dentro del rango mayor 10 y menor a 20 ENTONCES PARPADEO LED
            blink = 1;
        }
        else if (false == g_sw_S1_press){
            speed = 30; //fuera del rango mayor 10 y menor a 20 ENTOCES ENVIA MENSAJE FRENO OK
            blink = 0;
        }

        _frenoEmergencia(&emergencyBreak, speed);

        if(emergencyBreak != 0.0 && blink == 1){ //dentro del rango
            Parpadeo_LED(); //Parpadeo LED, Pin106
            g_sw_S1_press = false;  // Restablece la variable de la interrupción
        }
        else{
            led = R_IOPORT_PinWrite(&g_ioport_ctrl, BSP_IO_PORT_01_PIN_06, BSP_IO_LEVEL_LOW); //APAGAR EL LED
            R_BSP_SoftwareDelay(1, BSP_DELAY_UNITS_SECONDS); // DELAY
            uart = uart_write((uint8_t *)" FRENO OK");
        }
    }

#if BSP_TZ_SECURE_BUILD
    /* Enter non-secure code */
    R_BSP_NonSecureEnter();
#endif
}

/*******************************************************************************************************************//**
 * This function is called at various points during the startup process.  This implementation uses the event that is
 * called right before main() to set up the pins.
 *
 * @param[in]  event    Where at in the start up process the code is currently at
 **********************************************************************************************************************/
void R_BSP_WarmStart(bsp_warm_start_event_t event)
{
    if (BSP_WARM_START_RESET == event)
    {
#if BSP_FEATURE_FLASH_LP_VERSION != 0

        /* Enable reading from data flash. */
        R_FACI_LP->DFLCTL = 1U;

        /* Would normally have to wait tDSTOP(6us) for data flash recovery. Placing the enable here, before clock and
         * C runtime initialization, should negate the need for a delay since the initialization will typically take more than 6us. */
#endif
    }

    if (BSP_WARM_START_POST_C == event)
    {
        /* C runtime environment and system clocks are setup. */

        /* Configure pins. */
        R_IOPORT_Open (&IOPORT_CFG_CTRL, &IOPORT_CFG_NAME);

#if BSP_CFG_SDRAM_ENABLED

        /* Setup SDRAM and initialize it. Must configure pins first. */
        R_BSP_SdramInit(true);
#endif
    }
}

#if BSP_TZ_SECURE_BUILD

FSP_CPP_HEADER
BSP_CMSE_NONSECURE_ENTRY void template_nonsecure_callable ();

/* Trustzone Secure Projects require at least one nonsecure callable function in order to build (Remove this if it is not required to build). */
BSP_CMSE_NONSECURE_ENTRY void template_nonsecure_callable ()
{

}
FSP_CPP_FOOTER

#endif
