//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 704/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk704<F: Float>(t1454: F, t626: F, t1453: F, t2331: F, t666: F, t1444: F, t2341: F, t659: F, t2: F, t95: F, t584: F, t1449: F, t2349: F, t662: F, t103: F, t100: F, t1445: F, t1447: F, t657: F, t663: F, t92: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4041 = t626 * t1454;
    let t4043 = t2331 * t1453;
    let t4044 = t4043 * t666;
    let t4049 = t2341 * t1444;
    let t4050 = t4049 * t659;
    let t4053 = t95 * t2;
    let t4054 = t4053 * t584;
    let t4059 = t2349 * t1449;
    let t4060 = t4059 * t662;
    let t4063 = t103 * t2;
    let t4064 = t4063 * t584;
    let t4067 = -25.0 / 9.0 * t657 * t1445 + 10.0 / 9.0 * t92 * t4050 + 5.0 / 3.0 * t92 * t4054 - 25.0 / 9.0 * t1447 * t663 + 10.0 / 9.0 * t100 * t4060 - 5.0 / 3.0 * t100 * t4064;
    (t4041, t4043, t4044, t4049, t4050, t4053, t4054, t4059, t4063, t4067)
}
