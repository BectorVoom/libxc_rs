//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1292/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1292<F: Float>(t115390: F, t22751: F, t31620: F, t552: F, t7191: F, t22892: F, t22893: F, t31619: F, t31628: F, t6914: F, t22704: F, t22705: F, t31627: F) -> (F, F, F, F, F, F) {
    let t115391 = F::cast_from(0.82246703342411321824e-2_f64) * t115390;
    let t115397 = t22751 * t31620;
    let t115399 = t552 * t7191;
    let t115409 = t22892 * t22893 * t31619;
    let t115415 = t6914 * t31628;
    let t115423 = t22704 * t22705 * t31627;
    (t115391, t115397, t115399, t115409, t115415, t115423)
}
