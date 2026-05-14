//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1375/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1375<F: Float>(t12291: F, t1341: F, t1343: F, t16285: F, t1827: F, t19855: F, t20492: F, t20497: F, t20556: F, t20570: F, t3790: F, t40449: F, t5235: F, t54020: F, t54793: F, t6417: F, t6422: F, t74290: F, t80076: F, t80085: F, t80189: F, t80193: F, t820: F) -> (F,) {
    let t80474 = -t1341 * t1343 * t820 * t80193 / 3072.0 - t74290 * t1827 / 768.0 - t19855 * t6417 / 512.0 - t5235 * t20556 / 768.0 + t16285 * t20497 / 128.0 - t19855 * t6422 / 512.0 - 3.0 / 256.0 * t12291 * t1343 * t820 * t80189 - t5235 * t20570 / 768.0 - t54020 * t20492 / 128.0 - t1341 * t1343 * t820 * t80076 / 1024.0 - 595.0 / 2592.0 * t54793 + t40449 + t3790 * t1343 * t820 * t80085 / 512.0;
    (t80474,)
}
