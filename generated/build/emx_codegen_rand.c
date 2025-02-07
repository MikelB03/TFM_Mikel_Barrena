
#define		EMX_NO_LONG_LONG

#include <math.h>

#include "emx_codegen_rand.h"
#include "emx_codegen_intern.h"

static void mt_init_genrand(EMX_uint32_t s);
static void mt_init_by_array(const EMX_uint32_t init_key[], EMX_uint32_t key_length);
static EMX_uint32_t mt_genrand_int32(void);
static EMX_int32_t mt_genrand_int31(void);
static EMX_double_t mt_genrand_real1(void);
static EMX_double_t mt_genrand_real2(void);
static EMX_double_t mt_genrand_real3(void);
static EMX_double_t mt_genrand_res53(void);
static void urand_seed(EMX_uint32_t seed);
static EMX_uint32_t urand_genrand_int31(void);


/* Period parameters */
#define MT_N ((EMX_uint32_t)624)
#define MT_M ((EMX_uint32_t)397)
#define MT_MATRIX_A ((EMX_uint32_t)0x9908b0dfUL)   /* constant vector a */
#define MT_UPPER_MASK ((EMX_uint32_t)0x80000000UL) /* most significant w-r bits */
#define MT_LOWER_MASK ((EMX_uint32_t)0x7fffffffUL) /* least significant r bits */

static EMX_uint32_t mt[MT_N]; /* the array for the state vector  */
static EMX_uint32_t mti=MT_N+1; /* mti==MT_N+1 means mt[MT_N] is not initialized */

/* initializes mt[MT_N] with a seed */
static void mt_init_genrand(EMX_uint32_t s) {
    mt[0]= s;
    for (mti=1; mti<MT_N; mti++) {
        mt[mti] =
	    (((EMX_uint32_t)1812433253UL * (mt[mti-1] ^ (mt[mti-1] >> 30))) + mti);
        /* See Knuth TAOCP Vol2. 3rd Ed. P.106 for multiplier. */
        /* In the previous versions, MSBs of the seed affect   */
        /* only MSBs of the array mt[].                        */
        /* 2002/01/09 modified by Makoto Matsumoto             */
    }
}

/* initialize by an array with array-length */
/* init_key is the array for initializing keys */
/* key_length is its length */
static void mt_init_by_array(const EMX_uint32_t init_key[], EMX_uint32_t key_length) {
    EMX_uint32_t i, j, k;
    mt_init_genrand((EMX_uint32_t)19650218UL);
    i=1; j=0;
    k = (MT_N>key_length ? MT_N : key_length);
    for (; k != 0; k--) {
        mt[i] = (mt[i] ^ ((mt[i-1] ^ (mt[i-1] >> 30)) * (EMX_uint32_t)1664525UL))
          + init_key[j] + j; /* non linear */
        i++; j++;
        if (i>=MT_N) { 
			mt[0] = mt[MT_N-1]; 
			i=1; 
		}
        if (j>=key_length) {
			j=0;
		}
    }
    for (k=MT_N-1; k != 0; k--) {
        mt[i] = (mt[i] ^ ((mt[i-1] ^ (mt[i-1] >> 30)) * (EMX_uint32_t)1566083941UL))
          - i; /* non linear */
        i++;
        if (i>=MT_N) { mt[0] = mt[MT_N-1]; i=1; }
    }

    mt[0] = (EMX_uint32_t)0x80000000UL; /* MSB is 1; assuring non-zero initial array */
}

/* generates a random number on [0,0xffffffff]-interval */
static EMX_uint32_t mt_genrand_int32(void) {
    EMX_uint32_t y;
    static EMX_uint32_t mag01[2]={(EMX_uint32_t)0x0UL, (EMX_uint32_t)MT_MATRIX_A};
    /* mag01[x] = x * MT_MATRIX_A  for x=0,1 */

    if (mti >= MT_N) { /* generate MT_N words at one time */
        EMX_uint32_t kk;

        if (mti == (MT_N+1)) {   /* if mt_init_genrand() has not been called, */
            mt_init_genrand((EMX_uint32_t)5489UL); /* a default initial seed is used */
		}

        for (kk=0; kk < (MT_N-MT_M); kk++) {
            y = (mt[kk]&MT_UPPER_MASK)|(mt[kk+1]&MT_LOWER_MASK);
            mt[kk] = mt[kk+MT_M] ^ (y >> 1) ^ mag01[y & (EMX_uint32_t)0x1UL];
        }
        for ( ; kk < (MT_N-1); kk++) {
            y = (mt[kk]&MT_UPPER_MASK)|(mt[kk+1]&MT_LOWER_MASK);
            mt[kk] = mt[(kk+MT_M)-MT_N] ^ (y >> 1) ^ mag01[y & (EMX_uint32_t)0x1UL];
        }
        y = (mt[MT_N-1]&MT_UPPER_MASK)|(mt[0]&MT_LOWER_MASK);
        mt[MT_N-1] = mt[MT_M-1] ^ (y >> 1) ^ mag01[y & (EMX_uint32_t)0x1UL];

        mti = 0;
    }

    y = mt[mti];
    mti++;

    /* Tempering */
    y ^= (y >> 11);
    y ^= (y << 7) & (EMX_uint32_t)0x9d2c5680UL;
    y ^= (y << 15) & (EMX_uint32_t)0xefc60000UL;
    y ^= (y >> 18);

    return y;
}

/* generates a random number on [0,0x7fffffff]-interval */
static EMX_int32_t mt_genrand_int31(void) {
    return (EMX_int32_t)(mt_genrand_int32()>>1);
}

/* generates a random number on [0,1]-real-interval */
static EMX_double_t mt_genrand_real1(void) {
    return (EMX_double_t)mt_genrand_int32()*(1.0/4294967295.0);
    /* divided by 2^32-1 */
}

/* generates a random number on [0,1)-real-interval */
static EMX_double_t mt_genrand_real2(void) {
    return (EMX_double_t)mt_genrand_int32()*(1.0/4294967296.0);
    /* divided by 2^32 */
}

/* generates a random number on (0,1)-real-interval */
static EMX_double_t mt_genrand_real3(void) {
    return (((EMX_double_t)mt_genrand_int32()) + 0.5)*(1.0/4294967296.0);
    /* divided by 2^32 */
}

/* generates a random number on [0,1) with 53-bit resolution*/
static EMX_double_t mt_genrand_res53(void) {
    EMX_uint32_t a=mt_genrand_int32()>>5, b=mt_genrand_int32()>>6;
    return (((EMX_double_t)a*67108864.0) + (EMX_double_t)b) * (1.0/9007199254740992.0);
}
/* These real versions are due to Isaku Wada, 2002/01/09 added */


EMX_double_t EMX_rand_mt(void) {
	return mt_genrand_res53();
}

/* generates a integer random number on [1, nmax] */
EMX_double_t EMX_randi_mt(EMX_double_t nmax) {
	return floor(EMX_rand_mt() * nmax) + 1.0;
}

static EMX_uint32_t urand_data = 0;

static void urand_seed(EMX_uint32_t seed) {
	urand_data = seed;
}

static EMX_uint32_t urand_genrand_int31(void) {
	urand_data = ((EMX_uint32_t)843314861ul * urand_data) + (EMX_uint32_t)453816693ul;

	if (urand_data >= (EMX_uint32_t)2147483648ul) {
		urand_data -= (EMX_uint32_t)2147483648ul;
	}

	return urand_data;
}

EMX_double_t EMX_rand_urand(void) {
	return (EMX_double_t)urand_genrand_int31() * (1.0 / (EMX_double_t)0x7fffffff);
}

