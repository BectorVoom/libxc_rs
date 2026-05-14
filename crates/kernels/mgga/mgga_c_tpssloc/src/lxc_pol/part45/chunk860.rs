//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 860/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk860<F: Float>(t114552: F, t2040: F, t2314: F, t31744: F, t4034: F, t652: F, t6534: F, t7156: F, t12823: F, t8533: F, t31772: F, t112521: F, t112523: F, t114541: F, t114543: F, t12734: F, t22461: F, t23918: F, t23929: F, t26103: F, t31726: F, t6517: F, t6862: F, t7056: F, t7057: F, t8529: F) -> (F,) {
    let t114554 = 2.0 * t114552 * t2040;
    let t114559 = 4.0 * t2314 * t31744;
    let t114561 = 4.0 * t4034 * t31744;
    let t114564 = 4.0 * t652 * t7156 * t6534;
    let t114566 = 2.0 * t12823 * t8533;
    let t114568 = 4.0 * t4034 * t31772;
    let t114569 = -4.0 * t652 * t6862 * t7056 - 4.0 * t12734 * t8529 - 2.0 * t12823 * t8529 - 4.0 * t22461 * t7057 - 2.0 * t23918 * t6517 - 4.0 * t23929 * t6517 - 4.0 * t26103 * t7057 - 4.0 * t31726 * t4034 - t112521 - t112523 - t114541 - t114543 - t114554 - t114559 - t114561 - t114564 - t114566 - t114568;
    (t114569,)
}
