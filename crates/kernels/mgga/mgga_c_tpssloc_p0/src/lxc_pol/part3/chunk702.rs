//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 702/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk702<F: Float>(t1932: F, t475: F, t3611: F, t3590: F, t493: F, t1201: F, t1244: F, t1247: F, t1249: F, t3565: F, t3604: F, t3610: F, t3613: F, t3617: F, t3621: F, t3624: F, t470: F, t494: F) -> (F, F, F, F) {
    let t3625 = t1932 * t475;
    let t3626 = t3611 * t3625;
    let t3628 = t493 * t3590;
    let t3630 = F::cast_from(2.0_f64) * t1201 * t1249 + F::cast_from(2.0_f64) * t1244 * t3617 + t1244 * t3621 + F::cast_from(2.0_f64) * t1247 * t3604 + t3565 * t494 + F::cast_from(2.0_f64) * t3610 * t3613 - t3624 * t3626 + t3628 * t470;
    (t3625, t3626, t3628, t3630)
}
