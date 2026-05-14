//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 925/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk925<F: Float>(t114569: F, t115222: F, t115267: F, t115685: F, t115719: F, t115758: F, t115934: F, t115969: F, t7015: F, t84033: F, t12524: F, t31817: F, t114456: F, t114513: F, t114515: F, t114517: F, t114520: F, t114525: F, t114527: F, t114529: F, t114531: F, t2363: F, t23880: F, t23917: F, t24478: F, t31795: F, t577: F, t7010: F, t7235: F, t83980: F, t8508: F) -> (F, F) {
    let t115972 = t114569 + t115222 + t115267 + t115685 + t115719 + t115758 + t115934 + t115969;
    let t115978 = 54.0 * t84033 * t7015;
    let t115980 = 54.0 * t12524 * t31817;
    let t115981 = 0.135e2 * t7010 * t23917 + t114513 + t114515 + t114517 + t114520 + t114456 + 54.0 * t23880 * t24478 + t8508 + t114525 + t114527 + t114529 + t114531 + 0.135e2 * t31795 * t2363 + 0.45e1 * t115972 * t577 + 54.0 * t83980 * t7235 + t115978 + t115980;
    (t115972, t115981)
}
