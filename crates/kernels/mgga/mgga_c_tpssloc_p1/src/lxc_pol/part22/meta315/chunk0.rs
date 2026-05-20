//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1494/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1494<F: Float>(t15419: F, t4724: F, t3447: F, t15026: F, t3032: F, t3514: F) -> (F, F, F, F) {
    let t15420 = t15419 * t4724;
    let t15422 = F::cast_from(0.24691358024691358024e-3_f64) * t3447 * t15420;
    let t15437 = t15026 * t3032;
    let t15438 = t15437 * t3514;
    (t15420, t15422, t15437, t15438)
}
