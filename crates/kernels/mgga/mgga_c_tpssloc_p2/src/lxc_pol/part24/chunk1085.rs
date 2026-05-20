//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1085/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1085<F: Float>(t3698: F, t3701: F, t12125: F, t12128: F, t12131: F, t12133: F, t12135: F, t12137: F, t12139: F, t12141: F, t12143: F, t1307: F, t3719: F, t3734: F, t3914: F, t3918: F, t3919: F, t5126: F, t5160: F, t6999: F, t9853: F, t9859: F) -> F {
    let t12477 = t3698 * t3701;
    let t12490 = -F::new(9.0) * t12477 * t1307 * t3918 + F::new(9.0) * t3719 * t3918 * t3919 + F::new(18.0) * t3734 * t3919 * t5126 - F::new(3.0) * t3914 * t5160 * t6999 + t12125 + t12128 + t12131 + t12133 - t12135 + t12137 + t12139 - t12141 - t12143 + t9853 + t9859;
    t12490
}
