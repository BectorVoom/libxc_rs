//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1992/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1992<F: Float>(t25: F, t265: F, t394: F, t101892: F, t101937: F, t101209: F, t101248: F, t101283: F, t101843: F, t1409: F, t16558: F, t2064: F, t26807: F, t29149: F, t3966: F, t40: F, t5398: F, t607: F, t7131: F, t7865: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t101938 = t101892 + t101937;
    let t101939 = piecewise3::<F>(t395, F::cast_from(0.0_f64), t101938);
    let t101951 = piecewise3::<F>(t115, t101209 + t101248 + t101283 + t101843, t101939 * t40 / F::cast_from(2.0_f64) + t29149 * t607 / F::cast_from(2.0_f64) + t26807 * t1409 + t7865 * t3966 + t7131 * t5398 / F::cast_from(2.0_f64) + t2064 * t16558 / F::cast_from(2.0_f64));
    (t101938, t101951)
}
