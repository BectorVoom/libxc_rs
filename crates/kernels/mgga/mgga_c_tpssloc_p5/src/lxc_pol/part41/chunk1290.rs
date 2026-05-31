//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1290/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1290<F: Float>(t2331: F, t2585: F, t1851: F, t8217: F, t110075: F, t30281: F, t29895: F, t30285: F, t30304: F, t29900: F, t30308: F, t110140: F, t8262: F) -> (F, F, F, F, F, F, F) {
    let t110601 = t2585 * t2331;
    let t110919 = F::cast_from(2.0_f64) * t1851 * t8217;
    let t111056 = F::cast_from(4.0_f64) * t110075 * t30281;
    let t111058 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t29895 * t30285;
    let t111077 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t29895 * t30304;
    let t111079 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t29900 * t30308;
    let t111101 = t110140 * t8262;
    (t110601, t110919, t111056, t111058, t111077, t111079, t111101)
}
