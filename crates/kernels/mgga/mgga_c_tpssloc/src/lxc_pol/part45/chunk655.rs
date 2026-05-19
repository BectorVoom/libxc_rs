//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 655/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk655<F: Float>(t8466: F, t8467: F, t1998: F, t2006: F, t214: F, t1985: F, t1401: F, t8326: F, t63: F, t8301: F) -> (F, F, F, F, F, F) {
    let t8468 = t8466 * t8467;
    let t8479 = t1998 * t2006;
    let t8480 = t214 * t8479;
    let t8482 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t8480;
    let t8508 = F::new(0.135e2) * t1401 * t8326;
    let t8511 = t8301 * t63;
    (t8468, t8479, t8480, t8482, t8508, t8511)
}
