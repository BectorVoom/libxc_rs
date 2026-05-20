//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1949/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1949<F: Float>(t265: F, t394: F, t1070: F, t1637: F, t193: F, t23742: F, t25840: F, t28719: F, t28755: F, t336: F, t4700: F, t5946: F, t5950: F, t6822: F) -> F {
    let t395 = t265 < t394;
    let t28756 = piecewise3::<F>(t395, t1070 * t193 * t28719 * t336 - F::new(2.0) * t1637 * t25840 * t4700 + F::new(2.0) * t23742 * t4700 * t5950 - t4700 * t5946 * t6822, t28755);
    t28756
}
