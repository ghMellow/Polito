/*********************************************************************************************************
**--------------File Info---------------------------------------------------------------------------------
** File name:           lib_timer.h
** Last modified Date:  2014-09-25
** Last Version:        V1.00
** Descriptions:        atomic functions to be used by higher sw levels
** Correlated files:    lib_timer.c, funct_timer.c, IRQ_timer.c
**--------------------------------------------------------------------------------------------------------
*********************************************************************************************************/
#include "LPC17xx.h"
#include "timer.h"

/******************************************************************************
** Function name:		enable_timer
**
** Descriptions:		Enable timer
**
** parameters:			timer number: 0 or 1
** Returned value:		None
**
******************************************************************************/
// timer 0 e 1 sono gi� accesi, 2 e 3 no! | 
void enable_timer( uint8_t timer_num )
{
  if ( timer_num == 0 )
  {
	LPC_TIM0->TCR = 1;
  }
  else if (timer_num==1)
  {
	LPC_TIM1->TCR = 1;
  }
	else if (timer_num==2)
  {
	LPC_TIM2->TCR = 1;
  }
	else
  {
	LPC_TIM3->TCR = 1;
  }
  return;
}

/******************************************************************************
** Function name:		disable_timer
**
** Descriptions:		Disable timer
**
** parameters:			timer number: 0 or 1
** Returned value:		None
**
******************************************************************************/
void disable_timer( uint8_t timer_num )
{
  if ( timer_num == 0 )
  {
	LPC_TIM0->TCR = 0;
  }
  else if (timer_num==1)
  {
	LPC_TIM1->TCR = 0;
  }
	else if (timer_num==2)
  {
	LPC_TIM2->TCR = 0;
  }
	else
  {
	LPC_TIM3->TCR = 0;
  }
  return;
}

/******************************************************************************
** Function name:		reset_timer
**
** Descriptions:		Reset timer
**
** parameters:			timer number: 0 or 1
** Returned value:		None
**
******************************************************************************/
// farlo ripartire da zero senza per� pulire il primo bit
void reset_timer( uint8_t timer_num )
{
  uint32_t regVal;

  if ( timer_num == 0 )
  {
	regVal = LPC_TIM0->TCR; // prende val del timer 0 e metto in registro
	regVal |= 0x02;  				// bit wise con 2 (binario: 10) e cos� mette a 1 il 2 bit di reset e 0 che indica che non devono essere toccati
	LPC_TIM0->TCR = regVal; // sovrascire valore registro
  }
  else if (timer_num==1)
  {
	regVal = LPC_TIM1->TCR;
	regVal |= 0x02;
	LPC_TIM1->TCR = regVal;
  }
	else if (timer_num==2)
  {
	regVal = LPC_TIM2->TCR;
	regVal |= 0x02;
	LPC_TIM2->TCR = regVal;
  }
	else
  {
	regVal = LPC_TIM3->TCR;
	regVal |= 0x02;
	LPC_TIM3->TCR = regVal;
  }
  return;
}

