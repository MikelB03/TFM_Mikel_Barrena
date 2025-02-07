
#include "emx_codegen.h"
#include "emx_codegen_intern.h"

#include <math.h>
#include <stdlib.h>
#include <assert.h>

const char* EMX_matrix_type_string[matrix_type_count] = {
	"float",
	"int",
	"bool",
	"char",
	"ac_fixed",
};

EMX_size_t EMX_elementsize(const matrix_rtti_t* v) {
	return matrix_type_get_width(v->type) * ((v->type & (EMX_matrixtype_t)matrix_type_complex) != 0 ? (EMX_matrixtype_t)2 : (EMX_matrixtype_t)1);
}

void* EMX_advance(const matrix_rtti_t* v, EMX_size_t count, EMX_bool_t imag) {
	void* p = (EMX_int8_t*)v->data + (EMX_elementsize(v) * count);

	if (imag != 0) {
		p = (EMX_int8_t*)p + (EMX_elementsize(v)>>1);
	}

	return p;
}

void* EMX_advance_2D(const matrix_rtti_t* v, EMX_size_t i1, EMX_size_t i2, EMX_bool_t imag) {
	return EMX_advance(v, (i2 * v->ssize[0]) + i1, imag);
}
void* EMX_advance_3D(const matrix_rtti_t* v, EMX_size_t i1, EMX_size_t i2, EMX_size_t i3, EMX_bool_t imag) {
	return EMX_advance(v, (((i3 * v->ssize[1]) + i2) * v->ssize[0]) + i1, imag);
}

matrix_rtti_t EMX_submatrix(const matrix_rtti_t* v, EMX_size_t dim0pos) {
	EMX_ndim_t i;
	
	assert(v->ndim > 1);
	assert(dim0pos < v->dsize[v->ndim-1]);
	
	for (i=0; i<(v->ndim-1); ++i) {
		dim0pos *= v->ssize[i];
	}

	{
		matrix_rtti_t v2 = *v;
		
		v2.ndim -= 1;
		v2.data = EMX_advance(&v2, dim0pos, 0);
		
		return v2;
	}
}

EMX_double_t EMX_getdouble(const matrix_rtti_t* v, void* p) {
	/* printf("%p %p\n", v->data, p); */
	EMX_double_t d = 0.0;

	switch ((v->type & ((EMX_matrixtype_t)matrix_type_mask | (EMX_matrixtype_t)matrix_type_width_mask | (EMX_matrixtype_t)matrix_type_unsigned))) {
		case (EMX_matrixtype_t)matrix_type_float | matrix_type_make_width(sizeof(EMX_float_t)):
			d = (EMX_double_t)*(EMX_float_t*)p; break;
		case (EMX_matrixtype_t)matrix_type_float | matrix_type_make_width(sizeof(EMX_double_t)):
			d = (EMX_double_t)*(EMX_double_t*)p; break;

		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_int8_t)):
			d = (EMX_double_t)*(EMX_int8_t*)p; break;
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_uint8_t)) | (EMX_matrixtype_t)matrix_type_unsigned:
			d = (EMX_double_t)*(EMX_uint8_t*)p; break;

		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_int16_t)):
			d = (EMX_double_t)*(EMX_int16_t*)p; break;
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_uint16_t)) | (EMX_matrixtype_t)matrix_type_unsigned:
			d = (EMX_double_t)*(EMX_uint16_t*)p; break;

		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_int32_t)):
			d = (EMX_double_t)*(EMX_int32_t*)p; break;
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_uint32_t)) | (EMX_matrixtype_t)matrix_type_unsigned:
			d = (EMX_double_t)*(EMX_uint32_t*)p; break;

#ifdef EMX_INT64_MAX
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_int64_t)):
			d = (EMX_double_t)*(EMX_int64_t*)p; break;
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_uint64_t)) | (EMX_matrixtype_t)matrix_type_unsigned:
			d = (EMX_double_t)*(EMX_uint64_t*)p; break;
