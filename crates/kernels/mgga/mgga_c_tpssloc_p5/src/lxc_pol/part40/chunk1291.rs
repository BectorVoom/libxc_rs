//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1291/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1291<F: Float>(t30263: F, t576: F, t2193: F, t6470: F, t1851: F, t8256: F, t2186: F, t6483: F, t29895: F, t30411: F, t1453: F, t2: F) -> (F, F, F, F, F, F) {
    let t110910 = F::cast_from(2.0_f64) * t576 * t30263;
    let t111316 = t6470 * t2193;
    let t111317 = t1851 * t8256;
    let t111322 = t2186 * t6483;
    let t111326 = t29895 * t30411;
    let t111331 = t1453 * t2;
    (t110910, t111316, t111317, t111322, t111326, t111331)
}
