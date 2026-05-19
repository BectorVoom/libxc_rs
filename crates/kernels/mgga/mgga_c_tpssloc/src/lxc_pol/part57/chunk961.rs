//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 961/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk961<F: Float>(t118678: F, t1888: F, t232: F, t6646: F, t98541: F, t22996: F, t2632: F, t118709: F, t118690: F, t1510: F, t22986: F, t1880: F, t1894: F, t214: F, t28406: F) -> (F, F, F, F, F, F) {
    let t126433 = F::cast_from(0.76763589786250567036e-1_f64) * t118678;
    let t126437 = F::cast_from(0.16449340668482264365e-1_f64) * t1888 * t6646 * t98541 * t232;
    let t126441 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t22996 * t98541 * t2632;
    let t126442 = F::cast_from(0.16449340668482264365e-1_f64) * t118709;
    let t126446 = F::cast_from(0.6579736267392905746e-1_f64) * t22986 * t6646 * t118690 * t1510;
    let t126452 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t214 * t1894 * t28406;
    (t126433, t126437, t126441, t126442, t126446, t126452)
}
