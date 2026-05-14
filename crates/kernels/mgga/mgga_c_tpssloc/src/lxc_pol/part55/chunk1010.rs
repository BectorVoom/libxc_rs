//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1010/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1010<F: Float>(t34277: F, t466: F, t1653: F, t32457: F, t7362: F, t1716: F, t8891: F, t7376: F, t8082: F, t7375: F, t2147: F, t8054: F, t462: F, t1734: F, t8882: F, t1246: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34278 = t466 * t34277;
    let t34284 = t32457 * t1653;
    let t34285 = t7362 * t34284;
    let t34288 = t1716 * t8891;
    let t34291 = t8082 * t7376;
    let t34292 = t7375 * t34291;
    let t34295 = t2147 * t8054;
    let t34296 = t462 * t34295;
    let t34300 = t8882 * t1734;
    let t34301 = t34300 * t1246;
    (t34278, t34284, t34285, t34288, t34291, t34292, t34295, t34296, t34300, t34301)
}
