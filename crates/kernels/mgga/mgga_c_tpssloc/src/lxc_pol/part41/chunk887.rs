//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 887/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk887<F: Float>(t12344: F, t1336: F, t1369: F, t241: F, t67: F, t6924: F, t1339: F, t2690: F, t1354: F, t1307: F, t3792: F, t3788: F, t835: F, t1995: F, t246: F, t3777: F, t3802: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12345 = t1336 * t12344;
    let t12346 = t12345 * t1369;
    let t12351 = t241 * t6924 * t67;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    let t12366 = t12365 * t1354;
    let t12369 = t3792 * t1307;
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12429 = t3777 * t3802;
    (t12345, t12346, t12351, t12365, t12366, t12369, t12385, t12418, t12419, t12429)
}
