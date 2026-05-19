//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1042/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1042<F: Float>(t115658: F, t117317: F, t122206: F, t122384: F, t122390: F, t122394: F, t122399: F, t122406: F, t122551: F, t122562: F, t124223: F, t124245: F, t124273: F, t1375: F, t1378: F, t16022: F, t24095: F, t26224: F, t26990: F, t3887: F, t5325: F, t5353: F, t7213: F, t7925: F, t7936: F, t8794: F, t8800: F) -> F {
    let t124281 = -F::cast_from(0.6579736267392905746e-1_f64) * t122384 + F::cast_from(0.16449340668482264365e-1_f64) * t122390 + F::new(2.0) * t16022 * t8794 - F::cast_from(0.13159472534785811492e0_f64) * t122394 - F::cast_from(0.19739208802178717238e0_f64) * t122399 + F::new(4.0) * t24095 * t7925 + t117317 - F::cast_from(0.3289868133696452873e-1_f64) * t122406 + F::new(2.0) * t1375 * t3887 * t8800 * t5353 + F::new(4.0) * t1375 * t3887 * t7213 * t7936 - F::new(6.0) * t26224 * t124223 * t5325 - F::cast_from(0.16449340668482264365e-1_f64) * t122551 - t1375 * t1378 * (t124245 + t124273) - F::cast_from(0.16449340668482264365e-1_f64) * t115658 - F::cast_from(0.3289868133696452873e-1_f64) * t122562 - F::new(12.0) * t122206 * t26990;
    t124281
}
