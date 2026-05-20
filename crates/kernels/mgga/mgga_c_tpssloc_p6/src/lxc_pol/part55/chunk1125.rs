//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1125/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1125<F: Float>(t24615: F, t8060: F, t7300: F, t1760: F, t8887: F, t11606: F, t1653: F, t32514: F, t24601: F, t1716: F, t32515: F, t1238: F, t2155: F, t24589: F, t27406: F, t27830: F, t32498: F, t32542: F, t4945: F, t5055: F, t7283: F, t7351: F, t7999: F, t8061: F, t8088: F, t8868: F, t8872: F, t8888: F, t8898: F) -> (F, F, F, F, F, F, F) {
    let t34322 = t24615 * t8060;
    let t34323 = t7300 * t34322;
    let t34330 = t8887 * t1760;
    let t34331 = t11606 * t34330;
    let t34338 = t32514 * t1653;
    let t34339 = t24601 * t34338;
    let t34349 = t1716 * t32515;
    let t34352 = F::cast_from(0.3289868133696452873e-1_f64) * t7283 * t34323 + F::new(2.0) * t4945 * t8888 + F::new(2.0) * t5055 * t8888 - F::new(6.0) * t1238 * t34331 - F::new(2.0) * t7351 * t8088 + F::cast_from(0.43864908449286038307e-1_f64) * t27406 * t8872 + t32498 + F::cast_from(0.54831135561607547883e-2_f64) * t24589 * t34339 - F::new(2.0) * t27830 * t2155 - t5055 * t8898 + F::new(4.0) * t7351 * t8061 - F::cast_from(0.43864908449286038307e-1_f64) * t7999 * t8868 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t34349 - t32542;
    (t34322, t34323, t34331, t34338, t34339, t34349, t34352)
}
