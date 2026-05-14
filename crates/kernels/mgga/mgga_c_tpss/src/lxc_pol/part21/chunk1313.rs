//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1313/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1313<F: Float>(t1497: F, t2433: F, t31814: F, t33: F, t64248: F, t20047: F, t63844: F, t1006: F, t3610: F, t2133: F, t1692: F, t1713: F, t18052: F, t18239: F, t18265: F, t19670: F, t19798: F, t19816: F, t20021: F, t20025: F, t20041: F, t2439: F, t2829: F, t3552: F, t5586: F, t5671: F, t61269: F, t6149: F, t63782: F, t64305: F, t64870: F) -> (F,) {
    let t64876 = t1497 * t2433;
    let t64879 = t31814 * t33;
    let t64880 = t64879 * t64248;
    let t64888 = t20047 * t63844;
    let t64896 = t1006 * t3610;
    let t64905 = t1497 * t2133;
    let t64909 = 3.0 * t19670 * t64870 + 3.0 * t2439 * t5586 * t20025 + t1692 * t18052 * t64876 - 3.0 * t19816 * t64880 + t1692 * t64305 * t18265 + 3.0 * t3552 * t6149 * t18239 + t19816 * t64888 + 3.0 * t2439 * t5586 * t20021 + 3.0 * t2439 * t19798 * t5671 + 3.0 * t2439 * t1713 * t64896 + t1692 * t6149 * t2829 / 2.0 - 3.0 * t61269 * t20041 + 3.0 / 2.0 * t2439 * t1713 * t64905 - t63782;
    (t64909,)
}
