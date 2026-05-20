//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1667/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1667<F: Float>(t23967: F, t7432: F, t7032: F, t7435: F, t2032: F, t23975: F, t26055: F, t26063: F, t26067: F, t26070: F, t26073: F, t26076: F, t26090: F, t26911: F, t6492: F, t6495: F, t7026: F, t7035: F, t7782: F) -> (F, F, F) {
    let t26920 = t23967 * t7432;
    let t26936 = t7435 * t7032;
    let t26938 = -F::new(5.0) / F::new(3.0) * t26911 * t6492 - F::new(2.0) / F::new(3.0) * t26055 * t2032 - F::new(5.0) / F::new(3.0) * t23975 * t7432 - F::new(5.0) / F::new(3.0) * t7026 * t26063 + F::new(40.0) / F::new(9.0) * t26920 - F::new(5.0) / F::new(3.0) * t7026 * t26067 - F::new(2.0) / F::new(3.0) * t26070 * t2032 - F::new(2.0) / F::new(3.0) * t26073 * t2032 - F::new(2.0) / F::new(3.0) * t26076 * t2032 - F::new(2.0) / F::new(3.0) * t7435 * t7035 - F::new(5.0) / F::new(3.0) * t7026 * t26090 - F::new(2.0) / F::new(3.0) * t6495 * t7782 + F::new(16.0) / F::new(9.0) * t26936;
    (t26920, t26936, t26938)
}
