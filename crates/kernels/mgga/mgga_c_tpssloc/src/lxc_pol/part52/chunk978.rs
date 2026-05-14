//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 978/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk978<F: Float>(t1865: F, t22523: F, t22554: F, t26055: F, t26063: F, t26067: F, t26070: F, t26073: F, t26076: F, t26084: F, t26090: F, t6490: F, t6492: F, t6495: F, t6506: F, t6510: F, t7432: F, t7435: F, t7442: F, t7446: F) -> (F,) {
    let t26095 = t26055 * t1865 / 3.0 + 5.0 / 6.0 * t22554 * t7432 + 5.0 / 6.0 * t22523 * t7432 + 5.0 / 6.0 * t6490 * t26063 + 5.0 / 6.0 * t6490 * t26067 + t26070 * t1865 / 3.0 + t26073 * t1865 / 3.0 + t26076 * t1865 / 3.0 + t7435 * t6506 / 3.0 + t7435 * t6510 / 3.0 + 5.0 / 6.0 * t26084 * t6492 + t6495 * t7442 / 3.0 + 5.0 / 6.0 * t6490 * t26090 + t6495 * t7446 / 3.0;
    (t26095,)
}
