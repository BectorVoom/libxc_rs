//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1200/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1200<F: Float>(t6703: F, t7593: F, t1920: F, t32923: F, t968: F, t1945: F, t7577: F, t10165: F, t1052: F, t113217: F, t113231: F, t113240: F, t14529: F, t14552: F, t1635: F, t1927: F, t23327: F, t23346: F, t23369: F, t25442: F, t25755: F, t3026: F, t30793: F, t30904: F, t32913: F, t32976: F, t32998: F, t4557: F, t4693: F, t6687: F, t6691: F, t6706: F, t6816: F, t7625: F, t8396: F, t8397: F, t8407: F, t986: F) -> F {
    let t119076 = t6703 * t7593;
    let t119086 = t1920 * t968 * t32923;
    let t119088 = t7577 * t1945;
    let t119107 = -F::new(0.54831135561607547883e-2) * t113217 - t14529 * t8407 - F::new(2.0) * t25755 * t6816 - F::new(0.16449340668482264365e-1) * t6687 * t986 * t32923 + F::new(2.0) * t14552 * t8397 - F::new(0.16449340668482264365e-1) * t6687 * t119076 * t6706 - F::new(0.18277045187202515961e-2) * t113240 + F::new(0.43864908449286038307e-1) * t23346 * t32998 + F::new(2.0) * t3026 * t32913 + F::new(0.54831135561607547883e-2) * t119086 - F::new(0.54831135561607547883e-2) * t23327 * t119088 * t6691 - t14552 * t8407 + F::new(4.0) * t4557 * t30793 - F::new(2.0) * t23369 * t7625 - t113231 * t1635 - F::new(6.0) * t1052 * t10165 * t8396 * t4693 + F::new(0.43864908449286038307e-1) * t23346 * t32976 - F::new(0.3289868133696452873e-1) * t1927 * t25442 * t30904;
    t119107
}
