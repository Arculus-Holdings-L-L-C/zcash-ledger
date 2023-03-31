#pragma once

#include "../types.h"

typedef struct {
    fq_t u;
    fq_t v;
} affine_point_t;

typedef struct {
    fq_t u;
    fq_t v;
    fq_t z;
    fq_t t1;
    fq_t t2;
} extended_point_t;

typedef struct {
    fq_t vpu;
    fq_t vmu;
    fq_t z;
    fq_t t2d;
} extended_niels_point_t;

int ext_double(extended_point_t *v);
int ext_add(extended_point_t *v, extended_niels_point_t *a);
int to_affine(affine_point_t *v, extended_point_t *a);

int jubjub_test(extended_point_t *v);
