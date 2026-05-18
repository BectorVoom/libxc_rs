//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1048/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1048<F: Float>(t1055: F, t30899: F, t23394: F, t6775: F, t6704: F, t23365: F, t8380: F, t23384: F, t8381: F, t225: F, t8392: F, t1052: F, t1066: F, t1956: F, t23346: F, t23369: F, t23372: F, t3026: F, t30855: F, t30858: F, t30862: F, t30869: F, t3169: F, t6687: F, t8397: F, t8407: F) -> (F, F, F, F, F, F, F) {
    let t30900 = t1055 * t30899;
    let t30904 = t23394 * t6775;
    let t30905 = t6704 * t30904;
    let t30908 = t23365 * t8380;
    let t30912 = F::new(0.54831135561607547883e-2) * t23384 * t8381;
    let t30915 = t8392 * t225;
    let t30919 = -F::new(0.16449340668482264365e-1) * t6687 * t30855 - F::new(0.16449340668482264365e-1) * t6687 * t30858 + F::new(0.16449340668482264365e-1) * t6687 * t30862 + F::new(2.0) * t3026 * t8397 - t3169 * t8407 - F::new(0.16449340668482264365e-1) * t6687 * t30869 - t1052 * t30900 + F::new(0.43864908449286038307e-1) * t23346 * t8381 + F::new(0.3289868133696452873e-1) * t6687 * t30905 - F::new(0.16449340668482264365e-1) * t6687 * t30908 - t30912 - F::new(2.0) * t23372 * t1956 - t30915 * t1066 - F::new(2.0) * t23369 * t1956;
    (t30900, t30904, t30905, t30908, t30912, t30915, t30919)
}
