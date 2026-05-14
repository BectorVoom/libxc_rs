//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 952/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk952<F: Float>(t30843: F, t349: F, t1052: F, t1920: F, t23327: F, t3026: F, t30778: F, t30783: F, t30789: F, t30793: F, t30798: F, t30801: F, t30805: F, t30808: F, t3169: F, t388: F, t6680: F, t6687: F, t6771: F, t6776: F, t6816: F, t8377: F, t8397: F, t8407: F) -> (F, F) {
    let t30844 = t349 * t30843;
    let t30853 = 2.0 * t1052 * t30778 - 0.54831135561607547883e-2 * t23327 * t30783 + 4.0 * t6771 * t6776 + 0.54831135561607547883e-2 * t6687 * t30789 + 4.0 * t1052 * t30793 + t30798 + 0.16449340668482264365e-1 * t1920 * t30801 - 6.0 * t1052 * t30805 + t30808 * t388 + t30844 * t388 - 0.43864908449286038307e-1 * t6680 * t8377 - t3026 * t8407 + 2.0 * t3169 * t8397 - 2.0 * t6771 * t6816;
    (t30844, t30853)
}
