//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1048/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1048<F: Float>(t265: F, t394: F, t1068: F, t1070: F, t1637: F, t193: F, t23738: F, t23742: F, t25836: F, t25840: F, t25845: F, t25882: F, t336: F, t4696: F, t4700: F, t6822: F) -> F {
    let t395 = t265 < t394;
    let t25883 = piecewise3::<F>(t395, t1070 * t193 * t25836 * t336 - t1068 * t25840 * t4700 - t1637 * t23738 * t4700 + F::cast_from(2.0_f64) * t23742 * t25845 * t4700 - t4696 * t4700 * t6822, t25882);
    t25883
}
