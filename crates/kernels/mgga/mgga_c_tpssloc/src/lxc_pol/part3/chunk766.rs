//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 766/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk766<F: Float>(t4182: F, t4282: F, t1510: F, t2732: F, t4234: F, t860: F, t68: F, t814: F, t226: F, t829: F, t1519: F, t235: F, t4265: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4283 = t4282 * t4182;
    let t4286 = t2732 * t1510;
    let t4288 = t860 * t4234;
    let t4290 = t68 * t814;
    let t4291 = t226 * t4290;
    let t4292 = t4282 * t829;
    let t4295 = t814 * t1519;
    let t4296 = t4295 * t829;
    let t4298 = t235 * t4265;
    (t4283, t4286, t4288, t4290, t4291, t4292, t4295, t4296, t4298)
}
