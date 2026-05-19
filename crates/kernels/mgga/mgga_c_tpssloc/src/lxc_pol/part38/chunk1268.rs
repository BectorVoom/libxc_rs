//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1268/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1268<F: Float>(t12236: F, t1315: F, t16083: F, t16086: F, t16090: F, t16099: F, t16101: F, t16103: F, t16106: F, t16108: F, t16113: F, t16115: F, t16119: F, t5195: F) -> F {
    let t16121 = -t16083 + F::cast_from(0.99999999999999999996e-2_f64) * t5195 * t16086 + F::cast_from(0.49999999999999999998e-2_f64) * t5195 * t16090 - t16099 - t12236 - F::cast_from(0.19999999999999999999e-1_f64) * t16101 * t16103 + F::cast_from(0.77777777777777777774e-2_f64) * t16106 - F::cast_from(0.52777777777777777776e-2_f64) * t16108 + t16113 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t16115 + F::cast_from(0.16666666666666666666e-2_f64) * t16119;
    t16121
}
