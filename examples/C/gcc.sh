#!/usr/bin/env bash
#
# handy script for fast building, running & debugging notcurses c programs

CMD=$1
SOURCE=$2

BINS_DIR="bin/"

case "$CMD" in
	"build"|"b")
		CMD="build"
		echo ">> building..."
		;;
	"run"|"r")
		CMD="run"
		echo ">> running..."
		;;
	"valgrind"|"v")
		CMD="valgrind"
		echo -e ">> running with valgrind..."
		;;
	*)
		echo -e "usage:"
		echo -e "\tgcc.sh [build|run|valgrind] source_file.c\n"
		echo -e "\tgcc.sh [b|r|v] source_file.c"
		exit
		;;
esac

if [[ $SOURCE == "" ]]; then
	echo "need the name of the C source file"
	exit
fi

mkdir -p ${BINS_DIR}

BINFILENAME="${SOURCE%.*}"

gcc ${SOURCE} -o ${BINS_DIR}${BINFILENAME} $(pkg-config --cflags --libs notcurses)
GCC_RES="$?";

if [[ $GCC_RES == 0 ]]; then
	if [[ $CMD == "run" ]]; then
		${BINS_DIR}${BINFILENAME}
	elif [[ $CMD == "valgrind" ]] ; then
		valgrind --leak-check=full -s ${BINS_DIR}${BINFILENAME}
	fi
	RES="$?";
	echo -e "\n>> Done!\n>> Return value: $RES"
else
	echo -e "\n>>Compilation failed with error: $GCC_RES"
fi
