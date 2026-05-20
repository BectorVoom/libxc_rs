//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1628/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1628<F: Float>(t1246: F, t19189: F, t19120: F, t493: F, t1243: F, t19045: F, t3612: F, t5011: F, t1755: F, t11881: F, t1201: F, t1244: F, t1247: F, t1249: F, t1729: F, t1758: F, t18572: F, t19166: F, t19170: F, t19174: F, t19176: F, t19180: F, t3604: F, t3610: F, t470: F, t494: F, t4964: F, t5064: F, t5073: F, t5076: F, t5086: F, t6168: F, t6257: F, t6265: F) -> (F, F, F) {
    let t19190 = t19189 * t1246;
    let t19197 = t493 * t19120;
    let t19201 = t19045 * t1243;
    let t19203 = t3612 * t5011;
    let t19204 = t1755 * t19203;
    let t19207 = F::new(6.0) * t11881 * t19166 + t1201 * t6265 + F::new(2.0) * t1244 * t19170 + t1244 * t19174 + F::new(2.0) * t1244 * t19180 + t1244 * t19190 + t1247 * t19201 + t1249 * t6168 + F::new(2.0) * t1729 * t5086 + F::new(2.0) * t1758 * t4964 + t18572 * t494 + F::new(2.0) * t19176 * t3610 + t19197 * t470 + F::new(4.0) * t19204 * t3610 + F::new(2.0) * t3604 * t6257 + F::new(2.0) * t5064 * t5073 + F::new(2.0) * t5064 * t5076;
    (t19201, t19203, t19207)
}
