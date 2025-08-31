#include <stdio.h>
#include <stdlib.h>
#include <errno.h>

#include "data.h"

static unsigned int  get_data_size(char * const file_path, size_t *size){

    size_t      i;
    FILE        *f;
    char        line[BUFFER_SIZE];

    i = 0;
    f = fopen(file_path, "r");
    if (f == NULL){
        perror("ERROR: ");
        return (EXIT_FAILURE);
    }
    while(fgets(line, sizeof(line), f) != NULL){
        i++;
    }
    fclose(f);
    *size = i;
    return (EXIT_SUCCESS);
}

unsigned int load_data(char * const file_path, data_type **data, size_t *vector_size){

    size_t      i;
    FILE        *f;
    char        data_line[BUFFER_SIZE];
    data_type   *ptr;

    i = 0;
    if (get_data_size(file_path, vector_size) != EXIT_SUCCESS){
        perror("ERROR loading data: ");
        return (EXIT_FAILURE);
    }
    //Reservamos memoria....
    ptr = (data_type *)malloc(*vector_size*sizeof(data_type));
    if (ptr == NULL){
        perror("ERROR loading data: ");
        return (EXIT_FAILURE);
    }
    f = fopen(file_path, "r");
    if (f == NULL){
        perror("ERROR loading data: ");
        return (EXIT_FAILURE);
    }
    while(fgets(data_line, sizeof(data_line), f) != NULL){
        i++;
    }
    fclose(f);
    *data = ptr;
    return (EXIT_SUCCESS);
}
