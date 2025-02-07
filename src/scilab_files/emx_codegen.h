#ifndef __EMX_CODEGEN_H__
#define __EMX_CODEGEN_H__

#include <stddef.h>
#include <limits.h>


#ifdef __cplusplus
	/* C++ */

	typedef bool EMX_bool_t;
#elif __STDC_VERSION__ >= 199901L
	/* >= C99 */

	typedef _Bool EMX_bool_t;
#else
	/* C89/C90 */

	typedef unsigned char EMX_bool_t;
	#define		EMX_INLINE
#endif

#ifndef EMX_INLINE
	#define		EMX_INLINE		inline
#endif


#if SCHAR_MAX == 127

typedef signed char EMX_int8_t;
typedef unsigned char EMX_uint8_t;

#define EMX_INT8_MIN 	SCHAR_MIN
#define EMX_INT8_MAX 	SCHAR_MAX
#define EMX_UINT8_MAX 	UCHAR_MAX
#define EMX_PRId8		"d"
#define EMX_PRIi8		"i"
#define EMX_PRIo8		"o"
#define EMX_PRIu8		"u"
#define EMX_PRIx8		"x"
#define EMX_PRIX8		"X"

#else
#	error Invalid 8-bit integer
#endif


#if SHRT_MAX == 32767

typedef signed short EMX_int16_t;
typedef unsigned short EMX_uint16_t;

#define EMX_INT16_MIN 	SHRT_MIN
#define EMX_INT16_MAX 	SHRT_MAX
#define EMX_UINT16_MAX 	USHRT_MAX
#define EMX_PRId16		"d"
#define EMX_PRIi16		"i"
#define EMX_PRIo16		"o"
#define EMX_PRIu16		"u"
#define EMX_PRIx16		"x"
#define EMX_PRIX16		"X"

#else
#	error Invalid 16-bit integer
#endif


#if INT_MAX == 2147483647

typedef signed int EMX_int32_t;
typedef unsigned int EMX_uint32_t;

#define EMX_INT32_MIN 	INT_MIN
#define EMX_INT32_MAX 	INT_MAX
#define EMX_UINT32_MAX 	UINT_MAX
#define EMX_PRId32		"d"
#define EMX_PRIi32		"i"
#define EMX_PRIo32		"o"
#define EMX_PRIu32		"u"
#define EMX_PRIx32		"x"
#define EMX_PRIX32		"X"

#elif LONG_MAX == 2147483647

typedef signed long EMX_int32_t;
typedef unsigned long EMX_uint32_t;

#define EMX_INT32_MIN 	LONG_MIN
#define EMX_INT32_MAX 	LONG_MAX
#define EMX_UINT32_MAX 	ULONG_MAX
#define EMX_PRId32		"ld"
#define EMX_PRIi32		"li"
#define EMX_PRIo32		"lo"
#define EMX_PRIu32		"lu"
#define EMX_PRIx32		"lx"
#define EMX_PRIX32		"lX"

#else
#	error Invalid 32-bit integer
#endif


#if (!defined(LLONG_MAX)) && (!defined(EMX_NO_LONG_LONG))
#define		EMX_NO_LONG_LONG
#endif


#if LONG_MAX == 9223372036854775807

typedef signed long EMX_int64_t;
typedef unsigned long EMX_uint64_t;

#define EMX_INT64_MIN 	LONG_MIN
#define EMX_INT64_MAX 	LONG_MAX
#define EMX_UINT64_MAX 	ULLONG_MAX
#define EMX_PRId64		"ld"
#define EMX_PRIi64		"li"
#define EMX_PRIo64		"lo"
#define EMX_PRIu64		"lu"
#define EMX_PRIx64		"lx"
#define EMX_PRIX64		"lX"
  
#elif !defined(EMX_NO_LONG_LONG)

typedef signed long long EMX_int64_t;
typedef unsigned long long EMX_uint64_t;

#define EMX_INT64_MIN 	LLONG_MIN
#define EMX_INT64_MAX 	LLONG_MAX
#define EMX_UINT64_MAX 	ULLONG_MAX
#define EMX_PRId64		"lld"
#define EMX_PRIi64		"lli"
#define EMX_PRIo64		"llo"
#define EMX_PRIu64		"llu"
#define EMX_PRIx64		"llx"
#define EMX_PRIX64		"llX"

#endif


typedef size_t EMX_size_t;
typedef double EMX_double_t;
typedef float EMX_float_t;

typedef EMX_uint32_t EMX_funchandle_t;

typedef EMX_uint8_t EMX_ndim_t;
typedef EMX_uint32_t EMX_matrixtype_t;

typedef enum {
	matrix_type_0 = 0,

	matrix_type_float = 0,
	matrix_type_int = 1,
	matrix_type_bool = 2,
	matrix_type_char = 3,
	matrix_type_ac_fixed = 4,
	matrix_type_count = 5,
	
	matrix_type_mask = 0xF,
	matrix_type_unsigned = 0x40,
	matrix_type_complex = 0x80,
	
	matrix_type_width = 0x100,
	matrix_type_width_shift = 8,
	matrix_type_width_mask = 0xFF * matrix_type_width,

	matrix_type_fraction = 0x10000,
	matrix_type_fraction_shift = 16,
	matrix_type_fraction_mask = 0xFF * matrix_type_fraction
} matrix_type_t;


#define matrix_type_make_width(size)		((EMX_matrixtype_t)((EMX_matrixtype_t)(size) << matrix_type_width_shift))

#define matrix_type_ac_fixed(fraction)		((EMX_matrixtype_t)matrix_type_ac_fixed | ((EMX_matrixtype_t)(fraction) << matrix_type_fraction_shift))

#define matrix_type_get_fraction(type)		(((EMX_matrixtype_t)(type) & (EMX_matrixtype_t)matrix_type_fraction_mask) >> matrix_type_fraction_shift)

#define matrix_type_get_width(type)			(((EMX_matrixtype_t)(type) & (EMX_matrixtype_t)matrix_type_width_mask) >> matrix_type_width_shift)


#endif
