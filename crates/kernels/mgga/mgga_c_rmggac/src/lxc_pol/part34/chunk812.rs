//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 812/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk812<F: Float>(t2144: F, t3351: F, t3352: F, t9524: F, t70929: F, t74419: F, t74421: F, t74426: F, t74432: F, t74446: F, t74450: F, t74396: F, t74403: F, t74406: F, t74408: F, t74414: F, t74436: F, t74439: F, t74442: F) -> (F,) {
    let t76997 = t3351 * t3352 * t2144 * t9524;
    let t76998 = 0.38308139903968055457e-4 * t76997;
    let t76999 = 0.99317399751028291929e-5 * t70929;
    let t77004 = 0.3192344991997337955e-4 * t74419;
    let t77005 = 0.85129199786595678799e-5 * t74421;
    let t77006 = 0.2553875993597870364e-4 * t74426;
    let t77007 = 0.85129199786595678799e-5 * t74432;
    let t77011 = 0.5107751987195740728e-4 * t74446;
    let t77012 = 0.5107751987195740728e-4 * t74450;
    let t77013 = -0.10511583655740820313e-5 * t74396 + t76998 - t76999 + 0.35038612185802734376e-6 * t74403 - 0.35038612185802734376e-6 * t74406 - 0.58171619854173713846e-5 * t74408 - 0.58171619854173713846e-5 * t74414 + t77004 + t77005 + t77006 + t77007 - 0.35038612185802734376e-6 * t74436 - 0.35038612185802734376e-6 * t74439 - 0.8759653046450683594e-6 * t74442 + t77011 - t77012;
    (t77013,)
}
