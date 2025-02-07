#ifndef __EMX_CODEGEN_INTERN_H__
#define __EMX_CODEGEN_INTERN_H__

#include "emx_codegen.h"
#include <stdio.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif


typedef struct {
	EMX_matrixtype_t type;
	EMX_ndim_t ndim;
	EMX_size_t* dsize;
	const EMX_size_t* ssize;
	void* data;
	EMX_bool_t isconst;
} matrix_rtti_t;



extern const char* EMX_matrix_type_string[matrix_type_count];

extern EMX_size_t EMX_elementsize(const matrix_rtti_t* v);

extern void* EMX_advance(const matrix_rtti_t* v, EMX_size_t count, EMX_bool_t imag);
extern void* EMX_advance_2D(const matrix_rtti_t* v, EMX_size_t i1, EMX_size_t i2, EMX_bool_t imag);
extern void* EMX_advance_3D(const matrix_rtti_t* v, EMX_size_t i1, EMX_size_t i2, EMX_size_t i3, EMX_bool_t imag);

extern matrix_rtti_t EMX_submatrix(const matrix_rtti_t* v, EMX_size_t dim0pos);

extern EMX_double_t EMX_getdouble(const matrix_rtti_t* v, void* p);
extern void EMX_setdouble(const matrix_rtti_t* v, void* p, EMX_double_t d);

extern void EMX_getString(const matrix_rtti_t* fn, char* str, EMX_size_t max_length);


#ifndef EMX_PATH_MAX
	#ifdef PATH_MAX
		#define		EMX_PATH_MAX			PATH_MAX
	#else
		#define		EMX_PATH_MAX			256
	#endif
#endif

static EMX_INLINE matrix_rtti_t EMX_make_rtti(EMX_matrixtype_t v_type, EMX_ndim_t v_ndim, EMX_size_t* v_dsize, const EMX_size_t* v_ssize, void* v_data);
static EMX_INLINE matrix_rtti_t EMX_make_const_rtti(EMX_matrixtype_t v_type, EMX_ndim_t v_ndim, const EMX_size_t* v_dsize, const EMX_size_t* v_ssize, const void* v_data);


static EMX_INLINE matrix_rtti_t EMX_make_rtti(EMX_matrixtype_t v_type, EMX_ndim_t v_ndim, EMX_size_t* v_dsize, const EMX_size_t* v_ssize, void* v_data) {
	matrix_rtti_t rtti = {0};

	rtti.type = v_type;
	rtti.ndim = v_ndim;
	rtti.dsize = v_dsize;
	rtti.ssize = v_ssize;
	rtti.data = v_data;

	return rtti;
}
static EMX_INLINE matrix_rtti_t EMX_make_const_rtti(EMX_matrixtype_t v_type, EMX_ndim_t v_ndim, const EMX_size_t* v_dsize, const EMX_size_t* v_ssize, const void* v_data) {
	matrix_rtti_t rtti = {0};

	rtti.type = v_type;
	rtti.ndim = v_ndim;
	rtti.dsize = (EMX_size_t*)v_dsize;
	rtti.ssize = v_ssize;
	rtti.data = (void*)v_data;

	return rtti;
}

void EMX_load_fp(FILE* fp, const char* debug_file_name, EMX_matrixtype_t v_type, EMX_ndim_t v_ndim, EMX_size_t* v_dsize, const EMX_size_t* v_ssize, const void* v_data);
void EMX_save_fp(FILE* fp, EMX_matrixtype_t v_type, EMX_ndim_t v_ndim, const EMX_size_t* v_dsize, const EMX_size_t* v_ssize, const void* v_data);

typedef struct {
	EMX_uint8_t		id[4];
	EMX_uint32_t	type;
	EMX_uint32_t	ndim;
	EMX_uint32_t	dim[16];
} emxdat_header_t;

void emxdat_header_byteorder(emxdat_header_t *pHdr);

#ifdef __cplusplus
}
#endif

#endif
