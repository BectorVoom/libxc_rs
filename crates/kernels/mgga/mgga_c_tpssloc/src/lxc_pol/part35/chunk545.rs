//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 545/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk545<F: Float>(t4211: F, t758: F, t228: F, t68: F, t1484: F, t845: F, t1516: F, t2697: F, t1520: F, t225: F) -> (F, F, F, F, F) {
    let t4212 = t4211 * t758;
    let t4225 = t228 * t68;
    let t4226 = t845 * t1484;
    let t4253 = t2697 * t1516;
    let t4268 = t1520 * t225;
    (t4212, t4225, t4226, t4253, t4268)
}
