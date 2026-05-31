//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2320/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2320<F: Float>(t1530: F, t16662: F, t17109: F, t1877: F, t1915: F, t23290: F, t23295: F, t2522: F, t25358: F, t25374: F, t28448: F, t28732: F, t4119: F, t4303: F, t4314: F, t46341: F, t5527: F, t5660: F, t5664: F, t6666: F, t6670: F, t67123: F, t67164: F, t7541: F, t776: F, t81539: F, t868: F, t86836: F, t87975: F, t98030: F, t98054: F, t98102: F) -> F {
    let t100623 = F::cast_from(4.0_f64) * t1877 * t87975 * t25374 - F::cast_from(6.0_f64) * t2522 * t6670 * t67164 + F::cast_from(2.0_f64) * t1877 * t23295 * t98102 - t1877 * t98054 * t868 + F::cast_from(6.0_f64) * t46341 * t28732 + F::cast_from(4.0_f64) * t1877 * t23295 * t98030 - t1877 * t6670 * t17109 + F::cast_from(2.0_f64) * t1877 * t81539 * t5664 + F::cast_from(6.0_f64) * t2522 * t7541 * t4119 - F::cast_from(3.0_f64) * t2522 * t6670 * t67123 + F::cast_from(3.0_f64) * t2522 * t1915 * t16662 - F::cast_from(2.0_f64) * t1877 * t86836 * t1530 - F::cast_from(2.0_f64) * t1877 * t25358 * t4303 + F::cast_from(6.0_f64) * t4314 * t6666 * t5527 - t1877 * t23290 * t5660 + F::cast_from(3.0_f64) * t2522 * t28448 * t776;
    t100623
}
