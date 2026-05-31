//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 993/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk993<F: Float>(t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1756: F, t1758: F, t3604: F, t3610: F, t3624: F, t470: F, t494: F, t4964: F, t5064: F, t5069: F, t5073: F, t5076: F, t5080: F, t5084: F, t5086: F) -> F {
    let t5088 = t1201 * t1758 + t1244 * t5073 + t1244 * t5076 + t1244 * t5084 + t1247 * t5064 + t1249 * t1729 + t1756 * t3604 + F::cast_from(2.0_f64) * t3610 * t5069 - t3624 * t5080 + t470 * t5086 + t494 * t4964;
    t5088
}
