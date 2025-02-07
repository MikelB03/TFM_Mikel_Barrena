#ifndef __EMX_CODEGEN_RAND_H__
#define __EMX_CODEGEN_RAND_H__

#include "emx_codegen.h"

#ifdef __cplusplus
extern "C" {
#endif

extern EMX_double_t EMX_rand_mt(void);
extern EMX_double_t EMX_randi_mt(EMX_double_t maxn);
extern EMX_double_t EMX_rand_urand(void);

#ifdef __cplusplus
}
#endif

#endif
