//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 960/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk960<F: Float>(t2144: F, t3351: F, t3352: F, t9524: F, t70929: F, t74419: F, t74421: F, t74426: F, t74432: F, t74446: F, t74450: F, t74396: F, t74403: F, t74406: F, t74408: F, t74414: F, t74436: F, t74439: F, t74442: F) -> F {
    let t76997 = t3351 * t3352 * t2144 * t9524;
    let t76998 = F::cast_from(0.38308139903968055457e-4_f64) * t76997;
    let t76999 = F::cast_from(0.99317399751028291929e-5_f64) * t70929;
    let t77004 = F::cast_from(0.3192344991997337955e-4_f64) * t74419;
    let t77005 = F::cast_from(0.85129199786595678799e-5_f64) * t74421;
    let t77006 = F::cast_from(0.2553875993597870364e-4_f64) * t74426;
    let t77007 = F::cast_from(0.85129199786595678799e-5_f64) * t74432;
    let t77011 = F::cast_from(0.5107751987195740728e-4_f64) * t74446;
    let t77012 = F::cast_from(0.5107751987195740728e-4_f64) * t74450;
    let t77013 = -F::cast_from(0.10511583655740820313e-5_f64) * t74396 + t76998 - t76999 + F::cast_from(0.35038612185802734376e-6_f64) * t74403 - F::cast_from(0.35038612185802734376e-6_f64) * t74406 - F::cast_from(0.58171619854173713846e-5_f64) * t74408 - F::cast_from(0.58171619854173713846e-5_f64) * t74414 + t77004 + t77005 + t77006 + t77007 - F::cast_from(0.35038612185802734376e-6_f64) * t74436 - F::cast_from(0.35038612185802734376e-6_f64) * t74439 - F::cast_from(0.8759653046450683594e-6_f64) * t74442 + t77011 - t77012;
    t77013
}
