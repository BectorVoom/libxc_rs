//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1252/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1252<F: Float>(t607: F, t63: F, t193: F, t201: F, t7109: F, t10143: F, t111: F, t7415: F, t25: F, t40772: F, t1408: F, t2752: F) -> (F, F, F, F, F, F) {
    let t84186 = t607 * t63;
    let t84797 = t193 * t201 * t7109;
    let t84800 = t7109 * t10143;
    let t85416 = t7415 * t111;
    let t86716 = t40772 * t25;
    let t86721 = t2752 * t1408;
    (t84186, t84797, t84800, t85416, t86716, t86721)
}
