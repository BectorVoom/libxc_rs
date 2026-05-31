//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1008/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1008<F: Float>(t31682: F, t5398: F, t8308: F, t113875: F, t121022: F, t1433: F, t126103: F, t1862: F, t8513: F, t115860: F, t115895: F, t121029: F, t121058: F, t121064: F, t121066: F, t126070: F, t126100: F, t31681: F, t33115: F, t33560: F, t33568: F, t55921: F, t8511: F, t8512: F, t8515: F) -> F {
    let t128311 = t8308 * t31682 * t5398;
    let t128317 = t113875 * t121022 * t1433;
    let t128326 = t8513 * t126103 * t1862;
    let t128333 = -F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t121029 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t31681 * t126070 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31681 * t128311 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t121058 * t33568 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t115895 * t128317 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t55921 * t8511 * t8515 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t33560 * t33115 - t115860 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8512 * t128326 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8512 * t126100 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t121064 + F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t121066;
    t128333
}
