//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 434/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk434<F: Float>(t480: F, t11: F, t2: F, t584: F, t16: F, t9: F) -> (F, F, F, F, F) {
    let t2130 = t480 * t480;
    let t2218 = F::cast_from(0.174e1_f64) * t11;
    let t2219 = t2 * t584;
    let t2220 = F::cast_from(0.696e1_f64) * t2219;
    let t2221 = t9 * t16;
    (t2130, t2218, t2219, t2220, t2221)
}
