//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1217/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1217<F: Float>(t2553: F, t2632: F, t10024: F, t809: F, t10017: F, t838: F, t2614: F, t2693: F, t238: F, t244: F, t248: F, t40445: F) -> (F, F, F, F, F) {
    let t41123 = t2632 * t2553;
    let t41130 = t809 * t10024;
    let t41132 = t10017 * t838;
    let t41134 = t2614 * t2693;
    let t41139 = F::cast_from(13685.0_f64) / F::cast_from(31104.0_f64) * t238 * t40445 * t244 * t248;
    (t41123, t41130, t41132, t41134, t41139)
}
