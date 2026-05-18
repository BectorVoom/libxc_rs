//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 502/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk502<F: Float>(t1834: F, t539: F, t1380: F, t1825: F, t553: F, t1336: F, t1814: F, t544: F, t564: F, t1378: F, t1375: F, t1808: F, t568: F) -> (F, F, F, F, F, F) {
    let t1835 = t539 * t1834;
    let t1838 = t1380 * t1825;
    let t1840 = t553 * t1834;
    let t1842 = -t1336 * t1838 + t1814 * t564 + t1840 * t544;
    let t1843 = t1378 * t1842;
    let t1845 = -t1375 * t1843 + t1808 * t568 + t1835 * t568;
    (t1835, t1838, t1840, t1842, t1843, t1845)
}
