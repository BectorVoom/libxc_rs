//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2675/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2675<F: Float>(t12286: F, t12351: F, t1307: F, t1341: F, t1343: F, t1363: F, t1799: F, t19631: F, t19921: F, t19926: F, t20416: F, t20497: F, t20556: F, t20565: F, t3778: F, t3783: F, t3870: F, t5187: F, t5240: F, t56776: F, t56779: F, t56795: F, t56797: F, t6330: F, t6347: F, t74564: F, t820: F) -> F {
    let t74569 = F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t56776 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t56779 - F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t56795 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t56797 + t12286 * t20497 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1363 * t3870 * t820 * t20416 * t1307 - F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t1363 * t12351 * t820 * t6330 * t5187 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3783 * t20565 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t1363 * t3870 * t820 * t5187 * t6347 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t1363 * t3870 * t820 * t1799 * t19631 - F::cast_from(15.0_f64) / F::cast_from(128.0_f64) * t5240 * t19921 + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t5240 * t19926 - t3778 * t20556 / F::cast_from(3072.0_f64) - t1341 * t1343 * t820 * t74564 / F::cast_from(3072.0_f64);
    t74569
}
