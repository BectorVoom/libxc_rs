//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1144/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1144<F: Float>(t17863: F, t2986: F, t48279: F, t10231: F, t21409: F, t973: F, t21462: F, t2970: F, t10254: F, t21510: F, t21472: F, t13822: F, t21452: F, t21468: F, t42972: F, t21682: F, t225: F) -> (F, F, F, F, F, F, F, F) {
    let t69699 = t2986 * t48279 * t17863;
    let t69727 = t973 * t10231 * t21409;
    let t69739 = t973 * t2970 * t21462;
    let t69746 = t10254 * t21510;
    let t69796 = t973 * t2970 * t21472;
    let t69801 = t973 * t13822 * t21452;
    let t69806 = t973 * t42972 * t21468;
    let t69871 = t21682 * t225;
    (t69699, t69727, t69739, t69746, t69796, t69801, t69806, t69871)
}
