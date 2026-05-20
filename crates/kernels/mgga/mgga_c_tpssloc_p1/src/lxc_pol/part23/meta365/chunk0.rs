//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1165/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1165<F: Float>(t43776: F, t2296: F, t3241: F, t11778: F, t154: F, t22715: F, t268: F, t405: F, t39267: F, t404: F, t410: F, t407: F) -> (F, F, F, F, F, F, F) {
    let t43777 = F::cast_from(0.13490888888888888889e1_f64) * t43776;
    let t43791 = F::new(1.0) / t3241 / t2296;
    let t43809 = t154 * t11778;
    let t43819 = t268 * t22715 * t405;
    let t43820 = F::new(280.0) / F::new(81.0) * t43819;
    let t43880 = F::new(1.0) / t410 / t39267 / t404 / F::new(96.0);
    let t43889 = F::powf(t407, -F::new(0.25e1));
    (t43777, t43791, t43809, t43819, t43820, t43880, t43889)
}
