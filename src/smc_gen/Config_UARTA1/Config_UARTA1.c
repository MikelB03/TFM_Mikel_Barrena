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
* File Name        : Config_UARTA1.c
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
volatile uint8_t * gp_uarta1_tx_address;
volatile uint16_t g_uarta1_tx_count;
=======
>>>>>>> Stashed changes
/* Start user code for global. Do not edit comment generated here */
/* End user code. Do not edit comment generated here */

/***********************************************************************************************************************
* Function Name: R_Config_UARTA1_Create
* Description  : This function initializes the UARTA1 module.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void R_Config_UARTA1_Create(void)
{
    R_UARTA->ASIMA10_b.EN = 0U;
<<<<<<< Updated upstream
    R_CLIC->clicintie47_b.IE = 0U;    /* disable UARTA_TX_ENDI1 interrupt */
    R_ICU->IELSR28 &= 0xFFFEFFFFU;    /* clear UARTA_TX_ENDI1 interrupt flag */
    R_CLIC->clicintctl47 = _0F_INT_PRIORITY_15;    /* set UARTA_TX_ENDI1 interrupt priority */
    R_CLIC->clicintattr47 = _C3_INT_VECTOR_MODE;    /* set UARTA_TX_ENDI1 interrupt to vector mode */
    R_ICU->IELSR28 &= 0xFFFFFFE0U;
    R_ICU->IELSR28 |= 0x1BU;    /* set UARTA_TX_ENDI1 interrupt event selection */
    R_UARTA->BRGCA1 = _D0_UARTA_OUTPUT_BAUDRATE;
    R_UARTA->ASIMA11 = _00_UARTA_PARITY_NONE | _18_UARTA_TRANSFER_LENGTH_8 | _00_UARTA_STOP_BIT_1 | 
                       _02_UARTA_DIRECTION_LSB | _00_UARTA_DATA_NORMAL;
    R_UARTA->ASIMA10 = _02_UARTA_BUFFER_EMPTY;
=======
    R_UARTA->BRGCA1 = _D0_UARTA_OUTPUT_BAUDRATE;
    R_UARTA->ASIMA11 = _00_UARTA_PARITY_NONE | _18_UARTA_TRANSFER_LENGTH_8 | _00_UARTA_STOP_BIT_1 | 
                       _02_UARTA_DIRECTION_LSB | _00_UARTA_DATA_NORMAL;
    R_UARTA->ASIMA10 = _00_UARTA_TRANSFER_END;
>>>>>>> Stashed changes
    R_UARTA->UTA0CK &= _CF_UARTA_FSEL_SOURCE_CLEAR;
    R_UARTA->UTA0CK |= _20_UARTA_FSEL_SELECT_UARTAHCLK;
    R_UARTA->UTA1CK = _00_UARTA1_SELECT_FSEL;
    /* Set TxDA1 pin */
    R_PFS->P403PFS_b.NCODR = 0U;
    R_PFS->P403PFS_b.PMR = 0U;
    R_PFS->P403PFS_b.PSEL = 0x06U;
    R_PFS->P403PFS_b.PMR = 1U;
    R_PFS->P403PFS_b.PDR = 1U;

    R_Config_UARTA1_Create_UserInit();
}

/***********************************************************************************************************************
* Function Name: R_Config_UARTA1_Start
* Description  : This function starts UARTA1 module operation.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void R_Config_UARTA1_Start(void)
{
<<<<<<< Updated upstream
    R_ICU->IELSR28 &= 0xFFFEFFFFU;    /* clear UARTA_TX_ENDI1 interrupt flag */
    R_CLIC->clicintie47_b.IE = 1U;    /* enable UARTA_TX_ENDI1 interrupt */
=======
>>>>>>> Stashed changes
    R_UARTA->ASIMA10_b.EN = 1U;
    R_UARTA->ASIMA10_b.TXEA = 1U;

    /* Wait for the period of at least one cycle of the UARTA operation clock */
    R_BSP_DelayCycle(UARTA1_WAIT_1_CLOCK_CYCLE);
}

/***********************************************************************************************************************
* Function Name: R_Config_UARTA1_Stop
* Description  : This function stops UARTA1 module operation.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void R_Config_UARTA1_Stop(void)
{
<<<<<<< Updated upstream
    R_CLIC->clicintie47_b.IE = 0U;    /* disable UARTA_TX_ENDI1 interrupt */
    R_ICU->IELSR28 &= 0xFFFEFFFFU;    /* clear UARTA_TX_ENDI1 interrupt flag */
=======
>>>>>>> Stashed changes
    R_UARTA->ASIMA10_b.TXEA = 0U;
    R_UARTA->ASIMA10_b.EN = 0U;
}

/***********************************************************************************************************************
* Function Name: R_Config_UARTA1_Send
* Description  : This function sends UARTA1 data.
* Arguments    : tx_buf -
*                    transfer buffer pointer
*                tx_num -
*                    buffer size
* Return Value : status -
*                    MD_OK or MD_ARGERROR
***********************************************************************************************************************/
MD_STATUS R_Config_UARTA1_Send(uint8_t * const tx_buf, uint16_t tx_num)
{
    MD_STATUS status = MD_OK;
<<<<<<< Updated upstream
=======
    uint8_t * tx_address;
    uint16_t tx_count;
>>>>>>> Stashed changes

    if (tx_num < 1U)
    {
        status = MD_ARGERROR;
    }
    else
    {
<<<<<<< Updated upstream
        gp_uarta1_tx_address = tx_buf;
        g_uarta1_tx_count = tx_num;
        /* Disable UARTA_TX_ENDI1 interrupt operation */
        R_CLIC->clicintie47_b.IE = 0U;
        R_UARTA->TXBA1 = *gp_uarta1_tx_address;
        gp_uarta1_tx_address++;
        g_uarta1_tx_count--;
        /* Enable UARTA_TX_ENDI1 interrupt operation */
        R_CLIC->clicintie47_b.IE = 1U;
=======
        tx_address = tx_buf;
        tx_count = tx_num;

        while (0U < tx_count)
        {
            while (0U != (R_UARTA->ASISA1 & _20_UARTA_DATA_EXIST_IN_TXBA))
            {
                ;
            }

            R_UARTA->TXBA1 = *tx_address;
            tx_count--;
            tx_address++;
        }

        while (0U != (R_UARTA->ASISA1 & _10_UARTA_HAVE_NEXT_TRANSFER))
        {
            ;
        }

        R_Config_UARTA1_PollingEnd_UserCode();
>>>>>>> Stashed changes
    }

    return (status);
}

/***********************************************************************************************************************
* Function Name: R_Config_UARTA1_Loopback_Enable
* Description  : This function enables the UARTA1 loopback function.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void R_Config_UARTA1_Loopback_Enable(void)
{
    R_PORGA->ULBS_b.ULBS5 = 1U;
}

/***********************************************************************************************************************
* Function Name: R_Config_UARTA1_Loopback_Disable
* Description  : This function disables the UARTA1 loopback function.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void R_Config_UARTA1_Loopback_Disable(void)
{
    R_PORGA->ULBS_b.ULBS5 = 0U;
}

/* Start user code for adding. Do not edit comment generated here */
/* End user code. Do not edit comment generated here */

