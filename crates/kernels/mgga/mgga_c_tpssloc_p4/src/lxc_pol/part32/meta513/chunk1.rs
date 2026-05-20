//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1841/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1841<F: Float>(t26323: F, t6936: F, t22856: F, t22859: F, t22860: F, t22864: F, t22868: F, t26306: F, t26310: F, t26312: F, t26314: F, t26320: F) -> F {
    let t26324 = t6936 * t26323;
    let t26326 = t26306 / F::new(384.0) + t26310 / F::new(768.0) - t26312 / F::new(1536.0) + t26314 / F::new(384.0) + F::cast_from(0.33643963411783659045e-4_f64) * t22856 + t22859 - F::new(7.0) / F::new(2304.0) * t22860 + t22864 + t22868 + F::cast_from(0.40372756094140390854e-3_f64) * t26320 - F::cast_from(0.20186378047070195427e-3_f64) * t26324;
    t26326
}
