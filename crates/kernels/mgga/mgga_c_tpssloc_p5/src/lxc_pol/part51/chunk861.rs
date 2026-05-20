//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 861/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk861<F: Float>(t2006: F, t225: F, t567: F, t214: F, t1985: F, t2015: F, t6906: F) -> (F, F, F, F) {
    let t8454 = t2006 * t225 * t567;
    let t8455 = t214 * t8454;
    let t8457 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t8455;
    let t8458 = t6906 * t2015;
    (t8454, t8455, t8457, t8458)
}
