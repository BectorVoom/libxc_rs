//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 680/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk680<F: Float>(t776: F, t857: F, t865: F, t23270: F, t22986: F, t225: F, t6625: F, t6576: F, t10049: F, t1912: F, t23236: F, t23239: F, t23243: F, t23250: F, t23252: F, t23254: F, t23259: F, t23262: F, t23266: F, t2597: F, t2720: F, t2743: F, t6627: F, t6663: F, t866: F, t9590: F, t9593: F) -> (F, F, F, F, F) {
    let t23272 = t857 * t776 * t865;
    let t23273 = t23270 * t23272;
    let t23274 = t22986 * t23273;
    let t23278 = t6625 * t225;
    let t23281 = t6576 * t225;
    let t23284 = t23236 - 0.3289868133696452873e-1 * t23239 + 0.49348022005446793095e-1 * t23243 - t9590 * t1912 - 2.0 * t2597 * t6663 - t6627 * t2743 - t23250 + t23252 - 0.82246703342411321824e-2 * t23254 + 0.82246703342411321825e-2 * t23259 + t23262 - 2.0 * t9593 * t1912 - 0.16449340668482264365e-1 * t23266 - t10049 * t1912 + 0.3289868133696452873e-1 * t23274 + 2.0 * t6627 * t2720 - 2.0 * t23278 * t866 - 2.0 * t23281 * t866;
    (t23272, t23274, t23278, t23281, t23284)
}
