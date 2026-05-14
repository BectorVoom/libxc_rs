//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 922/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk922<F: Float>(t15402: F, t4729: F, t3447: F, t134: F, t3439: F, t461: F, t4724: F, t15026: F, t3032: F, t3514: F, t3572: F, t5002: F, t3523: F, t5005: F, t5019: F, t5024: F) -> (F, F, F, F, F, F, F, F) {
    let t15403 = t15402 * t4729;
    let t15405 = 0.37037037037037037036e-3 * t3447 * t15403;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15420 = t15419 * t4724;
    let t15422 = 0.24691358024691358024e-3 * t3447 * t15420;
    let t15437 = t15026 * t3032;
    let t15438 = t15437 * t3514;
    let t15446 = t5002 * t3572 / 2304.0;
    let t15448 = t5005 * t3523 / 3456.0;
    let t15450 = t5019 * t3572 / 432.0;
    let t15452 = t5024 * t3523 / 648.0;
    (t15405, t15422, t15437, t15438, t15446, t15448, t15450, t15452)
}
