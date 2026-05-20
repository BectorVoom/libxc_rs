//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1888/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1888<F: Float>(t1603: F, t3166: F, t13939: F, t381: F, t1049: F, t4552: F, t1052: F, t1066: F, t13736: F, t13743: F, t14527: F, t14529: F, t14532: F, t3026: F, t3169: F, t3207: F, t388: F, t4660: F, t4665: F, t4694: F) -> (F, F, F, F) {
    let t14534 = t1603 * t3166;
    let t14536 = t13939 * t381;
    let t14538 = t4552 * t1049;
    let t14543 = -F::new(6.0) * t1052 * t13736 + F::new(4.0) * t1052 * t13743 - F::new(2.0) * t1066 * t14529 + t14527 * t388 + t14532 * t388 + t14534 * t388 + t14536 * t388 + F::new(2.0) * t14538 * t388 + F::new(4.0) * t3026 * t4665 - F::new(2.0) * t3026 * t4694 - F::new(2.0) * t3169 * t4694 - t3207 * t4660;
    (t14534, t14536, t14538, t14543)
}
