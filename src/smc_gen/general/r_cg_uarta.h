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
* File Name        : r_cg_uarta.h
* Version          : 1.0.30
* Device(s)        : R9A02G0214CNE
* Description      : General header file for UARTA peripheral.
* Creation Date    : 
***********************************************************************************************************************/


#ifndef UARTA_H
#define UARTA_H

/***********************************************************************************************************************
Macro definitions (Register bit)
***********************************************************************************************************************/
/*
    Operation Mode Setting Register n0 (ASIMAn0)
*/
/* UART operation enable (EN) */
#define _00_UARTA_OPERATION_DISABLE           (0x00U)    /* disables the UART operation clock */
#define _80_UARTA_OPERATION_ENABLE            (0x80U)    /* enables the UART operation clock */
/* Transmission enable (TXEA) */
#define _00_UARTA_TRANSMIT_DISABLE            (0x00U)    /* disables transmission */
#define _40_UARTA_TRANSMIT_ENABLE             (0x40U)    /* enables transmission */
/* Reception enable (RXEA) */
#define _00_UARTA_RECEIVE_DISABLE             (0x00U)    /* disables reception */
#define _20_UARTA_RECEIVE_ENABLE              (0x20U)    /* enables reception */
/* Transmit interrupt mode select (ISSMA) */
#define _00_UARTA_TRANSFER_END                (0x00U)    /* the UARTA_TX_ENDIn interrupt is generated
                                                            on completion of transmission */
#define _02_UARTA_BUFFER_EMPTY                (0x02U)    /* the UARTA_TX_ENDIn interrupt is generated when the transmit
                                                            buffer becomes empty (for continuous transmission) */
/* Receive interrupt mode select (ISRMA) */
#define _00_UARTA_INTURE_OCCUR                (0x00U)    /* the UARTA_RX_ERIn interrupt is generated when
                                                            a reception error occurs (UARTA_RX_ENDIn is not generated) */
#define _01_UARTA_INTUR_OCCUR                 (0x01U)    /* the UARTA_RX_ENDIn interrupt is generated when
                                                            a reception error occurs (UARTA_RX_ERIn is not generated) */

/*
    Operation Mode Setting Register n1 (ASIMAn1)
*/
/* Transmission and reception parity bit setting (PS[1:0]) */
#define _00_UARTA_PARITY_NONE                 (0x00U)    /* no parity is expected */
#define _20_UARTA_PARITY_ZERO                 (0x20U)    /* set parity is zero */
#define _40_UARTA_PARITY_ODD                  (0x40U)    /* set parity is odd */
#define _60_UARTA_PARITY_EVEN                 (0x60U)    /* set parity is even */
/* Transmission and reception character length setting (CL[1:0]) */
#define _00_UARTA_TRANSFER_LENGTH_5           (0x00U)    /* character length of data = 5 bits */
#define _10_UARTA_TRANSFER_LENGTH_7           (0x10U)    /* character length of data = 7 bits */
#define _18_UARTA_TRANSFER_LENGTH_8           (0x18U)    /* character length of data = 8 bits */
/* Transmission stop bit length setting (SL) */
#define _00_UARTA_STOP_BIT_1                  (0x00U)    /* stop bit length = 2 bits */
#define _04_UARTA_STOP_BIT_2                  (0x04U)    /* stop bit length = 1 bit */
/* Transmission and reception order setting (DIR) */
#define _00_UARTA_DIRECTION_MSB               (0x00U)    /* MSB first */
#define _02_UARTA_DIRECTION_LSB               (0x02U)    /* LSB first */
/* Transmission and reception level setting (ALV) */
#define _00_UARTA_DATA_NORMAL                 (0x00U)    /* positive logic */
#define _01_UARTA_DATA_INVERTED               (0x01U)    /* negative logic */

/*
    Status Register n (ASISAn)
*/
/* Transmit buffer data flag (TXBFA) */
#define _00_UARTA_DATA_TRANSFER_END           (0x00U)    /* no valid data exists in the TXBAn register */
#define _20_UARTA_DATA_EXIST_IN_TXBA          (0x20U)    /* valid data exists in the TXBAn register */
/* Transmit shift register data flag (TXSFA) */
#define _00_UARTA_NO_NEXT_DATA                (0x00U)    /* EN is 0 or TXEA when set to 0 */
#define _10_UARTA_HAVE_NEXT_TRANSFER          (0x10U)    /* data is transferred from TXBAn */
/* Parity error flag (PEA) */
#define _00_UARTA_PARITY_ERROR_UNDETECTED     (0x00U)    /* parity error is not detected */
#define _04_UARTA_PARITY_ERROR_DETECTED       (0x04U)    /* parity error is detected */
/* Framing error flag (FEA) */
#define _00_UARTA_FRAME_ERROR_UNDETECTED      (0x00U)    /* frame error is not detected */
#define _02_UARTA_FRAME_ERROR_DETECTED        (0x02U)    /* frame error is detected */
/* Overrun error flag (OVEA) */
#define _00_UARTA_OVERRUN_ERROR_UNDETECTED    (0x00U)    /* overrun error is not detected */
#define _01_UARTA_OVERRUN_ERROR_DETECTED      (0x01U)    /* overrun error is detected */

