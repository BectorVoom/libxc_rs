//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2468/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2468<F: Float>(t14562: F, t225: F, t10160: F, t10170: F, t10182: F, t1052: F, t1066: F, t11010: F, t11084: F, t11085: F, t14529: F, t14545: F, t14549: F, t1634: F, t1635: F, t3020: F, t3026: F, t3174: F, t3207: F, t388: F, t43431: F, t4657: F, t4660: F, t4665: F, t4694: F) -> F {
    let t50653 = t14562 * t225;
    let t50678 = F::new(2.0) * t1052 * t11084 * t1634 * t3174 + F::new(3.0) * t3020 * t388 * t4657 + F::new(12.0) * t10160 * t4665 - F::new(3.0) * t10170 * t4694 + F::new(6.0) * t10182 * t4660 - F::new(6.0) * t1066 * t50653 + F::new(6.0) * t11010 * t4665 - F::new(3.0) * t11010 * t4694 - t11085 * t4660 - F::new(3.0) * t14529 * t3207 - F::new(3.0) * t14545 * t3207 + F::new(6.0) * t14549 * t3026 - F::new(3.0) * t1635 * t43431;
    t50678
}
