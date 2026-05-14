//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 931/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk931<F: Float>(t1338: F, t5318: F, t3866: F, t5310: F, t3799: F, t5289: F, t2371: F, t5154: F, t5151: F, t67: F, t758: F, t12365: F, t1827: F, t12300: F, t12418: F, t820: F) -> (F, F, F, F, F, F, F, F) {
    let t16132 = t1338 * t5318;
    let t16147 = 35.0 / 576.0 * t3866 * t5310;
    let t16159 = 7.0 / 2304.0 * t3799 * t5289;
    let t16164 = t5154 * t2371;
    let t16169 = t5151 * t67;
    let t16171 = 0.36622894612013090108e-3 * t16169 * t758;
    let t16211 = t12365 * t1827;
    let t16214 = 7.0 / 2304.0 * t12300 * t1827;
    let t16224 = t12418 * t820;
    (t16132, t16147, t16159, t16164, t16171, t16211, t16214, t16224)
}
