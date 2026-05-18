//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1026/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1026<F: Float>(t11677: F, t3624: F, t1090: F, t3516: F, t3578: F, t3521: F, t820: F, t3579: F, t3577: F, t248: F, t3494: F, t3570: F) -> (F, F, F, F) {
    let t11692 = t3624 * t11677;
    let t11693 = t3516 * t1090;
    let t11694 = t3578 * t11693;
    let t11697 = t820 * t3521;
    let t11698 = t11697 * t3579;
    let t11699 = t3577 * t11698;
    let t11702 = t248 * t3570 * t3494;
    (t11692, t11694, t11699, t11702)
}
