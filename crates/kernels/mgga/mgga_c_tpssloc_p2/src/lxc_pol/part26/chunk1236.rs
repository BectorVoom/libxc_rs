//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1236/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1236<F: Float>(t81074: F, t22724: F, t22727: F, t22894: F, t80670: F, t22882: F, t22892: F, t22893: F, t12156: F, t6637: F, t6968: F, t80732: F) -> (F, F, F, F, F) {
    let t81075 = F::cast_from(0.16220877603642232915e0_f64) * t81074;
    let t81076 = t22724 * t22727;
    let t81080 = t80670 * t22894;
    let t81083 = t22892 * t22893 * t22882;
    let t81087 = t80732 * t6637 * t6968 * t12156;
    (t81075, t81076, t81080, t81083, t81087)
}
