//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1039/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1039<F: Float>(t25446: F, t25762: F, t25794: F, t25834: F, t3216: F, t7627: F, t1068: F, t1637: F, t1484: F, t1530: F, t16596: F, t1877: F, t1915: F, t193: F, t202: F, t23290: F, t23295: F, t2522: F, t25353: F, t25358: F, t25365: F, t25374: F, t4119: F, t4255: F, t4303: F, t4314: F, t6666: F, t6670: F, t7541: F, t776: F, t868: F, t870: F) -> (F, F, F, F) {
    let t25836 = t25446 + t25762 + t25794 + t25834;
    let t25840 = t7627 * t3216;
    let t25845 = t1637 * t1068;
    let t25882 = t193 * t202 * t25353 * t870 + F::new(3.0) * t1484 * t2522 * t6666 - t1530 * t1877 * t23290 - F::new(3.0) * t16596 * t2522 * t6670 + F::new(2.0) * t1877 * t23295 * t25374 - t1877 * t25358 * t868 - t1877 * t4303 * t6670 + F::new(3.0) * t1915 * t2522 * t4119 + F::new(6.0) * t1915 * t4255 * t4314 - F::new(3.0) * t2522 * t25365 * t6670 + F::new(3.0) * t2522 * t7541 * t776;
    (t25836, t25840, t25845, t25882)
}
