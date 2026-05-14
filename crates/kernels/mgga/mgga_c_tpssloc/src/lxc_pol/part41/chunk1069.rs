//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1069/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1069<F: Float>(t1136: F, t6037: F, t1683: F, t4819: F, t6056: F, t6053: F, t3359: F, t6052: F, t4823: F, t11352: F, t6036: F, t11137: F, t11444: F, t14702: F, t14720: F, t15194: F, t15195: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t18239: F, t18243: F) -> (F, F, F, F, F, F, F, F) {
    let t18631 = t6037 * t1136;
    let t18634 = t1683 * t4819;
    let t18637 = t6056 * t1136;
    let t18640 = t6053 * t1136;
    let t18643 = t6052 * t3359;
    let t18644 = t18643 * t1136;
    let t18647 = t4823 * t4819;
    let t18650 = t6036 * t11352;
    let t18651 = t18650 * t1136;
    let t18668 = -t11444 + 0.76103703703703703703e-2 * t11137 + 0.1522074074074074074e-1 * t14702 + 0.761037037037037037e-2 * t14720 - t15194 - t15195 + 0.3805185185185185185e-2 * t18203 + 0.19025925925925925925e-1 * t18208 - 0.68493333333333333331e-1 * t18213 - 0.2283111111111111111e-1 * t18217 - 0.11415555555555555555e-1 * t18219 + 0.10274e0 * t18223 + 0.68493333333333333332e-1 * t18227 - 0.57077777777777777777e-2 * t18229 - 0.11415555555555555555e-1 * t18234 + 0.34246666666666666666e-1 * t18239 + 0.17123333333333333333e-1 * t18243;
    (t18631, t18634, t18637, t18640, t18644, t18647, t18651, t18668)
}
