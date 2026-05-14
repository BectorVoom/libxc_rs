//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1239/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1239<F: Float>(t17218: F, t5705: F, t21180: F, t4378: F, t48103: F, t68442: F, t68444: F, t68446: F, t68448: F, t68452: F, t68454: F, t68494: F, t68498: F, t68500: F, t77028: F, t77030: F) -> (F, F, F) {
    let t77032 = t17218 * t5705;
    let t77034 = t4378 * t21180;
    let t77037 = 0.23917333333333333333e1 * t68442 + 0.39862222222222222223e0 * t68444 + 0.44291358024691358024e0 * t68446 - 0.15944888888888888889e1 * t68448 - 0.13145066666666666666e1 * t68452 + 0.21908444444444444444e0 * t68454 + 0.97370864197530864199e0 * t48103 + 0.79724444444444444444e0 * t68494 - 0.23917333333333333333e1 * t68498 + 0.85451625e1 * t77028 - 0.379785e1 * t77030 - 0.46074375e0 * t77032 + 0.614325e0 * t77034 + 0.97370864197530864196e-1 * t68500;
    (t77032, t77034, t77037)
}
