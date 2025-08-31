#include <stdio.h>
#include <stdlib.h>
#include <errno.h>

#include "data.h"

#define NUM_VALUES 30

int main(int argc, char ** argv){
    
    data_type   *data;
    size_t      vector_size;

    argc++;
    data = NULL;
    vector_size = 0;
    if (load_data(argv[1], &data, &vector_size) != EXIT_SUCCESS){
        printf("Exiting with error...\n");
        return(EXIT_FAILURE);
    }
    for (int i = 0; i <= NUM_VALUES; i++){
        unsigned long long factorial = 1;
        if (i != 0){
            for (int j = 0 ; j < i; j++){
                factorial = factorial * (j + 1);
            }
        }
        printf("El factorial de: %d es : %llu\n", i, factorial);
    }
    free(data);
    return (EXIT_SUCCESS);
}