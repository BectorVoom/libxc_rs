//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1731/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1731<F: Float>(t532: F, t6995: F, t6879: F, t1983: F, t2018: F, t531: F, t1390: F, t3734: F, t1868: F, t2319: F, t6876: F, t6997: F) -> (F, F, F, F, F, F, F, F) {
    let t22591 = t532 * t6995;
    let t22592 = t22591 * t6879;
    let t22594 = F::cast_from(6.0_f64) * t1983 * t22592;
    let t22595 = t531 * t2018;
    let t22596 = t1390 * t3734;
    let t22597 = t22595 * t22596;
    let t22599 = F::cast_from(6.0_f64) * t1983 * t22597;
    let t22600 = t1868 * t2319;
    let t22605 = F::cast_from(2.0_f64) * t6876 * t6997;
    (t22591, t22592, t22594, t22596, t22597, t22599, t22600, t22605)
}
