//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 628/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk628<F: Float>(t103: F, t2: F, t584: F, t100: F, t1445: F, t1447: F, t4050: F, t4054: F, t4060: F, t657: F, t663: F, t92: F) -> (F, F) {
    let t4063 = t103 * t2;
    let t4064 = t4063 * t584;
    let t4067 = -25.0 / 9.0 * t657 * t1445 + 10.0 / 9.0 * t92 * t4050 + 5.0 / 3.0 * t92 * t4054 - 25.0 / 9.0 * t1447 * t663 + 10.0 / 9.0 * t100 * t4060 - 5.0 / 3.0 * t100 * t4064;
    (t4064, t4067)
}
