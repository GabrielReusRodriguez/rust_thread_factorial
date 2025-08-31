#!/usr/bin/env bash 

MIN_RANGE=1
MAX_RANGE=100000
NUM_LINES=3000
HOME_FOLDER="$( cd "$( dirname "${BASH_SOURCE[0]}" )" > /dev/null && pwd )"
OUT_FILE="${HOME_FOLDER}/../data/numbers.txt"

#Generamos numeros aleatorios con shuf.
shuf --input-range=${MIN_RANGE}-${MAX_RANGE} --head-count=${NUM_LINES} --output=${OUT_FILE} --repeat

#Eliminamos el \n final que genera el shuf en el fichero con truncate
truncate -s -1 ${OUT_FILE}