//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 998/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk998<F: Float>(t12156: F, t12351: F, t820: F, t3866: F, t3872: F, t3876: F, t12012: F, t1367: F, t1339: F, t2690: F, t1336: F, t1354: F, t120: F, t3791: F) -> (F, F, F, F, F, F, F, F) {
    let t12353 = t12351 * t820 * t12156;
    let t12356 = t3866 * t3872;
    let t12358 = t3866 * t3876;
    let t12361 = t1367 * t820 * t12012;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    let t12366 = t12365 * t1354;
    let t12368 = t120 * t3791;
    (t12353, t12356, t12358, t12361, t12364, t12365, t12366, t12368)
}
