//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 906/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk906<F: Float>(t27101: F, t75845: F, t35960: F, t649: F, t8985: F, t40928: F, t8976: F, t8947: F, t11704: F, t14293: F, t14296: F, t1652: F, t27: F, t29: F) -> (F, F, F, F, F, F) {
    let t76253 = F::new(0.11974241701863808564e0) * t27101 * t75845;
    let t76255 = t35960 * t649 * t8985;
    let t76258 = t40928 * t649 * t8976;
    let t76262 = t35960 * t649 * t8947;
    let t76264 = t14293 * t11704;
    let t76268 = t14296 * t27 * t29 * t1652;
    (t76253, t76255, t76258, t76262, t76264, t76268)
}
