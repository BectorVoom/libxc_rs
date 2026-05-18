//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1218/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1218<F: Float>(t107460: F, t107464: F, t107484: F, t107908: F, t107928: F, t107951: F, t107987: F, t1375: F, t1378: F, t20608: F, t2091: F, t29311: F, t40591: F, t5215: F, t84705: F, t91531: F, t91548: F, t97732: F, t97750: F) -> F {
    let t107993 = F::new(0.19739208802178717238e0) * t107460 + F::new(0.29608813203268075857e0) * t107464 + F::new(0.9869604401089358619e-1) * t97732 + F::new(24.0) * t1375 * t40591 * t2091 * t20608 - F::new(0.15626873635058151147e0) * t91531 + F::new(12.0) * t5215 * t29311 - F::new(0.11514538467937585055e0) * t97750 + F::new(0.9869604401089358619e-1) * t91548 - t1375 * t1378 * (t107908 + t107928 + t107951 + t107987) - t84705 - F::new(0.39478417604357434476e0) * t107484;
    t107993
}
