//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1072/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1072<F: Float>(t11424: F, t5989: F, t3259: F, t6021: F, t11136: F, t11137: F, t14702: F, t14922: F, t14923: F, t14924: F, t18203: F, t18208: F, t18213: F, t18217: F, t18219: F, t18223: F, t18227: F, t18229: F, t18234: F, t18239: F, t18243: F) -> (F, F, F) {
    let t18694 = 2.0 * t11424 * t5989;
    let t18696 = 1.0 * t3259 * t6021;
    let t18710 = -t11136 + 0.41203703703703703703e-2 * t11137 + 0.82407407407407407408e-2 * t14702 + t14922 - t14923 - t14924 + 0.20601851851851851852e-2 * t18203 + 0.10300925925925925926e-1 * t18208 - 0.37083333333333333333e-1 * t18213 - 0.12361111111111111111e-1 * t18217 - 0.61805555555555555557e-2 * t18219 + 0.55625000000000000001e-1 * t18223 + 0.37083333333333333334e-1 * t18227 - 0.30902777777777777778e-2 * t18229 - 0.61805555555555555555e-2 * t18234 + 0.18541666666666666667e-1 * t18239 + 0.92708333333333333333e-2 * t18243;
    (t18694, t18696, t18710)
}
