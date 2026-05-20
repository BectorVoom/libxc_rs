//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2608/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2608<F: Float>(t1734: F, t6218: F, t1089: F, t11678: F, t11692: F, t1215: F, t1227: F, t15569: F, t15659: F, t15700: F, t15701: F, t15702: F, t1735: F, t18237: F, t18321: F, t18368: F, t18395: F, t18397: F, t18401: F, t3577: F, t3578: F, t4582: F, t4729: F, t4972: F, t5046: F, t52879: F, t52903: F, t5398: F, t5979: F, t607: F, t6219: F, t65464: F, t65469: F, t66120: F, t70458: F) -> (F, F) {
    let t72767 = t6218 * t1734;
    let t72783 = -t52903 * t18397 / F::new(144.0) + t11692 * t3578 * t65469 * t18395 / F::new(1536.0) - t11678 * t3578 * t15659 * t5979 * t1215 / F::new(768.0) + t11692 * t3578 * t15700 * t15701 * t5398 / F::new(1536.0) - t3577 * t3578 * t1735 * t18237 / F::new(768.0) - t3577 * t3578 * t6219 * t4729 / F::new(768.0) - t11678 * t3578 * t65464 * t1734 * t1089 * t607 / F::new(768.0) + t11692 * t3578 * t72767 * t15702 / F::new(1536.0) + t15569 * t18401 / F::new(72.0) - t66120 / F::new(72.0) - F::new(11.0) / F::new(108.0) * t18321 * t5046 - t52879 * t18368 / F::new(768.0) - t1227 * t4582 * t4972 * t70458 / F::new(2304.0);
    (t72767, t72783)
}
