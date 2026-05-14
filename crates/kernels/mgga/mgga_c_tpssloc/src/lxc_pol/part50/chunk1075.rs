//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1075/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1075<F: Float>(t10164: F, t1052: F, t113243: F, t113278: F, t113286: F, t113313: F, t14529: F, t14555: F, t1599: F, t1603: F, t1920: F, t1956: F, t225: F, t23327: F, t25705: F, t25743: F, t25750: F, t25757: F, t3026: F, t30782: F, t30843: F, t30900: F, t3169: F, t3174: F, t32913: F, t32917: F, t345: F, t387: F, t388: F, t4552: F, t4557: F, t4664: F, t4693: F, t6687: F, t6771: F, t8391: F, t8397: F, t8406: F, t8407: F, t88050: F, t88744: F) -> (F,) {
    let t119149 = 2.0 * t14529 * t8397 + 2.0 * t14555 * t8397 + t4552 * t8391 * t388 + t1603 * t30843 * t388 + 2.0 * t3169 * t32913 - 2.0 * t88744 * t1956 + 2.0 * t1052 * t3174 * t8406 * t4693 + 0.10966227112321509577e-1 * t113286 + 0.16449340668482264365e-1 * t1920 * t345 * t25705 * t225 * t387 - t14555 * t8407 - 0.54831135561607547883e-2 * t23327 * t113243 * t25750 - 0.54831135561607547883e-2 * t23327 * t88050 * t30782 + 0.16449340668482264365e-1 * t6687 * t1599 * t113278 + 4.0 * t6771 * t25743 - 6.0 * t25757 * t10164 * t8406 * t4664 - t4557 * t30900 + 4.0 * t3026 * t32917 - t113313;
    (t119149,)
}
