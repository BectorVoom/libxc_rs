//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 997/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk997<F: Float>(t78026: F, t76305: F, t14451: F, t1652: F, t5148: F, t570: F, t71910: F, t8940: F, t72027: F, t118: F, t77416: F, t76313: F) -> (F, F, F, F, F, F, F) {
    let t78027 = F::cast_from(0.2993560425465952141e-1_f64) * t78026;
    let t78028 = F::cast_from(0.79828278012425390427e-1_f64) * t76305;
    let t78030 = t5148 * t14451 * t1652;
    let t78031 = F::cast_from(0.2993560425465952141e-1_f64) * t78030;
    let t78034 = F::cast_from(0.11974241701863808564e0_f64) * t8940 * t71910 * t570;
    let t78036 = F::cast_from(0.11974241701863808564e0_f64) * t72027;
    let t78038 = F::cast_from(0.39914139006212695214e-1_f64) * t118 * t77416;
    let t78039 = F::cast_from(0.20455996240684006296e-1_f64) * t76313;
    (t78027, t78028, t78031, t78034, t78036, t78038, t78039)
}
