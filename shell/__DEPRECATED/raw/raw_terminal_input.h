#include <stdbool.h>

#ifndef RAW_TERMINAL_INPUT
#define RAW_TERMINAL_INPUT

inline int CtrlKey(char c);

void KillProgram(const char* s);

void DisableRawMode();

void EnableRawMode();

void ReadBytes(char* c);

void FlushStdout();

bool CharEqual(char a, int code);

#endif//RAW_TERMINAL_INPUT
