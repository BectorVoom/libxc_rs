//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 971/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk971<F: Float>(t1246: F, t32474: F, t32451: F, t493: F, t1201: F, t1244: F, t2121: F, t32456: F, t32459: F, t32462: F, t32466: F, t32470: F, t470: F, t7283: F, t7373: F, t8895: F) -> (F, F, F) {
    let t32475 = t32474 * t1246;
    let t32477 = t493 * t32451;
    let t32479 = t32456 - 0.54831135561607547883e-2 * t7283 * t32459 - 0.16449340668482264365e-1 * t7283 * t32462 + 0.16449340668482264365e-1 * t7373 * t32466 + 0.16449340668482264365e-1 * t2121 * t32470 + t1201 * t8895 + t1244 * t32475 + t470 * t32477;
    (t32475, t32477, t32479)
}
