NAME		=	factorial
OBJ_DIR		=	./obj
BIN_DIR		=	./bin
SRC_DIR		=	./src
CC_FLAGS	= 	-Wall -Werror -Wextra -MMD -MP
LINK_FLAGS	=

SRC			=	data.c			\
				main.c			\
				trim.c			
				

SRCS		= $(patsubst %.c, ${SRC_DIR}/%.c, ${SRC})
OBJS		= $(patsubst %.c, ${OBJ_DIR}/%.o, ${SRC})
DEPS		= $(patsubst %.c, ${OBJ_DIR}/%.d, ${SRC})

all: ${BIN_DIR}/${NAME}

${OBJ_DIR}:
	@mkdir -p ${OBJ_DIR}

${BIN_DIR}:
	@mkdir -p ${BIN_DIR}

${BIN_DIR}/${NAME}: ${BIN_DIR} ${OBJ_DIR} ${OBJS} Makefile
	${CC} ${LINK_FLAGS} ${OBJS} -o ${BIN_DIR}/${NAME}

${OBJ_DIR}/%.o : ${SRC_DIR}/%.c  Makefile
	${CC} ${CC_FLAGS} -c $< -o $@

clean:
	@rm -rf ${OBJ_DIR}

fclean: clean
	@rm -rf ${BIN_DIR}

re: fclean all

.PHONY: all clean fclean re

-include ${DEPS}