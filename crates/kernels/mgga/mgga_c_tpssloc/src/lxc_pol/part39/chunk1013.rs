//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1013/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1013<F: Float>(t13931: F, t340: F, t343: F, t974: F, t10263: F, t10287: F, t10290: F, t10331: F, t10333: F, t10339: F, t10342: F, t10353: F, t13896: F, t13907: F, t13909: F, t13915: F, t1600: F, t2960: F, t4543: F, t973: F) -> (F,) {
    let t13933 = t340 * t13931 * t343;
    let t13934 = t974 * t13933;
    let t13937 = -0.6172839506172839506e-4 * t13896 + 0.37037037037037037036e-3 * t10287 - 0.27777777777777777777e-3 * t10290 + 0.27160493827160493826e-2 * t10331 + 0.98765432098765432096e-3 * t10333 + t10339 + 0.14814814814814814814e-2 * t10342 - 0.27777777777777777777e-3 * t10353 - 0.81481481481481481481e-2 * t10263 * t1600 + t13907 + 0.18518518518518518518e-3 * t13909 + 0.44444444444444444444e-2 * t2960 * t4543 - t13915 - 0.83333333333333333332e-3 * t973 * t13934;
    (t13937,)
}
