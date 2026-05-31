//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 705/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk705<F: Float>(t3: F, t7945: F, t1458: F, t2039: F, t1401: F, t3941: F, t5371: F, t577: F, t7230: F, t7801: F, t590: F, t60: F) -> (F, F, F, F) {
    let t7946 = t3 * t7945;
    let t7956 = t2039 * t1458;
    let t7961 = F::cast_from(0.45e1_f64) * t7945 * t577 + F::cast_from(0.135e2_f64) * t7230 * t1458 + F::cast_from(0.135e2_f64) * t5371 * t2039 + F::cast_from(27.0_f64) * t3941 * t7956 + F::cast_from(0.135e2_f64) * t1401 * t7801;
    let t8705 = F::cast_from(1.0_f64) / t60 / t590;
    (t7946, t7956, t7961, t8705)
}
