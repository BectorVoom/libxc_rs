//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 493/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk493<F: Float>(t225: F, t991: F, t1008: F, t191: F, t349: F, t1011: F, t68: F) -> (F, F, F, F) {
    let t3026 = t991 * t225;
    let t3030 = F::cast_from(1.0_f64) / t1008 / t191;
    let t3031 = t349 * t3030;
    let t3032 = t1011 * t68;
    (t3026, t3030, t3031, t3032)
}
