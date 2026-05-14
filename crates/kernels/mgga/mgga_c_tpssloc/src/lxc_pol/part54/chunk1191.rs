//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1191/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1191<F: Float>(t1998: F, t5187: F, t59: F, t6926: F, t5287: F, t6936: F, t6943: F, t22779: F, t32714: F, t5230: F, t8465: F, t8467: F, t1814: F, t31175: F, t26288: F, t5308: F, t6950: F) -> (F, F, F, F, F, F) {
    let t120405 = t6926 * t1998 * t59 * t5187;
    let t120408 = t6936 * t6943 * t5287;
    let t120410 = t22779 * t32714;
    let t120413 = t5230 * t8465 * t8467;
    let t120416 = t1814 * t31175 * t8467;
    let t120419 = t26288 * t6950 * t5308;
    (t120405, t120408, t120410, t120413, t120416, t120419)
}
