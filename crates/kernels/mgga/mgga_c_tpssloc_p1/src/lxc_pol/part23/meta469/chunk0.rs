//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1386/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1386<F: Float>(t1021: F, t10403: F, t1041: F, t10480: F, t10883: F, t10970: F, t1409: F, t17712: F, t21405: F, t21532: F, t248: F, t3071: F, t3146: F, t360: F, t42358: F, t4582: F, t48670: F, t48674: F, t49934: F, t50193: F, t5878: F, t61782: F, t62079: F, t62840: F, t70100: F, t70239: F, t70346: F, t70351: F, t70363: F, t70389: F, t70404: F, t75847: F, t76581: F, t76740: F, t973: F, t974: F) -> F {
    let t77539 = t50193 * t21405 / F::new(768.0) - t42358 * t248 * t1021 * t76740 * t360 / F::new(3072.0) - F::new(5.0) / F::new(432.0) * t1041 * t248 * t10970 * t76581 - F::new(5.0) / F::new(864.0) * t70239 + t10403 * t3071 * t62840 * t70100 * t1409 / F::new(192.0) - t61782 / F::new(3456.0) + t973 * t974 * t3146 * t75847 / F::new(72.0) + t10883 * t4582 * t17712 * t5878 / F::new(512.0) - t49934 * t21532 / F::new(384.0) + t70346 / F::new(1152.0) - t70351 / F::new(384.0) + t70363 / F::new(1152.0) + t48670 / F::new(2592.0) + t48674 / F::new(3888.0) + F::new(5.0) / F::new(1944.0) * t70389 + F::new(3.0) / F::new(256.0) * t10480 * t4582 * t17712 * t62079 - t70404 / F::new(288.0);
    t77539
}
