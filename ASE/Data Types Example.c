#include <stdio.h>
#include <stdint.h> // Per tipi standard come uint8_t, int16_t

int main() {
    // 1. Basic Data Types

    // char: 8-bit signed integer (-128 to 127)
    char a = 'A'; // Può rappresentare caratteri o piccoli numeri interi

    // unsigned char: 8-bit unsigned integer (0 to 255)
    unsigned char b = 200; // Solo numeri positivi

    // int: Generalmente 32-bit signed integer (-2^31 to 2^31-1, ma dipende dall'architettura)
    int c = -1000; // Valori negativi e positivi

    // unsigned int: Generalmente 32-bit unsigned integer (0 to 2^32-1)
    unsigned int d = 3000; // Solo numeri positivi

    // short: Generalmente 16-bit signed integer (-2^15 to 2^15-1)
    short e = -32768; // Usato per risparmiare memoria

    // unsigned short: 16-bit unsigned integer (0 to 65535)
    unsigned short f = 65535; // Solo numeri positivi

    // long: Generalmente 64-bit signed integer (-2^63 to 2^63-1)
    long g = 1000000000L; // Valori molto grandi

    // unsigned long: 64-bit unsigned integer (0 to 2^64-1)
    unsigned long h = 4000000000UL; // Solo numeri positivi molto grandi

    // float: Single precision floating-point (generalmente 32-bit)
    float i = 3.14f; // Valori con decimali, precisione di circa 6-7 cifre significative

    // double: Double precision floating-point (generalmente 64-bit)
    double j = 2.718281828459045; // Maggiore precisione rispetto a float (circa 15 cifre significative)

    // 2. Fixed-Width Integer Types (stdint.h)

    // int8_t: 8-bit signed integer (-128 to 127)
    int8_t k = -128;

    // uint8_t: 8-bit unsigned integer (0 to 255)
    uint8_t l = 255;

    // int16_t: 16-bit signed integer (-32768 to 32767)
    int16_t m = -32768;

    // uint16_t: 16-bit unsigned integer (0 to 65535)
    uint16_t n = 65535;

    // int32_t: 32-bit signed integer (-2^31 to 2^31-1)
    int32_t o = -2147483648;

    // uint32_t: 32-bit unsigned integer (0 to 2^32-1)
    uint32_t p = 4294967295U;

    // int64_t: 64-bit signed integer (-2^63 to 2^63-1)
    int64_t q = -9223372036854775807LL;

    // uint64_t: 64-bit unsigned integer (0 to 2^64-1)
    uint64_t r = 18446744073709551615ULL;

    // 3. Arrays (Vettori)

    // Array di char (stringa): 8-bit per carattere
    char str[6] = "Hello"; // Termina con carattere nullo '\0'

    // Array di int: Generalmente 32-bit per elemento
    int intArray[5] = {1, 2, 3, 4, 5}; // Sequenza di numeri interi

    // Array di float: Generalmente 32-bit per elemento
    float floatArray[3] = {3.14f, 1.618f, 2.718f}; // Numeri con decimali

    // Array di double: Generalmente 64-bit per elemento
    double doubleArray[2] = {6.022e23, 3.0e8}; // Maggiore precisione

    // Array di uint8_t (8-bit per elemento)
    uint8_t byteArray[4] = {0xFF, 0x7F, 0x00, 0x80}; // Sequenza di byte

    // Stampa di esempio per alcuni valori
    printf("char: %c\n", a);
    printf("unsigned char: %u\n", b);
    printf("int: %d\n", c);
    printf("unsigned int: %u\n", d);
    printf("float: %f\n", i);
    printf("double: %lf\n", j);

    return 0;
}
