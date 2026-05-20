//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1959/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1959<F: Float>(t23967: F, t26067: F, t2032: F, t22519: F, t23975: F, t26055: F, t26070: F, t26090: F, t26945: F, t6495: F, t7026: F, t7035: F, t7782: F, t90150: F, t90177: F, t90334: F, t90337: F, t90340: F, t90343: F) -> F {
    let t91980 = F::new(80.0) / F::new(9.0) * t23967 * t26067;
    let t91993 = -F::new(4.0) / F::new(3.0) * t26055 * t7035 - F::new(10.0) / F::new(3.0) * t23975 * t26090 - F::new(4.0) / F::new(3.0) * t22519 * t7782 - F::new(10.0) / F::new(3.0) * t7026 * t90177 - F::new(4.0) / F::new(3.0) * t6495 * t26945 + t91980 - F::new(5.0) / F::new(3.0) * t7026 * t90334 - F::new(2.0) / F::new(3.0) * t90337 * t2032 - F::new(4.0) / F::new(3.0) * t90340 * t2032 - F::new(4.0) / F::new(3.0) * t90343 * t2032 - F::new(4.0) / F::new(3.0) * t26070 * t7035 - F::new(2.0) / F::new(3.0) * t90150 * t2032;
    t91993
}
