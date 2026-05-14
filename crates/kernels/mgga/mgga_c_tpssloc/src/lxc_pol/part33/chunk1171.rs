//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1171/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1171<F: Float>(t1920: F, t28630: F, t968: F, t23384: F, t28618: F, t28671: F, t82736: F, t28610: F, t28557: F, t6743: F, t28660: F, t28614: F, t362: F, t5914: F, t28719: F, t3216: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100324 = t1920 * t968 * t28630;
    let t100378 = t23384 * t28618;
    let t100390 = t82736 * t28671;
    let t100399 = t23384 * t28610;
    let t100417 = t28557 * t6743;
    let t100431 = t23384 * t28660;
    let t100436 = t23384 * t28614;
    let t100449 = t362 * t5914;
    let t100497 = t28719 * t3216;
    (t100324, t100378, t100390, t100399, t100417, t100431, t100436, t100449, t100497)
}
