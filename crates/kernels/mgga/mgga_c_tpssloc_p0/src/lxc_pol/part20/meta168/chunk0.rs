//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1055/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1055<F: Float>(t33: F, t3997: F, t1409: F, t2291: F, t3966: F, t634: F, t2298: F, t638: F, t607: F, t72: F) -> (F, F, F, F, F, F) {
    let t3998 = t33 * t3997;
    let t4007 = t2291 * t1409;
    let t4010 = t634 * t3966;
    let t4012 = t2298 * t1409;
    let t4015 = t638 * t3966;
    let t4017 = F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4007 * t607 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4010 + F::cast_from(28.0_f64) / F::cast_from(9.0_f64) * t4012 * t607 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4015;
    let t4018 = t72 * t4017;
    (t3998, t4007, t4010, t4012, t4015, t4018)
}
