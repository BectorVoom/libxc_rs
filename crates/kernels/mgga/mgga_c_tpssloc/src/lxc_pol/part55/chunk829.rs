//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 829/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk829<F: Float>(t24574: F, t7303: F, t7291: F, t2123: F, t3427: F, t2121: F, t221: F, t3448: F, t2127: F) -> (F, F, F, F, F) {
    let t24575 = t24574 * t7303;
    let t24577 = t24574 * t7291;
    let t24585 = t3427 * t2123;
    let t24587 = 0.18277045187202515961e-2 * t2121 * t24585;
    let t24588 = t221 * t3448;
    let t24589 = t2127 * t24588;
    (t24575, t24577, t24587, t24588, t24589)
}
