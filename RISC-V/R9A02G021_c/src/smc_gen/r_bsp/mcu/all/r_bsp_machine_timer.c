/***********************************************************************************************************************
* DISCLAIMER
* This software is supplied by Renesas Electronics Corporation and is only intended for use with Renesas products. No 
* other uses are authorized. This software is owned by Renesas Electronics Corporation and is protected under all 
* applicable laws, including copyright laws. 
* THIS SOFTWARE IS PROVIDED "AS IS" AND RENESAS MAKES NO WARRANTIES REGARDING
* THIS SOFTWARE, WHETHER EXPRESS, IMPLIED OR STATUTORY, INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY, 
* FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. ALL SUCH WARRANTIES ARE EXPRESSLY DISCLAIMED. TO THE MAXIMUM 
* EXTENT PERMITTED NOT PROHIBITED BY LAW, NEITHER RENESAS ELECTRONICS CORPORATION NOR ANY OF ITS AFFILIATED COMPANIES 
* SHALL BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL, INCIDENTAL OR CONSEQUENTIAL DAMAGES FOR ANY REASON RELATED TO THIS 
* SOFTWARE, EVEN IF RENESAS OR ITS AFFILIATES HAVE BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
* Renesas reserves the right, without notice, to make changes to this software and to discontinue the availability of 
* this software. By using this software, you agree to the additional terms and conditions found by accessing the 
* following link:
* http://www.renesas.com/disclaimer
*
* Copyright (C) 2024 Renesas Electronics Corporation. All rights reserved.
***********************************************************************************************************************/
/***********************************************************************************************************************
* File Name    : r_bsp_machine_timer.c
* Description  : Implements functions that generate interrupt support.
***********************************************************************************************************************/
/***********************************************************************************************************************
* History : DD.MM.YYYY Version  Description
*         : 10.06.2024 1.20     Support machine timer setting"
***********************************************************************************************************************/
#include "platform.h"
#include "r_cg_macrodriver.h"
#include "r_bsp_machine_timer.h"

/***********************************************************************************************************************
* Function Name: machine_timer_create
* Description  : This function initializes machine timer.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void machine_timer_create(void)
{
#if BSP_CFG_MACHINE_TIMER == 1
    /* Select clock source */
    R_CPU_AUX->MACTCR_b.CLOCKSOURCE = BSP_CFG_MTIME_CLOCK_SOURCE;
    /* Set counter value */
    R_MTIME->mtime_hi = (uint32_t)BSP_CFG_MTIME_COUNTER_HIGHERBITS;
    R_MTIME->mtime_lo = (uint32_t)BSP_CFG_MTIME_COUNTER_LOWERBITS;
    /* Set compare value */
    R_MTIME->mtimecmp_hi = (uint32_t)BSP_CFG_MTIME_COMPARE_HIGHERBITS;
    R_MTIME->mtimecmp_lo = (uint32_t)BSP_CFG_MTIME_COMPARE_LOWERBITS;
    /* Set mtip priority */
    R_CLIC->clicintie7_b.IE = 0U;    /* disable mtip interrupt */
    R_CLIC->clicintctl7 = BSP_CFG_MTIP_PRIORITY;
    R_CLIC->clicintattr7 = _C3_INT_VECTOR_MODE;
#endif
#if BSP_CFG_SOFTWARE_INTERRUPT == 1
    /* Set msip priority */
    R_CLIC->clicintie3_b.IE = 0U;    /* disable msip interrupt */
    R_CLIC->clicintctl3 = (uint8_t)BSP_CFG_MSIP_PRIORITY;
    R_CLIC->clicintattr3 = _C3_INT_VECTOR_MODE;
#endif
}

#if BSP_CFG_MACHINE_TIMER == 1
/***********************************************************************************************************************
* Function Name: machine_timer_start
* Description  : This function starts machine timer.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void machine_timer_start(void)
{
    R_CPU_AUX->MACTCR_b.ENABLE = 1U;
    R_CLIC->clicintie7_b.IE = 1U;    /* enable mtip interrupt */
}
/***********************************************************************************************************************
* Function Name: machine_timer_start
* Description  : This function stops machine timer.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void machine_timer_stop(void)
{
    R_CLIC->clicintie7_b.IE = 0U;    /* disable mtip interrupt */
    R_CPU_AUX->MACTCR_b.ENABLE = 0U;
}
#endif
#if BSP_CFG_SOFTWARE_INTERRUPT == 1
/***********************************************************************************************************************
* Function Name: trigger_software_interrupt
* Description  : This function trigger software interrupt.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void trigger_software_interrupt(void)
{
    R_MTIME->msip_b.MSIP = 1U;
    R_CLIC->clicintie3_b.IE = 1U;    /* enable msip interrupt */
    R_MTIME->msip_b.MSIP = 0U;
}
/***********************************************************************************************************************
* Function Name: clear_software_interrupt
* Description  : This function clear software interrupt.
* Arguments    : None
* Return Value : None
***********************************************************************************************************************/
void clear_software_interrupt(void)
{
    R_CLIC->clicintie3_b.IE = 0U;    /* disable msip interrupt */
    R_MTIME->msip_b.MSIP = 1U;
}
#endif
