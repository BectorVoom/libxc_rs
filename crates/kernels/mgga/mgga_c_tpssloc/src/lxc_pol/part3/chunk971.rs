//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 971/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk971<F: Float>(t12344: F, t1336: F, t1369: F, t241: F, t67: F, t6924: F, t3866: F, t3872: F, t3876: F, t1339: F, t2690: F, t1354: F) -> (F, F, F, F, F, F, F) {
    let t12345 = t1336 * t12344;
    let t12346 = t12345 * t1369;
    let t12351 = t241 * t6924 * t67;
    let t12356 = t3866 * t3872;
    let t12358 = t3866 * t3876;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    let t12366 = t12365 * t1354;
    (t12345, t12346, t12351, t12356, t12358, t12365, t12366)
}
