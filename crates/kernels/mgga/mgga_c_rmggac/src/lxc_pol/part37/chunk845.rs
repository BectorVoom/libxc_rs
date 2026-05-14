//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 845/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk845<F: Float>(t78030: F, t570: F, t71910: F, t8940: F, t72027: F, t118: F, t77416: F, t76313: F, t76315: F, t352: F, t77960: F, t25877: F, t77094: F, t25854: F, t77097: F, t76323: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t78031 = 0.2993560425465952141e-1 * t78030;
    let t78034 = 0.11974241701863808564e0 * t8940 * t71910 * t570;
    let t78036 = 0.11974241701863808564e0 * t72027;
    let t78038 = 0.39914139006212695214e-1 * t118 * t77416;
    let t78039 = 0.20455996240684006296e-1 * t76313;
    let t78040 = 0.20455996240684006296e-1 * t76315;
    let t78046 = 0.11974241701863808564e0 * t8940 * t77960 * t352;
    let t78047 = t25877 * t77094;
    let t78048 = 0.17961362552795712846e0 * t78047;
    let t78049 = t25854 * t77097;
    let t78050 = 0.8980681276397856423e-1 * t78049;
    let t78051 = 0.14967802127329760705e-1 * t76323;
    (t78031, t78034, t78036, t78038, t78039, t78040, t78046, t78048, t78050, t78051)
}
