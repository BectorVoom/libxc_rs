//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1992/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1992<F: Float>(t87796: F, t87804: F, t13071: F, t13460: F, t2053: F, t2054: F, t24305: F, t24330: F, t25168: F, t2597: F, t26700: F, t26703: F, t26713: F, t26728: F, t2718: F, t2720: F, t4268: F, t4273: F, t46452: F, t82230: F, t82236: F, t855: F, t87822: F) -> (F, F, F) {
    let t92872 = F::cast_from(0.76763589786250567036e-1_f64) * t87796;
    let t92874 = F::cast_from(0.76763589786250567036e-1_f64) * t87804;
    let t92907 = F::new(2.0) * t26713 * t2720 - F::cast_from(0.76763589786250567036e-1_f64) * t82230 + F::new(2.0) * t4268 * t24330 - F::cast_from(0.3289868133696452873e-1_f64) * t87822 - F::cast_from(0.82246703342411321825e-2_f64) * t82236 - F::new(12.0) * t25168 * t26728 * t13071 + F::new(4.0) * t24305 * t4273 + F::new(2.0) * t855 * t2718 * t2053 * t13460 + F::new(2.0) * t26700 * t2720 + F::new(4.0) * t2597 * t26703 - t46452 * t2054;
    (t92872, t92874, t92907)
}
