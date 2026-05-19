//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1285/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1285<F: Float>(t32514: F, t8009: F, t117855: F, t118059: F, t118067: F, t1186: F, t1238: F, t15797: F, t1716: F, t24615: F, t24893: F, t27406: F, t27411: F, t27415: F, t27741: F, t27760: F, t27792: F, t27830: F, t32480: F, t32489: F, t32530: F, t32543: F, t3598: F, t4930: F, t5055: F, t7283: F, t7300: F, t7301: F, t7356: F, t7391: F, t7392: F, t8061: F, t8087: F, t8088: F, t8867: F, t8898: F) -> F {
    let t125662 = t8009 * t32514;
    let t125668 = F::new(2.0) * t5055 * t32489 + F::new(4.0) * t24893 * t8061 - F::new(2.0) * t27792 * t7392 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t4930 * t8867 + F::new(4.0) * t27830 * t7356 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t32543 * t27415 - F::cast_from(0.54831135561607547883e-2_f64) * t118059 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t7300 * t7301 * t27741 + F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t7300 * t24615 * t27760 - t5055 * t32480 - t15797 * t8898 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t117855 + F::new(4.0) * t1238 * t3598 * t7391 * t8087 + t118067 + F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t32543 * t27411 - F::new(2.0) * t24893 * t8088 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1186 * t125662 + F::cast_from(0.43864908449286038307e-1_f64) * t27406 * t32530;
    t125668
}
