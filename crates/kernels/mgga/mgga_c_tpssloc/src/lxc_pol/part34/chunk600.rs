//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 600/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk600<F: Float>(t1246: F, t6260: F, t3625: F, t6252: F, t493: F, t6238: F, t1244: F, t1729: F, t1756: F, t1758: F, t3610: F, t3624: F, t470: F, t494: F, t5064: F, t6168: F, t6253: F, t6257: F) -> (F, F, F, F) {
    let t6261 = t6260 * t1246;
    let t6263 = t6252 * t3625;
    let t6265 = t493 * t6238;
    let t6267 = F::new(2.0) * t1244 * t6257 + t1244 * t6261 + F::new(2.0) * t1729 * t1758 + F::new(2.0) * t1756 * t5064 + F::new(2.0) * t3610 * t6253 - t3624 * t6263 + t470 * t6265 + t494 * t6168;
    (t6261, t6263, t6265, t6267)
}
