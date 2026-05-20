//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2295/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2295<F: Float>(t1244: F, t3068: F, t478: F, t6163: F, t11697: F, t18386: F, t3577: F, t15608: F, t15740: F, t1174: F, t6183: F, t698: F) -> (F, F, F, F) {
    let t66622 = t1244 * t478 * t6163 * t3068;
    let t66646 = t3577 * t11697 * t18386;
    let t66648 = t15740 * t15608;
    let t66668 = t1174 * t698 * t6183;
    (t66622, t66646, t66648, t66668)
}
