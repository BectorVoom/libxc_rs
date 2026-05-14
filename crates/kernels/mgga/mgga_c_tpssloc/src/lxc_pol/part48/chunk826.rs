//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 826/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk826<F: Float>(t22716: F, t8480: F, t31203: F, t6914: F, t2006: F, t3791: F, t1992: F, t550: F, t6976: F, t22897: F, t3792: F, t31207: F, t6883: F, t22724: F, t31198: F, t22704: F, t22705: F, t31202: F) -> (F, F, F, F, F, F, F) {
    let t114104 = 0.12793931631041761173e0 * t22716 * t8480;
    let t114105 = t6914 * t31203;
    let t114106 = 0.76763589786250567036e-1 * t114105;
    let t114107 = t2006 * t3791;
    let t114111 = 0.16449340668482264365e-1 * t1992 * t6976 * t114107 * t550;
    let t114115 = 0.3289868133696452873e-1 * t1992 * t22897 * t114107 * t3792;
    let t114116 = t6883 * t31207;
    let t114117 = 0.76763589786250567036e-1 * t114116;
    let t114119 = 0.52089578783527170489e-1 * t22724 * t31198;
    let t114121 = t22704 * t22705 * t31202;
    (t114104, t114106, t114111, t114115, t114117, t114119, t114121)
}
