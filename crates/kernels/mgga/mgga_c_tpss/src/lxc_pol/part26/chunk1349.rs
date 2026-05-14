//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1349/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1349<F: Float>(t13133: F, t1322: F, t13554: F, t20944: F, t20969: F, t22108: F, t3491: F, t3493: F, t624: F, t6486: F, t6540: F, t69427: F, t69437: F, t69439: F, t69441: F, t69444: F, t69768: F, t69770: F, t69773: F, t69775: F, t69776: F, t69779: F, t69782: F, t69784: F) -> (F,) {
    let t73130 = -4.0 * t13133 * t6486 - 2.0 * t1322 * t20944 - 4.0 * t13554 * t6486 - 4.0 * t20969 * t3493 - t22108 * t624 - 2.0 * t3491 * t6540 + t69427 - t69437 - t69439 - t69441 - t69444 + t69768 - t69770 + t69773 + t69775 - t69776 + t69779 - t69782 - t69784;
    (t73130,)
}
