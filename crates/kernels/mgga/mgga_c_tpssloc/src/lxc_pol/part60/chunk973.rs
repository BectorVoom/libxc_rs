//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 973/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk973<F: Float>(t118709: F, t118690: F, t1510: F, t22986: F, t6646: F, t1880: F, t1894: F, t214: F, t28406: F, t118727: F, t118738: F, t1888: F, t232: F, t98524: F) -> (F, F, F, F, F, F) {
    let t126442 = F::cast_from(0.16449340668482264365e-1_f64) * t118709;
    let t126446 = F::cast_from(0.6579736267392905746e-1_f64) * t22986 * t6646 * t118690 * t1510;
    let t126452 = F::cast_from(0.16449340668482264365e-1_f64) * t1880 * t214 * t1894 * t28406;
    let t126453 = F::cast_from(0.3289868133696452873e-1_f64) * t118727;
    let t126456 = F::cast_from(0.76763589786250567036e-1_f64) * t118738;
    let t126472 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t6646 * t98524 * t232;
    (t126442, t126446, t126452, t126453, t126456, t126472)
}
