//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2030/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2030<F: Float>(t1984: F, t80845: F, t2010: F, t6973: F, t80742: F, t22724: F, t22727: F, t22894: F, t80670: F, t22882: F, t22892: F, t22893: F) -> (F, F, F, F, F, F) {
    let t81071 = t80845 * t1984;
    let t81072 = t81071 * t2010;
    let t81073 = F::cast_from(0.27720185200590482541e0_f64) * t81072;
    let t81074 = t80742 * t6973;
    let t81075 = F::cast_from(0.16220877603642232915e0_f64) * t81074;
    let t81076 = t22724 * t22727;
    let t81080 = t80670 * t22894;
    let t81083 = t22892 * t22893 * t22882;
    (t81071, t81073, t81075, t81076, t81080, t81083)
}
