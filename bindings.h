#define WIN32_LEAN_AND_MEAN
#include <Windows.h>
#include <Wingdi.h>

//XXX: stupid hacks below
#include <GL/gl.h>

static const int ALLES_DOOF1 = 123;
#define ALLES_DOOF2       0x123

 //XXX: defines mit cast gehen sterben
#define ALLES_DOOF3       ((int)0x123)
