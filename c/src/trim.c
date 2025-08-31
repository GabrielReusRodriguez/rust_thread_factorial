#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static unsigned int char_in_set(char c, char const *set){

    size_t  i;

    i = 0;
    while (set[i] != '\0'){
        if (set[i] == c){
            return (1);
        }
        i++;
    }
    return (0);
}

static size_t get_init_pos(const char * str, char const *characters){

    size_t  i;

    i = 0;
    while(str[i] != '\0' && char_in_set(str[i], characters) == 1){
        i++;
    }
    return (i);
}

static size_t get_end_pos(const char * str, size_t len, char const *characters){

    int  i;

    i = (int)len - 1;
    while( i >= 0 && char_in_set(str[i], characters) == 1){
        i--;
    }
    return (i);
}

static  unsigned int substring(const char *str, size_t len, size_t init, size_t end, char **substring){

    int     new_len;
    char    *new_str;

    new_len = (int) len - (init + (len - end));
    if (new_len < 0){
        *substring = NULL;
        return (EXIT_SUCCESS);
    }
    new_str = (char *)malloc(new_len * sizeof(char));
    if (new_str == NULL){
        perror("Error at substring: ");
        return (EXIT_FAILURE);
    }
    if (strncpy(new_str, str + init, new_len) == NULL){
        perror("Error at substring: ");
        free(new_str);
        *substring = NULL;
        return (EXIT_FAILURE);
    }
    *substring = new_str;
    return (EXIT_SUCCESS);
}

unsigned int trim(char **str, char const *characters){
    
    size_t  len;
    size_t  init;
    size_t  end;
    char    *new_str;

    if (str == NULL || *str == NULL){
        fprintf(stderr, "ERROR trimming string\n");
        return (EXIT_FAILURE);
    }
    len = strlen(*str);
    if (len == 0){
        return (EXIT_SUCCESS);
    }
    init = get_init_pos(*str, characters);
    end = get_end_pos(*str, len, characters);
    if (substring(*str, len, init, end, &new_str) != EXIT_SUCCESS){
        return (EXIT_FAILURE);
    }
    free (*str);
    *str = new_str;
    // Hago el substring.
    return (EXIT_SUCCESS);

}
