//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 671/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk671<F: Float>(t3701: F, t6463: F, t562: F, t6414: F, t1824: F, t1834: F, t6387: F, t120: F, t225: F, t6364: F, t6435: F, t6362: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19596 = t6463 * t3701;
    let t19660 = t562 * t6414;
    let t19739 = t1834 * t1824;
    let t19743 = t562 * t6387;
    let t19871 = t120 * t6387;
    let t19956 = t120 * t6414;
    let t20029 = t6364 * t225;
    let t20044 = t6435 * t225;
    let t20060 = t6362 * t225;
    (t19596, t19660, t19739, t19743, t19871, t19956, t20029, t20044, t20060)
}
