#include <stdbool.h>

#ifndef SHELL_KEY_READER_H
#define SHELL_KEY_READER_H

inline int CtrlKey(char c);

void KillProgram(const char* s);

void DisableRawMode();

void EnableRawMode();

void ReadBytes(char* c);

void FlushStdout();

bool CharEqual(char a, int code);

#endif//SHELL_KEY_READER_H
