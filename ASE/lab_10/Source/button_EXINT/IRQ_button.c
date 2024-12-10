#include "button.h"
#include "LPC17xx.h"


#include "../led/led.h"
#include "../timer/timer.h"
#include "../RIT/RIT.h"


float bubblesort(int n) {
	float sum;
	int i,k;
	unsigned int temp;
	for(i = 0; i<n-1; i++) {
		sum+=camp[i];
	 for(k = 0; k<n-1-i; k++) {
					 if(camp[k] > camp[k+1]) {
						temp = camp[k];
						camp[k] = camp[k+1];
						camp[k+1] = temp;
					 }
	 }
	}
	sum+=camp[n-1];
	sum=sum/n;
	
	return sum;
}

void clean_vector(int n){
	int i =0;
	for(i=0; i<n; i++){
		camp[i]=(unsigned int)0xCAFECAFE;
	}
}

void EINT0_IRQHandler (void)	  	/* INT0														 */
{
	
	LPC_SC->EXTINT &= (1 << 0);     /* clear pending interrupt         */
}


//void EINT1_IRQHandler (void)	  	/* KEY1														 */
//{
//	
//	bubblesort(7000);
//	clean_vector(7000);
//	LPC_SC->EXTINT &= (1 << 1);     /* clear pending interrupt         */
//}
void EINT1_IRQHandler (void)	  	/* KEY1														 */
{
	enable_RIT();										/* enable RIT to count 50ms				 */
	NVIC_DisableIRQ(EINT1_IRQn);		/* disable Button interrupts			 */
	LPC_PINCON->PINSEL4    &= ~(1 << 22);     /* GPIO pin selection */
	
	LPC_SC->EXTINT &= (1 << 1);     /* clear pending interrupt         */
}


void EINT2_IRQHandler (void)	  	/* KEY2														 */
{
	LPC_SC->EXTINT &= (1 << 2);     /* clear pending interrupt         */  
	enable_timer(0);  
}


