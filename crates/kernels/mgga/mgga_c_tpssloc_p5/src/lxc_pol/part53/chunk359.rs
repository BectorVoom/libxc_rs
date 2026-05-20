//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 359/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk359<F: Float>(t1834: F, t539: F, t1380: F, t1825: F, t553: F, t1336: F, t1814: F, t544: F, t564: F) -> (F, F, F, F) {
    let t1835 = t539 * t1834;
    let t1838 = t1380 * t1825;
    let t1840 = t553 * t1834;
    let t1842 = -t1336 * t1838 + t1814 * t564 + t1840 * t544;
    (t1835, t1838, t1840, t1842)
}
