#include <stdio.h>

#define NUM_VALUES 30

int main(int argc, char ** argv){
    
    for (int i = 0; i <= NUM_VALUES; i++){
        unsigned long long factorial = 1;
        if (i != 0){
            for (int j = 0 ; j < i; j++){
                factorial = factorial * (j + 1);
            }
        }
        printf("El factorial de: %d es : %llu\n", i, factorial);
    }
    return (0);
}