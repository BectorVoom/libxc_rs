//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 886/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk886<F: Float>(t126103: F, t1862: F, t8513: F, t115860: F, t115895: F, t121029: F, t121058: F, t121064: F, t121066: F, t126070: F, t126100: F, t128311: F, t128317: F, t31681: F, t33115: F, t33560: F, t33568: F, t55921: F, t8511: F, t8512: F, t8515: F) -> (F,) {
    let t128326 = t8513 * t126103 * t1862;
    let t128333 = -40.0 / 27.0 * t121029 + 5.0 / 9.0 * t31681 * t126070 + 5.0 / 18.0 * t31681 * t128311 + 5.0 / 9.0 * t121058 * t33568 + 5.0 / 3.0 * t115895 * t128317 - 5.0 / 72.0 * t55921 * t8511 * t8515 - 5.0 / 36.0 * t33560 * t33115 - t115860 - 5.0 / 36.0 * t8512 * t128326 - 5.0 / 72.0 * t8512 * t126100 - 20.0 / 9.0 * t121064 + 20.0 / 27.0 * t121066;
    (t128333,)
}
