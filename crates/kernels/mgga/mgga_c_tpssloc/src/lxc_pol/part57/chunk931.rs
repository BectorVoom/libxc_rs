//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 931/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk931<F: Float>(t31169: F, t5234: F, t114011: F, t32721: F, t1824: F, t22705: F, t22852: F, t550: F, t59: F, t1831: F, t31176: F, t22804: F, t32711: F) -> (F, F, F, F, F) {
    let t120341 = t5234 * t31169;
    let t120350 = t114011 * t32721;
    let t120363 = t22852 * t22705 * t59 * t1824 * t550;
    let t120375 = t31176 * t1831;
    let t120383 = t22804 * t32711;
    (t120341, t120350, t120363, t120375, t120383)
}
