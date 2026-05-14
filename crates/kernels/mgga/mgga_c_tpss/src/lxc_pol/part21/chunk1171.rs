//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1171/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1171<F: Float>(t1726: F, t2715: F, t2723: F, t2785: F, t2724: F, t5623: F, t940: F) -> (F, F, F, F, F) {
    let t18172 = t2715 * t1726;
    let t18173 = t2723 * t2785;
    let t18174 = t18173 * t2724;
    let t18175 = t18172 * t18174;
    let t18178 = t940 * t5623;
    (t18172, t18173, t18174, t18175, t18178)
}
