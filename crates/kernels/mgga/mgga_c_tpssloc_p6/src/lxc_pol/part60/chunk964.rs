//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 964/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk964<F: Float>(t118472: F, t1484: F, t22986: F, t23270: F, t112899: F, t28267: F, t118821: F, t1527: F, t1888: F, t1880: F, t28263: F, t30663: F) -> (F, F, F, F) {
    let t126233 = F::cast_from(0.6579736267392905746e-1_f64) * t22986 * t23270 * t118472 * t1484;
    let t126240 = F::cast_from(0.6579736267392905746e-1_f64) * t22986 * t112899 * t28267;
    let t126246 = F::cast_from(0.6579736267392905746e-1_f64) * t1888 * t23270 * t118821 * t1527;
    let t126249 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t30663 * t28263;
    (t126233, t126240, t126246, t126249)
}
