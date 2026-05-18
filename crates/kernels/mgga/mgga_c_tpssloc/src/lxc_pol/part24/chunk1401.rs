//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1401/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1401<F: Float>(t225: F, t23410: F, t6692: F, t82632: F, t6707: F, t82573: F, t6695: F, t3166: F, t6703: F, t1049: F, t6733: F, t10160: F, t10165: F, t10181: F, t10316: F, t10327: F, t1052: F, t1055: F, t1066: F, t1922: F, t23310: F, t23327: F, t23332: F, t23346: F, t23722: F, t25757: F, t25758: F, t3169: F, t3175: F, t6687: F, t6689: F, t6690: F, t6691: F, t6706: F, t6776: F, t6815: F, t82499: F, t82502: F, t82561: F, t82603: F, t82660: F, t82705: F, t82749: F, t82795: F, t82834: F, t83270: F) -> F {
    let t83276 = t23410 * t225;
    let t83281 = t82632 * t6692;
    let t83285 = t82573 * t6707;
    let t83287 = t82573 * t6695;
    let t83296 = t6703 * t3166;
    let t83303 = t6733 * t1049;
    let t83307 = F::new(0.16449340668482264365e-1) * t6687 * t6689 * t6690 * t10316 + F::new(0.13159472534785811492e0) * t23346 * t23310 - F::new(3.0) * t82499 * t1066 + F::new(0.16449340668482264365e-1) * t23327 * t82502 * t23332 - t1052 * t1055 * (t82561 + t82603 + t82660 + t82705 + t82749 + t82795 + t82834 + t83270) - F::new(6.0) * t83276 * t1066 + F::new(12.0) * t10160 * t6776 - F::new(0.18277045187202515961e-2) * t83281 - F::new(3.0) * t3169 * t23722 + F::new(0.43864908449286038307e-1) * t83285 + F::new(0.43864908449286038307e-1) * t83287 - F::new(0.82246703342411321825e-2) * t6687 * t10327 * t1922 - F::new(18.0) * t1052 * t10165 * t6815 * t3175 - F::new(0.24674011002723396548e-1) * t6687 * t83296 * t6706 - F::new(18.0) * t25757 * t25758 * t10181 - F::new(0.16449340668482264365e-1) * t23327 * t83303 * t6691;
    t83307
}
