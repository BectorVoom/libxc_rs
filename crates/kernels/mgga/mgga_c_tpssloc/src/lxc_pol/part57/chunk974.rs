//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 974/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk974<F: Float>(t1992: F, t550: F, t6976: F, t97181: F, t120437: F, t1825: F, t22633: F, t120514: F, t120521: F, t97172: F, t22897: F, t3792: F) -> (F, F, F, F, F, F) {
    let t127391 = F::cast_from(0.16449340668482264365e-1_f64) * t1992 * t6976 * t97181 * t550;
    let t127402 = F::cast_from(0.6579736267392905746e-1_f64) * t22633 * t6976 * t120437 * t1825;
    let t127403 = F::cast_from(0.76763589786250567036e-1_f64) * t120514;
    let t127404 = F::cast_from(0.16449340668482264365e-1_f64) * t120521;
    let t127408 = F::cast_from(0.16449340668482264365e-1_f64) * t1992 * t6976 * t97172 * t550;
    let t127412 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t22897 * t97172 * t3792;
    (t127391, t127402, t127403, t127404, t127408, t127412)
}
