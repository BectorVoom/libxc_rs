//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1199/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1199<F: Float>(t225: F, t24162: F, t81317: F, t12030: F, t12437: F, t1375: F, t1386: F, t2091: F, t2092: F, t24082: F, t3887: F, t3911: F, t3912: F, t39910: F, t7199: F, t7213: F, t81307: F, t81311: F, t81315: F, t81328: F) -> F {
    let t84655 = t24162 * t225;
    let t84659 = F::cast_from(0.55440370401180965083e0_f64) * t81317;
    let t84667 = F::new(2.0) * t1375 * t3887 * t2091 * t12437 + F::new(6.0) * t12030 * t7199 - F::cast_from(0.11514538467937585055e0_f64) * t81307 - F::cast_from(0.49348022005446793095e-1_f64) * t81311 - t39910 * t2092 - F::new(3.0) * t84655 * t1386 + F::cast_from(0.9869604401089358619e-1_f64) * t81315 - t84659 - F::cast_from(0.9869604401089358619e-1_f64) * t81328 + F::new(6.0) * t1375 * t3887 * t7213 * t3911 - F::new(3.0) * t24082 * t3912;
    t84667
}
