//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 876/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk876<F: Float>(t2101: F, t600: F, t629: F, t98: F, t99: F, t2: F, t22: F, t106: F, t107: F, t10: F, t555: F, t551: F) -> (F, F, F, F, F, F, F) {
    let t7592 = t600 * t2101;
    let t7593 = t629 * t629;
    let t7594 = F::cast_from(1.0_f64) / t7593;
    let t7612 = t99 * t98;
    let t7613 = F::cast_from(1.0_f64) / t7612;
    let t7622 = t2 * t22;
    let t7628 = t107 * t106;
    let t7629 = F::cast_from(1.0_f64) / t7628;
    let t7651 = t10 * t555;
    let t7653 = t551 * t22;
    (t7592, t7594, t7613, t7622, t7629, t7651, t7653)
}
