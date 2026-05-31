//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1751/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1751<F: Float>(t1926: F, t3158: F, t40: F, t6722: F, t1937: F, t6712: F, t995: F, t1942: F, t3082: F, t344: F, t1009: F, t6740: F) -> (F, F, F, F, F, F, F) {
    let t23447 = t1926 * t3158 / F::cast_from(432.0_f64);
    let t23448 = t6722 * t40;
    let t23449 = t23448 * t1937;
    let t23463 = t6712 * t995;
    let t23469 = t1942 * t3082 / F::cast_from(6912.0_f64);
    let t23470 = t40 * t344;
    let t23471 = t23470 * t1009;
    let t23472 = t6740 * t23471;
    (t23447, t23449, t23463, t23469, t23470, t23471, t23472)
}
