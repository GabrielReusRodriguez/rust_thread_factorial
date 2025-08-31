#ifndef DATA_H
    #define DATA_H

    #define BUFFER_SIZE 128

    typedef unsigned int data_type;

    unsigned int load_data(char * const file_path, data_type **data, size_t *vector_size);

#endif