/*
    Status Clear Trigger Register n (ASCTAn)
*/
/* Parity error flag clear trigger (PECTA) */
#define _04_UARTA_PARITY_FLAG_CLEAR           (0x04U)    /* clear parity error flag */
/* Framing error flag clear trigger (FECTA) */
#define _02_UARTA_FRAME_FLAG_CLEAR            (0x02U)    /* clear frame error flag */
/* Overrun error flag clear trigger (OVECTA) */
#define _01_UARTA_OVERRUN_FLAG_CLEAR          (0x01U)    /* clear overrun error flag */

/*
    UARTA Clock Select Register 0 (UTA0CK)
*/
/* UARTA0 clock output function enable (EN) */
#define _00_UARTA0_CLOCK_OUTPUT_DISABLE       (0x00U)    /* disables CLKA0 output */
#define _80_UARTA0_CLOCK_OUTPUT_ENABLE        (0x80U)    /* enables CLKA0 output */
/* fSEL clock select (SEL[1:0]) */
#define _00_UARTA_FSEL_DISABLE                (0x00U)    /* stop */
#define _10_UARTA_FSEL_SELECT_UARTAMCLK       (0x10U)    /* UARTAMCLK */
#define _20_UARTA_FSEL_SELECT_UARTAHCLK       (0x20U)    /* UARTAHCLK */
#define _30_UARTA_FSEL_SELECT_UARTAMOCLK      (0x30U)    /* UARTAMOCLK */
#define _CF_UARTA_FSEL_SOURCE_CLEAR           (0xCFU)
/* UARTA0 operation clock select (fUTA0) (CK[3:0]) */
#define _00_UARTA0_SELECT_FSEL                (0x00U)    /* fSEL */
#define _01_UARTA0_SELECT_FSEL2               (0x01U)    /* fSEL/2 */
#define _02_UARTA0_SELECT_FSEL4               (0x02U)    /* fSEL/4 */
#define _03_UARTA0_SELECT_FSEL8               (0x03U)    /* fSEL/8 */
#define _04_UARTA0_SELECT_FSEL16              (0x04U)    /* fSEL/16 */
#define _05_UARTA0_SELECT_FSEL32              (0x05U)    /* fSEL/32 */
#define _06_UARTA0_SELECT_FSEL64              (0x06U)    /* fSEL/64 */
#define _08_UARTA0_SELECT_UARTALCLK_SCLK      (0x08U)    /* UARTALCLK/UARTASCLK */
#define _F0_UARTA0_CLOCK_SELECT_CLEAR         (0xF0U)

/*
    UARTA Clock Select Register 1 (UTA1CK)
*/
/* UARTA1 clock output function enable (EN) */
#define _00_UARTA1_CLOCK_OUTPUT_DISABLE       (0x00U)    /* disables CLKA1 output */
#define _80_UARTA1_CLOCK_OUTPUT_ENABLE        (0x80U)    /* enables CLKA1 output */
/* UARTA1 operation clock select (fUTA1) (CK[3:0]) */
#define _00_UARTA1_SELECT_FSEL                (0x00U)    /* fSEL */
#define _01_UARTA1_SELECT_FSEL2               (0x01U)    /* fSEL/2 */
#define _02_UARTA1_SELECT_FSEL4               (0x02U)    /* fSEL/4 */
#define _03_UARTA1_SELECT_FSEL8               (0x03U)    /* fSEL/8 */
#define _04_UARTA1_SELECT_FSEL16              (0x04U)    /* fSEL/16 */
#define _05_UARTA1_SELECT_FSEL32              (0x05U)    /* fSEL/32 */
#define _06_UARTA1_SELECT_FSEL64              (0x06U)    /* fSEL/64 */
#define _08_UARTA1_SELECT_UARTALCLK_SCLK      (0x08U)    /* UARTALCLK/UARTASCLK */
#define _F0_UARTA1_CLOCK_SELECT_CLEAR         (0xF0U)

/*
    UART Loopback Select Register (ULBS)
*/
/* Selection of the UART loopback function (ULBS5) */
#define _00_UARTA1_LOOPBACK_DISABLE           (0x00U)    /* the state of the RxDA1 pin of the serial interface UARTA1
                                                            is input to the receive shift register */
#define _20_UARTA1_LOOPBACK_ENABLE            (0x20U)    /* the output from the transmit shift register
                                                            is looped back to the receive shift register */
/* Selection of the UART loopback function (ULBS4) */
#define _00_UARTA0_LOOPBACK_DISABLE           (0x00U)    /* the state of the RxDA0 pin of the serial interface UARTA0
                                                            is input to the receive shift register */
#define _10_UARTA0_LOOPBACK_ENABLE            (0x10U)    /* the output from the transmit shift register
                                                            is looped back to the receive shift register */

/***********************************************************************************************************************
Macro definitions
***********************************************************************************************************************/

/***********************************************************************************************************************
Typedef definitions
***********************************************************************************************************************/

/***********************************************************************************************************************
Global functions
***********************************************************************************************************************/
/* Start user code for function. Do not edit comment generated here */
/* End user code. Do not edit comment generated here */
#endif

