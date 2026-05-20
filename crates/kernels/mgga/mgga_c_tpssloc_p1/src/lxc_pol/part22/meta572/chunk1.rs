//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2081/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2081<F: Float>(t43776: F, t2296: F, t3241: F, t11778: F, t154: F, t1091: F, t9698: F) -> (F, F, F, F) {
    let t43777 = F::cast_from(0.13490888888888888889e1_f64) * t43776;
    let t43791 = F::new(1.0) / t3241 / t2296;
    let t43809 = t154 * t11778;
    let t43816 = t9698 * t1091;
    (t43777, t43791, t43809, t43816)
}
