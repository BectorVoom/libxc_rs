//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 750/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk750<F: Float>(t112: F, t7945: F, t2109: F, t26012: F, t33: F, t7973: F, t2240: F, t12571: F, t7245: F, t1419: F, t55: F, t1240: F, t1760: F) -> (F, F, F, F, F, F, F) {
    let t27254 = t7945 * t112;
    let t27298 = t2109 * t26012;
    let t27331 = t33 * t7973;
    let t27332 = t2240 * t27331;
    let t27341 = t12571 * t7245;
    let t27356 = t1419 * t55;
    let t27381 = t1240 * t1760;
    (t27254, t27298, t27331, t27332, t27341, t27356, t27381)
}
