//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2266/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2266<F: Float>(t2314: F, t26003: F, t1874: F, t90381: F, t1983: F, t2019: F, t55169: F, t510: F, t652: F, t86604: F, t26114: F, t6535: F) -> (F, F, F, F, F) {
    let t91724 = F::new(4.0) * t2314 * t26003;
    let t91726 = F::new(2.0) * t90381 * t1874;
    let t91730 = t1983 * t2019 * t55169;
    let t91735 = F::new(2.0) * t652 * t510 * t86604;
    let t91737 = F::new(4.0) * t26114 * t6535;
    (t91724, t91726, t91730, t91735, t91737)
}
