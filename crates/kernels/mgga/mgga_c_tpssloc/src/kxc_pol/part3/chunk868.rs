//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 868/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk868<F: Float>(t53: F, t54: F, t2585: F, t2769: F, t73: F, t3241: F, t76: F, t111: F, t2311: F, t107: F, t2281: F, t667: F) -> (F, F, F, F, F, F, F) {
    let t9300 = F::cast_from(1.0_f64) / t54 / t53;
    let t9311 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t2585;
    let t9321 = F::cast_from(1.0_f64) / t73 / t2769;
    let t9330 = F::cast_from(1.0_f64) / t76 / t3241;
    let t9348 = t2311 * t111;
    let t9358 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t2585 * t107;
    let t9359 = t2281 * t667;
    (t9300, t9311, t9321, t9330, t9348, t9358, t9359)
}
