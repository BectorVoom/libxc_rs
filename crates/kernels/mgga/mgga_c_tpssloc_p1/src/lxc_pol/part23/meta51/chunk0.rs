//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 326/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk326<F: Float>(t500: F, t111: F, t88: F, t522: F, t588: F, t592: F, t521: F, t750: F) -> (F, F, F, F, F) {
    let t1256 = F::new(1.0) / t500;
    let t1268 = t88 * t111;
    let t1274 = F::new(4.0) * t588 * t522;
    let t1276 = F::new(4.0) * t592 * t522;
    let t1287 = t521 * t750;
    (t1256, t1268, t1274, t1276, t1287)
}
