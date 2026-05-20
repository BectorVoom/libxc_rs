//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2447/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2447<F: Float>(t1041: F, t4584: F, t49850: F, t10422: F, t14032: F, t3070: F, t13969: F, t14166: F, t1023: F, t10390: F, t10483: F, t13611: F, t13762: F, t14012: F, t14189: F, t1539: F, t2960: F, t2979: F, t3048: F, t3071: F, t42388: F, t43143: F, t43155: F, t43157: F, t43161: F, t47726: F, t973: F) -> F {
    let t50047 = t1041 * t49850 * t4584;
    let t50048 = t50047 / F::new(3456.0);
    let t50056 = t3070 * t10422 * t14032;
    let t50062 = t1041 * t13969 * t14166;
    let t50066 = t973 * t2979 * t47726 / F::new(6.0) - F::new(2.0) / F::new(27.0) * t2960 * t14012 + t42388 * t3071 * t1539 * t10483 / F::new(768.0) - t43143 / F::new(216.0) + t50048 + t10390 * t13762 / F::new(768.0) + t3070 * t3071 * t13611 * t1023 / F::new(1536.0) + t50056 / F::new(2304.0) - F::new(11.0) / F::new(162.0) * t43155 - F::new(5.0) / F::new(162.0) * t43157 - t43161 / F::new(4608.0) + t50062 / F::new(384.0) - F::new(5.0) / F::new(324.0) * t3048 * t14189;
    t50066
}
