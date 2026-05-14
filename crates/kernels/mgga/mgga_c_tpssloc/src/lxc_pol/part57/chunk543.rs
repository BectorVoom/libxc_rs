//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 543/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk543<F: Float>(t387: F, t7560: F, t345: F, t1634: F, t6705: F, t6704: F, t1603: F, t1945: F, t1409: F, t3: F, t1933: F, t1597: F, t343: F, t6734: F, t1615: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7561 = t7560 * t387;
    let t7562 = t345 * t7561;
    let t7565 = t6705 * t1634;
    let t7566 = t6704 * t7565;
    let t7569 = t1603 * t1945;
    let t7573 = t3 * t1409;
    let t7574 = t1933 * t7573;
    let t7577 = t1597 * t343;
    let t7578 = t7577 * t6734;
    let t7581 = t1615 * t68;
    (t7561, t7562, t7565, t7566, t7569, t7573, t7574, t7577, t7578, t7581)
}
