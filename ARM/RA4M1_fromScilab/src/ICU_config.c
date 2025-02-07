/*
 * ICU_config.c
 *
 *  Created on: 15 ene 2025
 *      Author: mbarrena
 */


#include "ICU_config.h"


/* Boolean flag to determine switch is pressed or not.*/
volatile bool g_sw_S1_press = false;

void external_irq_callback(external_irq_callback_args_t *p_args);




 fsp_err_t icu_init(void)
{
    fsp_err_t err = FSP_SUCCESS;

    external_irq_cfg_t icu_cfg =
    {
        .channel          = 0,
        .trigger          = EXTERNAL_IRQ_TRIG_RISING,
        .filter_enable    = false,
        .clock_source_div = EXTERNAL_IRQ_CLOCK_SOURCE_DIV_64,
        .p_callback       = external_irq_callback,
        .p_context        = 0,
        .ipl              = 0,
        .irq              = VECTOR_NUMBER_ICU_IRQ0,
    };


    /* Open ICU module */
    err = R_ICU_ExternalIrqOpen(&g_external_irq0_ctrl, &icu_cfg);

    return err;
}


fsp_err_t icu_enable(void)
{
    fsp_err_t err = FSP_SUCCESS;

    /* Enable ICU module */
    err = R_ICU_ExternalIrqEnable(&g_external_irq0_ctrl);
    return err;
}


void icu_deinit(void)
{
    fsp_err_t err = FSP_SUCCESS;

    /* Close ICU module */
    err = R_ICU_ExternalIrqClose(&g_external_irq0_ctrl);

}


void external_irq_callback(external_irq_callback_args_t *p_args)
{
    if(USER_SW_IRQ_NUMBER == p_args->channel)
    {
        g_sw_S1_press = true;
    }
}

void Parpadeo_LED (void)
{
    fsp_err_t led;

    for(int i=0; i<= 20; i++) {
        led = R_IOPORT_PinWrite(&g_ioport_ctrl, BSP_IO_PORT_01_PIN_06, BSP_IO_LEVEL_HIGH);
        R_BSP_SoftwareDelay(500, BSP_DELAY_UNITS_MILLISECONDS); // DELAY
        led = R_IOPORT_PinWrite(&g_ioport_ctrl, BSP_IO_PORT_01_PIN_06, BSP_IO_LEVEL_LOW);
        R_BSP_SoftwareDelay(500, BSP_DELAY_UNITS_MILLISECONDS); // DELAY
    }
}

