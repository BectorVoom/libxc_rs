//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 815/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk815<F: Float>(t30686: F, t6579: F, t1902: F, t2631: F, t1888: F, t22996: F, t2632: F, t23110: F, t23185: F, t30685: F, t1880: F, t1894: F, t214: F, t23150: F, t23012: F, t8357: F) -> (F, F, F, F, F, F) {
    let t112974 = t6579 * t30686;
    let t112975 = 0.76763589786250567036e-1 * t112974;
    let t112976 = t1902 * t2631;
    let t112980 = 0.3289868133696452873e-1 * t1888 * t22996 * t112976 * t2632;
    let t112983 = t23185 * t23110 * t30685;
    let t112984 = 0.16449340668482264365e-1 * t112983;
    let t112988 = 0.16449340668482264365e-1 * t1880 * t214 * t1894 * t23150;
    let t112990 = 0.12793931631041761173e0 * t23012 * t8357;
    (t112975, t112976, t112980, t112984, t112988, t112990)
}
