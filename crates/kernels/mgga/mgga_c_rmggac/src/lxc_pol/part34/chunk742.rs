//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 742/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk742<F: Float>(t3352: F, t69004: F, t8416: F, t3351: F, t515: F, t8982: F, t9188: F, t2144: F, t8985: F, t875: F, t8976: F, t1986: F, t2396: F, t7720: F, t2046: F, t2050: F, t2406: F, t31: F) -> (F, F, F, F, F, F) {
    let t75465 = t69004 * t3352 * t8416;
    let t75469 = t3351 * t9188 * t515 * t8982;
    let t75473 = t3351 * t3352 * t2144 * t8985;
    let t75477 = t3351 * t3352 * t875 * t8976;
    let t75479 = t1986 * t2396;
    let t75480 = t7720 * t75479;
    let t75484 = t2046 * t2050 * t2406 * t31;
    (t75465, t75469, t75473, t75477, t75480, t75484)
}
