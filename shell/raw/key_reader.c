#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <termios.h>
#include <unistd.h>

#include "key_reader.h"

struct termios orig;

inline int CtrlKey(char c) {
	return c & 0x1f;
}

void KillProgram(const char *s) {
	write(STDOUT_FILENO, "\x1b[2J", 4);
	write(STDOUT_FILENO, "\x1b[H", 3);
	perror(s);
	exit(1);
}

void DisableRawMode() {
	if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig) == -1) {
		KillProgram("tcsetattr");
	}
}

void EnableRawMode() {
	if (tcgetattr(STDIN_FILENO, &orig) == -1) {
		KillProgram("tcgetattr");
	}

	atexit(DisableRawMode);

	struct termios raw = orig;

	raw.c_iflag &= ~(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
	raw.c_oflag &= ~(OPOST);
	raw.c_cflag |= (CS8);
	raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
	raw.c_cc[VMIN] = 0;
	raw.c_cc[VTIME] = 1;

	if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw) == -1) {
		KillProgram("tcsetattr");
	}
}

void ReadBytes(char* c) {
	if (read(STDIN_FILENO, c, 1) == -1 && errno != EAGAIN) {
		KillProgram("read");
	}
}

bool CharEqual(char a, int code) {
	return (int) a == code;
}

void FlushStdout() {
	fflush(stdout);
}
