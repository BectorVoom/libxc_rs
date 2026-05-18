//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 896/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk896<F: Float>(t3576: F, t5064: F, t1725: F, t698: F, t1174: F, t5168: F, t588: F, t592: F, t2528: F, t5154: F, t2535: F, t118: F, t1787: F) -> (F, F, F, F, F, F, F, F) {
    let t15740 = t5064 * t3576;
    let t15753 = t698 * t1725;
    let t15754 = t1174 * t15753;
    let t15875 = t588 * t5168;
    let t15877 = t592 * t5168;
    let t15890 = t5154 * t2528;
    let t15895 = t5154 * t2535;
    let t15908 = t1787 * t118;
    (t15740, t15753, t15754, t15875, t15877, t15890, t15895, t15908)
}
