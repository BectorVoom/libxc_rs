//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1015/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1015<F: Float>(t24175: F, t7687: F, t6999: F, t7940: F, t532: F, t7939: F, t6879: F, t12571: F, t7025: F, t23967: F, t7432: F, t7032: F, t7435: F, t2032: F, t23975: F, t26055: F, t26063: F, t26067: F, t26070: F, t26073: F, t26076: F, t26090: F, t6492: F, t6495: F, t7026: F, t7035: F, t7782: F) -> (F, F, F, F) {
    let t26898 = t24175 * t7687;
    let t26902 = t7940 * t6999;
    let t26905 = t532 * t7939;
    let t26906 = t26905 * t6879;
    let t26911 = t12571 * t7025;
    let t26920 = t23967 * t7432;
    let t26936 = t7435 * t7032;
    let t26938 = -5.0 / 3.0 * t26911 * t6492 - 2.0 / 3.0 * t26055 * t2032 - 5.0 / 3.0 * t23975 * t7432 - 5.0 / 3.0 * t7026 * t26063 + 40.0 / 9.0 * t26920 - 5.0 / 3.0 * t7026 * t26067 - 2.0 / 3.0 * t26070 * t2032 - 2.0 / 3.0 * t26073 * t2032 - 2.0 / 3.0 * t26076 * t2032 - 2.0 / 3.0 * t7435 * t7035 - 5.0 / 3.0 * t7026 * t26090 - 2.0 / 3.0 * t6495 * t7782 + 16.0 / 9.0 * t26936;
    (t26898, t26902, t26906, t26938)
}
