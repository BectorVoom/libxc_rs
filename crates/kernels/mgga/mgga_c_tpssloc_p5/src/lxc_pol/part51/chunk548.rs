//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 548/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk548<F: Float>(t4053: F, t584: F, t1449: F, t2349: F, t662: F, t103: F, t2: F, t100: F, t1445: F, t1447: F, t4050: F, t657: F, t663: F, t92: F) -> F {
    let t4054 = t4053 * t584;
    let t4059 = t2349 * t1449;
    let t4060 = t4059 * t662;
    let t4063 = t103 * t2;
    let t4064 = t4063 * t584;
    let t4067 = -F::new(25.0) / F::new(9.0) * t657 * t1445 + F::new(10.0) / F::new(9.0) * t92 * t4050 + F::new(5.0) / F::new(3.0) * t92 * t4054 - F::new(25.0) / F::new(9.0) * t1447 * t663 + F::new(10.0) / F::new(9.0) * t100 * t4060 - F::new(5.0) / F::new(3.0) * t100 * t4064;
    t4067
}
