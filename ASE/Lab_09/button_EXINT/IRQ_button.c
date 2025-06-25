#include "button.h"
#include "lpc17xx.h"

#include "../led/led.h"

void EINT0_IRQHandler (void)	  
{
	//LED_On(0);
  LPC_SC->EXTINT &= (1 << 0);     /* clear pending interrupt         */	
}

extern unsigned char seed;

const unsigned char taps = 0x1D;  // posizione degli xor nei 8 bit
unsigned char current_state = 0;
int output_bit = 0;

void show_number(unsigned char number) {
	int i, x, potenza_di_due = 0;
	for(i=0;i<8;i++){
		potenza_di_due = 1 << i;  // shift potenza di 2 a sx
		x = number & potenza_di_due;  // and tra due valori, 0 se tutti 0 altrimenti valore != 0
		if(x){
			LED_On(i);
		} else {
			LED_Off(i);
		} 
	} 
}

void start_seed() {
	current_state = seed;
	show_number(seed);
}

unsigned char next_state_1(unsigned char current_state, unsigned char taps, int *output_bit) {
	
	*output_bit = current_state & 1;
	//																7								 5											4										 	 3
	unsigned char input_bit = (current_state ^ (current_state >> 2) ^ (current_state >> 3) ^ (current_state >> 4)) & 1;
	
	return (current_state >> 1) | (input_bit << 7);
}

unsigned char next_state_2(unsigned char current_state, unsigned char taps, int *output_bit) {
	
	*output_bit = current_state & 1;
		//															7								 6											2										 	 1
	unsigned char input_bit = (current_state ^ (current_state >> 1) ^ (current_state >> 5) ^ (current_state >> 6)) & 1;
	
	return (current_state >> 1) | (input_bit << 7);
}

unsigned char next_state_3(unsigned char current_state, unsigned char taps, int *output_bit) {
	
	*output_bit = current_state & 1;
		//															7								 4											3											 2										 	1
	unsigned char input_bit = (current_state ^ (current_state >> 3) ^ (current_state >> 4) ^ (current_state >> 5) ^ (current_state >> 6)) & 1;
	
	return (current_state >> 1) | (input_bit << 7);
}

unsigned char next_state_4(unsigned char current_state, unsigned char taps, int *output_bit) {
	
	*output_bit = current_state & 1;
		//															7								6											 5												4											3											 2										 	1
	unsigned char input_bit = (current_state ^ (current_state >> 1) ^ (current_state >> 2) ^ (current_state >> 3) ^ (current_state >> 4) ^ (current_state >> 5) ^ (current_state >> 6)) & 1;
	
	return (current_state >> 1) | (input_bit << 7);
}

void EINT1_IRQHandler (void)	  
{
  //LED_On(1);
	
	//lunch function
	current_state = next_state_1(current_state, taps, &output_bit);
	show_number(current_state);
	
	
	LPC_SC->EXTINT &= (1 << 1);     /* clear pending interrupt         */
}

void EINT2_IRQHandler (void)	  
{
	//LED_Off(0);
	//LED_Off(1);

	//lunch function
	int count = 0;
	do {
			current_state = next_state_4(current_state, taps, &output_bit);
			//show_number(current_state);
			count++;
	} while(current_state != seed);
	
	// output the max lenght
	show_number((unsigned char) count);

	
	LPC_SC->EXTINT &= (1 << 2);     /* clear pending interrupt         */  
}


