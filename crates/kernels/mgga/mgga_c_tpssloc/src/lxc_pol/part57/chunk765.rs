//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 765/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk765<F: Float>(t1825: F, t6943: F, t6936: F, t1814: F, t8465: F, t8467: F, t5248: F, t5249: F, t550: F, t31170: F, t1831: F, t8466: F, t31137: F, t7691: F, t6888: F, t7700: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32714 = t6943 * t1825;
    let t32715 = t6936 * t32714;
    let t32717 = t1814 * t8465;
    let t32718 = t32717 * t8467;
    let t32721 = t5248 * t5249 * t550;
    let t32722 = t31170 * t32721;
    let t32724 = t8466 * t1831;
    let t32731 = t31137 * t7691;
    let t32733 = 0.3289868133696452873e-1 * t6888 * t32731;
    let t32735 = t31137 * t7700;
    (t32714, t32715, t32717, t32718, t32721, t32722, t32724, t32731, t32733, t32735)
}
