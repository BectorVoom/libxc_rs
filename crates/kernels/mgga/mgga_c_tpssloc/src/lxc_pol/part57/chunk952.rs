//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 952/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk952<F: Float>(t1530: F, t7540: F, t25373: F, t118480: F, t22986: F, t32814: F, t86873: F, t118472: F, t1484: F, t23270: F, t112899: F, t28267: F) -> (F, F, F, F, F, F) {
    let t126197 = t7540 * t1530;
    let t126198 = t25373 * t126197;
    let t126226 = F::new(0.15352717957250113407e0) * t118480;
    let t126229 = F::new(0.6579736267392905746e-1) * t22986 * t86873 * t32814;
    let t126233 = F::new(0.6579736267392905746e-1) * t22986 * t23270 * t118472 * t1484;
    let t126240 = F::new(0.6579736267392905746e-1) * t22986 * t112899 * t28267;
    (t126197, t126198, t126226, t126229, t126233, t126240)
}
