//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1216/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1216<F: Float>(t265: F, t394: F, t1068: F, t1070: F, t193: F, t23734: F, t23738: F, t23742: F, t23772: F, t3209: F, t3213: F, t336: F, t4700: F, t6822: F) -> F {
    let t395 = t265 < t394;
    let t23773 = piecewise3::<F>(t395, t1070 * t193 * t23734 * t336 - F::cast_from(2.0_f64) * t1068 * t23738 * t4700 + F::cast_from(2.0_f64) * t23742 * t3213 * t4700 - t3209 * t4700 * t6822, t23772);
    t23773
}
