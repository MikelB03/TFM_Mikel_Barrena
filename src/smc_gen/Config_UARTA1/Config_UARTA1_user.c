/***********************************************************************************************************************
* DISCLAIMER
* This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products.
* No other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all
* applicable laws, including copyright laws. 
* THIS SOFTWARE IS PROVIDED "AS IS" AND RENESAS MAKES NO WARRANTIES REGARDING THIS SOFTWARE, WHETHER EXPRESS, IMPLIED
* OR STATUTORY, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
* NON-INFRINGEMENT.  ALL SUCH WARRANTIES ARE EXPRESSLY DISCLAIMED.TO THE MAXIMUM EXTENT PERMITTED NOT PROHIBITED BY
* LAW, NEITHER RENESAS ELECTRONICS CORPORATION NOR ANY OF ITS AFFILIATED COMPANIES SHALL BE LIABLE FOR ANY DIRECT,
* INDIRECT, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES FOR ANY REASON RELATED TO THIS SOFTWARE, EVEN IF RENESAS OR
* ITS AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
* Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability 
* of this software. By using this software, you agree to the additional terms and conditions found by accessing the 
* following link:
* http://www.renesas.com/disclaimer
*
* Copyright (C) 2021, 2023 Renesas Electronics Corporation. All rights reserved.
***********************************************************************************************************************/

/***********************************************************************************************************************
* File Name        : Config_UARTA1_user.c
* Component Version: 1.0.0
* Device(s)        : R9A02G0214CNE
* Description      : This file implements device driver for Config_UARTA1.
* Creation Date    : 
***********************************************************************************************************************/

/***********************************************************************************************************************
Pragma directive
***********************************************************************************************************************/
/* Start user code for pragma. Do not edit comment generated here */
/* End user code. Do not edit comment generated here */

/***********************************************************************************************************************
Includes
***********************************************************************************************************************/
#include "r_cg_macrodriver.h"
#include "Config_UARTA1.h"
/* Start user code for include. Do not edit comment generated here */
/* End user code. Do not edit comment generated here */
#include "r_cg_userdefine.h"

/***********************************************************************************************************************
Global variables and functions
***********************************************************************************************************************/
<<<<<<< Updated upstream
extern volatile uint8_t * gp_uarta1_tx_address;
extern volatile uint16_t g_uarta1_tx_count;
=======
>>>>>>> Stashed changes
/* Start user code for global. Do not edit comment generated here */
/* End user code. Do not edit comment generated here */

/***********************************************************************************************************************
* Function Name: R_Config_UARTA1_Create_UserInit
* Description  : This function adds user code after initializing UARTA1.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void R_Config_UARTA1_Create_UserInit(void)
{
    /* Start user code for user init. Do not edit comment generated here */
    /* End user code. Do not edit comment generated here */
}

/***********************************************************************************************************************
<<<<<<< Updated upstream
* Function Name: r_Config_UARTA1_callback_sendend
* Description  : This function is a callback function when UARTA1 finishes transmission.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
static void r_Config_UARTA1_callback_sendend(void)
{
    /* Start user code for r_Config_UARTA1_callback_sendend. Do not edit comment generated here */
    /* End user code. Do not edit comment generated here */
}

/***********************************************************************************************************************
* Function Name: r_Config_UARTA1_interrupt_send
* Description  : This function is UARTA1 send interrupt service routine.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void r_Config_UARTA1_interrupt_send(void)
{
    if (g_uarta1_tx_count > 0U)
    {
        R_UARTA->TXBA1 = *gp_uarta1_tx_address;
        gp_uarta1_tx_address++;
        g_uarta1_tx_count--;
    }
    else if (0U != (R_UARTA->ASIMA10 & _02_UARTA_BUFFER_EMPTY))
    {
        R_UARTA->ASIMA10_b.ISSMA = 0U;
    }
    else
    {
        r_Config_UARTA1_callback_sendend();
    }
    R_ICU->IELSR28 &= 0xFFFEFFFFU;    /* clear UARTA_TX_ENDI1 interrupt flag */
}

=======
* Function Name: R_Config_UARTA1_PollingEnd_UserCode
* Description  : This function is a callback function when UARTA1 finishes polling transmission.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void R_Config_UARTA1_PollingEnd_UserCode(void)
{
    /* Start user code for R_Config_UARTA1_PollingEnd_UserCode. Do not edit comment generated here */
    /* End user code. Do not edit comment generated here */
}

>>>>>>> Stashed changes
/* Start user code for adding. Do not edit comment generated here */
/* End user code. Do not edit comment generated here */

