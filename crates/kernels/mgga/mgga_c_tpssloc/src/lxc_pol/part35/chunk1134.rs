//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1134/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1134<F: Float>(t3701: F, t7752: F, t1458: F, t576: F, t2113: F, t22811: F, t85: F, t24: F, t12019: F, t566: F, t68: F, t3700: F, t2751: F, t10108: F, t257: F, t3639: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33136 = t3701 * t7752;
    let t33185 = t576 * t1458;
    let t33690 = t2113 * t1458;
    let t39041 = 1.0 / t22811;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = 1.0 / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0 / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = 1.0 / t40771;
    let t40889 = 1.0 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t43705 = t3639 * t3639;
    (t33136, t33185, t33690, t39041, t39063, t40591, t40611, t40772, t40890, t43705)
}
