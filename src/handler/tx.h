#pragma once

#include <stddef.h>   // size_t
#include <stdbool.h>  // bool
#include <stdint.h>   // uint*_t
#include "chacha.h"

#include "../types.h"
#include "../tx.h"

int change_stage(uint8_t new_stage);
int init_tx(uint8_t *header_hash);
int add_t_input_amount(uint64_t amount);
int add_t_output(t_out_t *output);
int add_s_output(s_out_t *output);
int set_sapling_net(int64_t balance);
int set_t_merkle_proof(t_proofs_t *t_proofs);
int set_s_merkle_proof(s_proofs_t *s_proofs);
int sighash(uint8_t *sighash, uint8_t *txin_sig_digest);
int sign_sapling();
int get_sighash();

int prf_chacha(cx_chacha_context_t *rng, uint8_t *v, size_t len);