// vogliamo inizializzare dal main diverse configurazioni
uint32_t init_timer ( uint8_t timer_num, uint32_t Prescaler, uint8_t MatchReg, uint8_t SRImatchReg, uint32_t TimerInterval )
{
  if ( timer_num == 0 )
  {
		LPC_TIM0 -> PR = Prescaler;
		if(MatchReg == 0) {
			LPC_TIM0->MR0 = TimerInterval;
			LPC_TIM0->MCR |= SRImatchReg << (3*MatchReg); //bit wise OR cos� da usare in contemporanea tutti i match register senza pulire il registro
		} else if (MatchReg == 1) {
			LPC_TIM0->MR1 = TimerInterval;
			LPC_TIM0->MCR |= SRImatchReg << (3*MatchReg);
		} else if (MatchReg == 2) {
			LPC_TIM0->MR2 = TimerInterval;
			LPC_TIM0->MCR |= SRImatchReg << (3*MatchReg);
		} else if (MatchReg == 3) {
			LPC_TIM0->MR3 = TimerInterval;
			LPC_TIM0->MCR |= SRImatchReg << (3*MatchReg);
		}
		
		NVIC_EnableIRQ(TIMER0_IRQn);
		/*NVIC_SetPriority(TIMER0_IRQn, 4);*/		/* less priority than buttons */
		NVIC_SetPriority(TIMER0_IRQn, 0);		/* more priority than buttons */
		return (1);
  }
	// timer 1
  else if ( timer_num == 1 )
  {	
		LPC_TIM1 -> PR = Prescaler;
		if(MatchReg == 0) {
			LPC_TIM1->MR0 = TimerInterval;
			LPC_TIM1->MCR |= SRImatchReg << (3*MatchReg); //bit wise OR cos� da usare in contemporanea tutti i match register senza pulire il registro
		} else if (MatchReg == 1) {
			LPC_TIM1->MR1 = TimerInterval;
			LPC_TIM1->MCR |= SRImatchReg << (3*MatchReg);
		} else if (MatchReg == 2) {
			LPC_TIM1->MR2 = TimerInterval;
			LPC_TIM1->MCR |= SRImatchReg << (3*MatchReg);
		} else if (MatchReg == 3) {
			LPC_TIM1->MR3 = TimerInterval;
			LPC_TIM1->MCR |= SRImatchReg << (3*MatchReg);
		}		

		NVIC_EnableIRQ(TIMER1_IRQn);
		NVIC_SetPriority(TIMER1_IRQn, 5);	/* less priority than buttons and timer0*/
		return (1);
  }
	// timer 2
	else if ( timer_num == 2 )
  {	
		LPC_TIM2 -> PR = Prescaler;
		if(MatchReg == 0) {
			LPC_TIM2->MR0 = TimerInterval;
			LPC_TIM2->MCR |= SRImatchReg << (3*MatchReg); //bit wise OR cos� da usare in contemporanea tutti i match register senza pulire il registro
		} else if (MatchReg == 1) {
			LPC_TIM2->MR1 = TimerInterval;
			LPC_TIM2->MCR |= SRImatchReg << (3*MatchReg);
		} else if (MatchReg == 2) {
			LPC_TIM2->MR2 = TimerInterval;
			LPC_TIM2->MCR |= SRImatchReg << (3*MatchReg);
		} else if (MatchReg == 3) {
			LPC_TIM2->MR3 = TimerInterval;
			LPC_TIM2->MCR |= SRImatchReg << (3*MatchReg);
		}		

		NVIC_EnableIRQ(TIMER2_IRQn);
		NVIC_SetPriority(TIMER2_IRQn, 5);	/* less priority than buttons and timer0*/
		return (1);
  }
	// timer 3
	else if ( timer_num == 3 )
  {	
		LPC_TIM3 -> PR = Prescaler;
		if(MatchReg == 0) {
			LPC_TIM3->MR0 = TimerInterval;
			LPC_TIM3->MCR |= SRImatchReg << (3*MatchReg); //bit wise OR cos� da usare in contemporanea tutti i match register senza pulire il registro
		} else if (MatchReg == 1) {
			LPC_TIM3->MR1 = TimerInterval;
			LPC_TIM3->MCR |= SRImatchReg << (3*MatchReg);
		} else if (MatchReg == 2) {
			LPC_TIM3->MR2 = TimerInterval;
			LPC_TIM3->MCR |= SRImatchReg << (3*MatchReg);
		} else if (MatchReg == 3) {
			LPC_TIM3->MR3 = TimerInterval;
			LPC_TIM3->MCR |= SRImatchReg << (3*MatchReg);
		}		

		NVIC_EnableIRQ(TIMER3_IRQn);
		NVIC_SetPriority(TIMER3_IRQn, 5);	/* less priority than buttons and timer0*/
		return (1);
  }
	
	
  return (0); // i return la prof li ha al contrario tutti 0 e qui 1
}

/******************************************************************************
**                            End Of File
******************************************************************************/
