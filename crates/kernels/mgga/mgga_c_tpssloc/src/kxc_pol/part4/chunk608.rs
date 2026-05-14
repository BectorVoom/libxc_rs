//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 608/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk608<F: Float>(t109: F, t4059: F, t662: F, t103: F, t2: F, t584: F, t100: F, t1445: F, t1447: F, t4050: F, t4054: F, t657: F, t663: F, t92: F, t656: F, t2327: F, t2328: F, t4041: F, t4044: F, t64: F) -> (F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t4060 = t4059 * t662;
    let t4063 = t103 * t2;
    let t4064 = t4063 * t584;
    let t4067 = -25.0 / 9.0 * t657 * t1445 + 10.0 / 9.0 * t92 * t4050 + 5.0 / 3.0 * t92 * t4054 - 25.0 / 9.0 * t1447 * t663 + 10.0 / 9.0 * t100 * t4060 - 5.0 / 3.0 * t100 * t4064;
    let t4068 = t656 * t4067;
    let t4072 = piecewise3(t110, 0.0, t2327 + t2328 / 3.0 + t4041 / 3.0 + t64 * t4044 / 4.0 - t64 * t4068 / 8.0);
    (t4060, t4064, t4067, t4068, t4072)
}