#endif

		case (EMX_matrixtype_t)matrix_type_char | matrix_type_make_width(sizeof(char)):
			d = (EMX_double_t)*(char*)p; break;
		case (EMX_matrixtype_t)matrix_type_bool | matrix_type_make_width(sizeof(EMX_bool_t)):
			d = (EMX_double_t)*(EMX_bool_t*)p; break;
			
		default:
			if ((v->type & (EMX_matrixtype_t)matrix_type_mask) == matrix_type_ac_fixed) {
				EMX_uint32_t width = matrix_type_get_width(v->type);
				EMX_uint32_t frac = matrix_type_get_fraction(v->type);

				/* printf("Q%d.%d\n", width*8, frac); */
				
				EMX_uint32_t w;
				EMX_uint32_t* p2 = (EMX_uint32_t*)p;
				for (w=0; w<width; w+=4) {
					EMX_double_t d2 = (EMX_double_t)p2[w/4] / pow(2.0, frac-(w*8));
					
					d += d2;
					
					/* printf("%X %f\n", *(uint32_t*)p2, d2); */
				}
			} else {
				assert((0 != 0) && "Data type not supported");
			}
		
			break;
	}
	

	/* printf("name: %s\n", matrix_type_string[v->type & matrix_type_mask]); */
	/* printf("width: %d\n", matrix_type_get_width(v->type)); */
	
	return d;
}

void EMX_setdouble(const matrix_rtti_t* v, void* p, EMX_double_t d) {
	/* printf("%p %p\n", v->data, p); */

	switch ((v->type & ((EMX_matrixtype_t)matrix_type_mask | (EMX_matrixtype_t)matrix_type_width_mask | (EMX_matrixtype_t)matrix_type_unsigned))) {
		case (EMX_matrixtype_t)matrix_type_float | matrix_type_make_width(sizeof(EMX_float_t)):
			*(EMX_float_t*)p = (EMX_float_t)d; break;
		case (EMX_matrixtype_t)matrix_type_float | matrix_type_make_width(sizeof(EMX_double_t)):
			*(EMX_double_t*)p = (EMX_double_t)d; break;

		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_int8_t)):
			*(EMX_int8_t*)p = (EMX_int8_t)d; break;
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_uint8_t)) | (EMX_matrixtype_t)matrix_type_unsigned:
			*(EMX_uint8_t*)p = (EMX_uint8_t)d; break;

		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_int16_t)):
			*(EMX_int16_t*)p = (EMX_int16_t)d; break;
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_uint16_t)) | (EMX_matrixtype_t)matrix_type_unsigned:
			*(EMX_uint16_t*)p = (EMX_uint16_t)d; break;

		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_int32_t)):
			*(EMX_int32_t*)p = (EMX_int32_t)d; break;
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_uint32_t)) | (EMX_matrixtype_t)matrix_type_unsigned:
			*(EMX_uint32_t*)p = (EMX_uint32_t)d; break;

#ifdef EMX_INT64_MAX
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_int64_t)):
			*(EMX_int64_t*)p = (EMX_int64_t)d; break;
		case (EMX_matrixtype_t)matrix_type_int | matrix_type_make_width(sizeof(EMX_uint64_t)) | (EMX_matrixtype_t)matrix_type_unsigned:
			*(EMX_uint64_t*)p = (EMX_uint64_t)d; break;
#endif

		case (EMX_matrixtype_t)matrix_type_char | matrix_type_make_width(sizeof(char)):
			*(char*)p = (char)d; break;
		case (EMX_matrixtype_t)matrix_type_bool | matrix_type_make_width(sizeof(EMX_bool_t)):
			*(EMX_bool_t*)p = (EMX_bool_t)d; break;
			
		default:
			assert((0 != 0) && "Data type not supported");
			break;
		
	}
}

void EMX_getString(const matrix_rtti_t* fn, char* str, EMX_size_t max_length) {
	EMX_size_t i;
	EMX_size_t length = fn->dsize[1];

	if (length >= max_length) {
		length = max_length-1;
	}

	for (i=0; i < length; ++i) {
		str[i] = (char)EMX_getdouble(fn, EMX_advance_2D(fn, i, 0, 0));
	}
	str[length] = 0;
}
