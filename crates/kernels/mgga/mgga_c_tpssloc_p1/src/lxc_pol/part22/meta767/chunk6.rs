//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2598/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2598<F: Float>(t22298: F, t486: F, t11668: F, t11678: F, t11692: F, t15659: F, t1735: F, t18232: F, t19000: F, t19033: F, t3577: F, t3578: F, t45037: F, t45114: F, t45197: F, t4582: F, t4724: F, t4729: F, t4974: F, t4978: F, t4984: F, t6225: F, t6230: F, t65464: F, t65474: F, t65545: F, t65689: F, t65691: F, t72146: F) -> (F, F) {
    let t72445 = t486 * t22298;
    let t72452 = F::new(5.0) / F::new(2304.0) * t11678 * t11668 * t6225 * t4724 - F::new(5.0) / F::new(4608.0) * t11692 * t11668 * t6230 * t4724 - t45197 * t3578 * t65474 * t19000 / F::new(256.0) + t45114 * t3578 * t6225 * t19000 / F::new(256.0) - t11678 * t3578 * t6225 * t4729 / F::new(384.0) - t11678 * t3578 * t65464 * t19000 / F::new(768.0) - t11678 * t3578 * t15659 * t72146 / F::new(384.0) + F::new(5.0) / F::new(4608.0) * t3577 * t11668 * t1735 * t18232 + t65689 / F::new(3456.0) - F::new(11.0) / F::new(324.0) * t65691 - F::new(19.0) / F::new(576.0) * t65545 * t4984 + F::new(7.0) / F::new(1536.0) * t45037 * t4582 * t72445 * t4978 - F::new(19.0) / F::new(432.0) * t19033 * t4974;
    (t72445, t72452)
}
