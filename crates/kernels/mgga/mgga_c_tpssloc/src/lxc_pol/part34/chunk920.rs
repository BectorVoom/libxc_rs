//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 920/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk920<F: Float>(t21238: F, t942: F, t951: F, t959: F, t21093: F, t21097: F, t21099: F, t21103: F, t21105: F, t21107: F, t21365: F, t21367: F, t21369: F, t21375: F) -> (F, F) {
    let t21589 = t942 * t21238 * t951;
    let t21591 = F::cast_from(0.5848223622634646207e0_f64) * t959 * t21589;
    let t21592 = t21367 + t21375 + t21369 - t21093 + t21097 - t21591 + t21365 - t21099 - t21105 - t21107 - t21103;
    (t21591, t21592)
}
