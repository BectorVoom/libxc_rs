//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1126/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1126<F: Float>(t135: F, t23631: F, t6688: F, t23509: F, t25651: F, t1016: F, t3034: F, t1930: F, t6741: F, t10469: F, t10474: F, t363: F, t10401: F, t23417: F, t3186: F, t10383: F, t1926: F) -> (F, F, F, F, F, F, F) {
    let t82822 = t23631 * t135 * t6688;
    let t82895 = t23509 * t25651;
    let t82985 = 1.0 / t3034 / t1016;
    let t82986 = t1930 * t82985;
    let t82987 = t82986 * t6741;
    let t82989 = t10469 * t10474 * t363;
    let t83015 = t23417 * t10401;
    let t83016 = t3186 * t83015;
    let t83028 = 5.0 / 1296.0 * t1926 * t10383;
    (t82822, t82895, t82987, t82989, t83015, t83016, t83028)
}
