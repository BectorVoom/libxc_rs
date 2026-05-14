//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1006/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1006<F: Float>(t8227: F, t256: F, t4701: F, t2112: F, t4678: F, t10708: F, t13335: F, t190: F, t681: F, t10698: F, t1342: F, t4741: F, t725: F, t14108: F, t150: F, t4579: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14147 = 0.10843581300301739842e-1 * t8227;
    let t14151 = t256 * t4701;
    let t14156 = 4.0 * t2112 * t4678;
    let t14157 = 8.0 * t10708;
    let t14158 = t190 * t13335;
    let t14160 = 4.0 * t681 * t14158;
    let t14162 = 8.0 * t10698 * t1342;
    let t14163 = t4741 * t725;
    let t14164 = t150 * t14108;
    let t14165 = t14164 * t190;
    let t14166 = t725 * t4579;
    (t14147, t14151, t14156, t14157, t14160, t14162, t14163, t14165, t14166)
}
