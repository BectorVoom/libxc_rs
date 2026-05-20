//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1269/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1269<F: Float>(t3866: F, t3872: F, t3876: F, t1339: F, t2690: F, t1336: F, t1354: F, t1307: F, t3792: F, t3788: F, t835: F, t3795: F) -> (F, F, F, F, F, F) {
    let t12356 = t3866 * t3872;
    let t12358 = t3866 * t3876;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    let t12366 = t12365 * t1354;
    let t12369 = t3792 * t1307;
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12386 = t12385 * t3795;
    (t12356, t12358, t12365, t12366, t12369, t12386)
}
