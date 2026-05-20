//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2173/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2173<F: Float>(t22765: F, t6417: F, t6390: F, t80997: F, t81000: F, t1351: F, t3788: F, t6388: F, t6936: F, t19958: F, t22833: F, t80867: F, t80886: F, t91304: F, t91311: F, t91323: F, t91328: F, t91345: F, t91346: F, t91357: F, t91359: F, t91365: F, t93721: F, t93723: F) -> F {
    let t97378 = t22765 * t6417;
    let t97380 = t80997 * t6390;
    let t97382 = t81000 * t6390;
    let t97387 = t6936 * t3788 * t6388 * t1351;
    let t97389 = t22833 * t19958;
    let t97392 = -t91304 + t93721 + t91311 - t93723 - F::new(119.0) / F::new(1728.0) * t80867 + F::new(7.0) / F::new(2304.0) * t97378 - F::new(7.0) / F::new(1152.0) * t97380 + t97382 / F::new(768.0) + F::cast_from(0.20186378047070195427e-3_f64) * t91323 + t91328 + F::cast_from(0.12111826828242117256e-2_f64) * t97387 + t97389 / F::new(384.0) - t91345 + F::cast_from(0.33643963411783659045e-4_f64) * t91346 - t80886 - t91357 + t91359 - t91365;
    t97392
}
