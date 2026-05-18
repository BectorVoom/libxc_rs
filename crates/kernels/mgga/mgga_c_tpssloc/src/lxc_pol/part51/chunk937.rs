//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 937/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk937<F: Float>(t1878: F, t244: F, t2230: F, t6589: F, t213: F, t6593: F, t229: F, t6546: F, t805: F, t243: F, t598: F, t6584: F, t6604: F) -> (F, F, F, F, F, F, F, F) {
    let t23056 = t1878 * t244;
    let t23061 = t2230 * t6589;
    let t23062 = t23061 * t213;
    let t23063 = t23062 * t6593;
    let t23069 = t6546 * t229;
    let t23070 = t23069 * t805;
    let t23071 = F::new(7.0) / F::new(72.0) * t23070;
    let t23075 = t243 * t243;
    let t23076 = F::new(1.0) / t23075;
    let t23077 = t598 * t23076;
    let t23083 = t6584 * t6604;
    (t23056, t23062, t23063, t23069, t23070, t23071, t23077, t23083)
}
