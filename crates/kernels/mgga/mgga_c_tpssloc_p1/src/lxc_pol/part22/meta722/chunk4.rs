//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2360/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2360<F: Float>(t1530: F, t16596: F, t16944: F, t17120: F, t1877: F, t2522: F, t41258: F, t41262: F, t4310: F, t4314: F, t46436: F, t59584: F, t67487: F, t67488: F, t67489: F, t67490: F, t67494: F) -> F {
    let t68391 = -F::new(3.0) * t1530 * t1877 * t59584 + F::new(18.0) * t16596 * t17120 * t2522 + F::new(36.0) * t16944 * t4310 * t4314 - t41258 - t41262 + t46436 - t67487 + t67488 - t67489 - t67490 + t67494;
    t68391
}
