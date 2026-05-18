//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 998/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk998<F: Float>(t76315: F, t352: F, t77960: F, t8940: F, t25877: F, t77094: F, t25854: F, t77097: F, t76323: F, t25820: F, t77085: F, t27101: F, t77088: F) -> (F, F, F, F, F, F, F) {
    let t78040 = F::new(0.20455996240684006296e-1) * t76315;
    let t78046 = F::new(0.11974241701863808564e0) * t8940 * t77960 * t352;
    let t78047 = t25877 * t77094;
    let t78048 = F::new(0.17961362552795712846e0) * t78047;
    let t78049 = t25854 * t77097;
    let t78050 = F::new(0.8980681276397856423e-1) * t78049;
    let t78051 = F::new(0.14967802127329760705e-1) * t76323;
    let t78052 = t25820 * t77085;
    let t78053 = F::new(0.8980681276397856423e-1) * t78052;
    let t78054 = t27101 * t77088;
    (t78040, t78046, t78048, t78050, t78051, t78053, t78054)
}
