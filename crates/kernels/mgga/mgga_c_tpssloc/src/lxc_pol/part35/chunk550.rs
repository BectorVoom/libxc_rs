//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 550/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk550<F: Float>(t1409: F, t2989: F, t2987: F, t344: F, t135: F, t1599: F, t973: F, t1597: F, t340: F, t974: F, t1604: F, t225: F) -> (F, F, F, F, F, F) {
    let t4514 = t2989 * t1409;
    let t4518 = t2987 * t344;
    let t4528 = t135 * t1599;
    let t4529 = t973 * t4528;
    let t4531 = t2987 * t1597;
    let t4546 = t974 * t340;
    let t4557 = t1604 * t225;
    (t4514, t4518, t4529, t4531, t4546, t4557)
}
