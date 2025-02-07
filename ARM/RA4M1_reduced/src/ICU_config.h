/*
 * ICU_config.h
 *
 *  Created on: 14 ene 2025
 *      Author: mbarrena
 */

#include "hal_data.h"
#include "r_icu.h"
#include "r_external_irq_api.h"
#include "common_data.h"

#ifndef ICU_CONFIG_H_
#define ICU_CONFIG_H_

/* External IRQ channel for BOARD_RA4M1_EK*/
#define USER_SW_IRQ_NUMBER        (0x00)

extern volatile bool g_sw_S1_press;
//extern volatile int boton = 0;

//void irq_callback(external_irq_callback_args_t *p_args);


/* Function declaration */
fsp_err_t icu_init(void);
fsp_err_t icu_enable(void);
void icu_deinit(void);
void Parpadeo_LED (void);

#endif /* ICU_CONFIG_H_ */
