/*********************************************************************************************************
**--------------File Info---------------------------------------------------------------------------------
** File name:           IRQ_RIT.c
** Last modified Date:  2014-09-25
** Last Version:        V1.00
** Descriptions:        functions to manage T0 and T1 interrupts
** Correlated files:    RIT.h
**--------------------------------------------------------------------------------------------------------
*********************************************************************************************************/
#include "LPC17xx.h"
#include "RIT.h"
#include "../led/led.h"

/******************************************************************************
** Function name:		RIT_IRQHandler
**
** Descriptions:		REPETITIVE INTERRUPT TIMER handler
**
** parameters:			None
** Returned value:		None
**
******************************************************************************/

float res=0;

void RIT_IRQHandler (void)
{			
	static int down=0;	
	static int down0=0;
	static uint8_t position=0;
	
	if((LPC_PINCON->PINSEL4 & (1 << 22)) == 0){
		down++;
		if((LPC_GPIO2->FIOPIN & (1<<11)) == 0){   //key1
			reset_RIT(); // si può rimuovere
			switch(down){
				case 1:
					res = bubblesort(7000);
					clean_vector(7000);
					break;
				default:
					break;
			}
		}
		else {	/* button released */
			down=0;			
			disable_RIT();
			reset_RIT();
			NVIC_EnableIRQ(EINT1_IRQn);							 /* disable Button interrupts			*/
			LPC_PINCON->PINSEL4    |= (1 << 22);     /* External interrupt 0 pin selection */
		}
	}			
	
	
  LPC_RIT->RICTRL |= 0x1;	/* clear interrupt flag */
	
  return;
}

/******************************************************************************
**                            End Of File
******************************************************************************/
