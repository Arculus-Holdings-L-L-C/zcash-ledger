/*****************************************************************************
 *   Zcash Ledger App.
 *   (c) 2022 Hanh Huynh Huu.
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *****************************************************************************/

#include <stdint.h>
#include <stdbool.h>

#include "dispatcher.h"
#include "../constants.h"
#include "../globals.h"
#include "../types.h"
#include "../io.h"
#include "../sw.h"
#include "../tx.h"
#include "../common/buffer.h"
#include "../crypto/key.h"
#include "../handler/get_version.h"
#include "../handler/get_app_name.h"
#include "../handler/get_fvk.h"
#include "../handler/get_address.h"
#include "../handler/tx.h"
#include "../handler/test_math.h"
#include "../helper/send_response.h"

int apdu_dispatcher(const command_t *cmd) {
    if (cmd->cla != CLA) {
        return io_send_sw(SW_CLA_NOT_SUPPORTED);
    }

    switch (cmd->ins) {
        case GET_VERSION:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }

            return handler_get_version();
        case GET_APP_NAME:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }

            return handler_get_app_name();
        
        case INITIALIZE:
            if (cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }

            crypto_derive_spending_key(cmd->p1);
            return helper_send_response_bytes(NULL, 0);

        case GET_FVK:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }

            return handler_get_fvk();
        
        case GET_ADDRESS:
            if (cmd->p1 > 1 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }

            return handler_get_address(cmd->p1 == 1);
        
        case INIT_TX:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != 32)
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            return init_tx(cmd->data);

        case ADD_T_IN:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != sizeof(uint64_t))
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            uint64_t amount = *(uint64_t *)cmd->data;
            return add_t_input_amount(amount);

        case ADD_T_OUT:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != sizeof(t_out_t))
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            return add_t_output((t_out_t *)cmd->data);

        case ADD_S_OUT:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != sizeof(s_out_t))
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            return add_s_output((s_out_t *)cmd->data);

        case SET_S_NET:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != sizeof(int64_t))
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            return set_sapling_net(*(int64_t *)cmd->data);

        case SET_T_MERKLE_PROOF:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != 3 * 32)
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            return set_t_merkle_proof((t_proofs_t *)cmd->data);

        case SET_S_MERKLE_PROOF:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != 3 * 32)
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            return set_s_merkle_proof((s_proofs_t *)cmd->data);

        case CHANGE_STAGE:
            if (cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }

            return change_stage(cmd->p1);

        case GET_PROOFGEN_KEY:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            return get_proofgen_key();

        case SIGN_SAPLING:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != 0)
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            return sign_sapling();

        case GET_SIGHASH:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }
            if (cmd->lc != 0)
                return io_send_sw(SW_WRONG_DATA_LENGTH);

            return get_sighash();

        case TEST_MATH:
            if (cmd->p1 != 0 || cmd->p2 != 0) {
                return io_send_sw(SW_WRONG_P1P2);
            }

            return handler_test_math();
        default:
            return io_send_sw(SW_INS_NOT_SUPPORTED);
    }
}
