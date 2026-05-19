//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 967/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk967<F: Float>(t22833: F, t5293: F, t5303: F, t1351: F, t16311: F, t3788: F, t6936: F, t16306: F, t550: F, t1339: F, t22856: F, t22859: F, t22860: F, t22864: F, t22868: F, t26306: F, t26310: F) -> (F, F, F) {
    let t26312 = t22833 * t5293;
    let t26314 = t22833 * t5303;
    let t26318 = t16311 * t1351;
    let t26319 = t3788 * t26318;
    let t26320 = t6936 * t26319;
    let t26322 = t16306 * t550;
    let t26323 = t1339 * t26322;
    let t26324 = t6936 * t26323;
    let t26326 = t26306 / F::new(384.0) + t26310 / F::new(768.0) - t26312 / F::new(1536.0) + t26314 / F::new(384.0) + F::cast_from(0.33643963411783659045e-4_f64) * t22856 + t22859 - F::new(7.0) / F::new(2304.0) * t22860 + t22864 + t22868 + F::cast_from(0.40372756094140390854e-3_f64) * t26320 - F::cast_from(0.20186378047070195427e-3_f64) * t26324;
    (t26318, t26322, t26326)
}
