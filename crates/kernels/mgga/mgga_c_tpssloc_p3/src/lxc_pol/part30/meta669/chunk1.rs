//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2098/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2098<F: Float>(t91402: F, t22804: F, t26277: F, t225: F, t26221: F, t22674: F, t22892: F, t26189: F, t26329: F, t26229: F, t22724: F, t26344: F) -> (F, F, F, F, F, F, F) {
    let t91403 = F::new(7.0) / F::new(72.0) * t91402;
    let t91404 = t22804 * t26277;
    let t91441 = t26221 * t225;
    let t91486 = t22892 * t22674 * t26189;
    let t91487 = F::cast_from(0.16449340668482264365e-1_f64) * t91486;
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t91531 = t22724 * t26344;
    (t91403, t91404, t91441, t91487, t91488, t91491, t91531)
}
