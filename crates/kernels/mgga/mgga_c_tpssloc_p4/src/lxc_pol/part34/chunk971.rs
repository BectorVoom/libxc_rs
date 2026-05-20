//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 971/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk971<F: Float>(t1755: F, t22368: F, t22364: F, t3625: F, t22327: F, t493: F, t22243: F, t491: F, t1246: F, t1751: F, t6218: F, t11881: F, t11888: F, t11914: F, t1244: F, t15027: F, t15245: F, t1729: F, t1756: F, t1758: F, t19201: F, t22114: F, t22341: F, t22349: F, t22355: F, t22358: F, t22361: F, t22365: F, t3610: F, t3624: F, t470: F, t494: F, t5064: F, t6168: F, t6253: F, t6257: F, t6261: F, t6263: F, t6265: F) -> F {
    let t22369 = t1755 * t22368;
    let t22372 = t22364 * t3625;
    let t22375 = t493 * t22327;
    let t22386 = t491 * t22243;
    let t22387 = t22386 * t1246;
    let t22389 = t1751 * t6218;
    let t22390 = t22389 * t1246;
    let t22393 = F::new(3.0) * t1244 * t22341 + F::new(3.0) * t5064 * t6261 + F::new(6.0) * t5064 * t6257 + t11914 * t22349 + F::new(3.0) * t19201 * t1756 - F::new(3.0) * t3624 * t22355 + F::new(6.0) * t11881 * t22358 - F::new(6.0) * t11888 * t22361 + F::new(6.0) * t3610 * t22365 + F::new(6.0) * t3610 * t22369 - F::new(3.0) * t3624 * t22372 + t470 * t22375 + F::new(3.0) * t1729 * t6265 + F::new(6.0) * t15027 * t6253 - F::new(3.0) * t15245 * t6263 + t22114 * t494 + F::new(3.0) * t6168 * t1758 + t1244 * t22387 + F::new(3.0) * t1244 * t22390;
    t22393
}
