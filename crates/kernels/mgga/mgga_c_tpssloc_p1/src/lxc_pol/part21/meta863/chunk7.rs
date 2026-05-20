//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3148/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3148<F: Float>(t19256: F, t225: F, t11606: F, t11613: F, t1190: F, t1238: F, t1252: F, t15787: F, t15794: F, t15820: F, t1761: F, t19120: F, t19214: F, t19226: F, t19232: F, t3487: F, t3593: F, t3598: F, t3599: F, t3600: F, t3630: F, t491: F, t4945: F, t498: F, t5055: F, t5089: F, t51937: F, t52386: F, t6243: F, t6244: F, t6267: F, t65165: F) -> F {
    let t65203 = t19256 * t225;
    let t65206 = -F::new(2.0) * t52386 * t1761 + t65165 * t491 * t498 - F::new(12.0) * t3487 * t19226 - F::new(6.0) * t1238 * t11606 * t6243 * t3630 - F::new(12.0) * t3593 * t19226 - F::new(2.0) * t4945 * t15787 + F::new(2.0) * t19232 * t3600 + F::new(2.0) * t1190 * t19120 * t498 + F::new(8.0) * t3487 * t19214 + F::new(4.0) * t11613 * t6244 - F::new(12.0) * t5055 * t15794 - F::new(4.0) * t15820 * t5089 - F::new(2.0) * t51937 * t1761 - F::new(6.0) * t1238 * t11606 * t6267 * t3599 + F::new(2.0) * t1238 * t3598 * t6267 * t3630 + F::new(8.0) * t3593 * t19214 - F::new(4.0) * t65203 * t1252;
    t65206
}
