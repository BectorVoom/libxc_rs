//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1322/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1322<F: Float>(t11310: F, t11365: F, t11420: F, t15126: F, t15146: F, t15207: F, t1682: F, t1694: F, t18622: F, t21839: F, t21842: F, t21845: F, t21887: F, t21939: F, t3332: F, t3376: F, t3401: F, t6052: F, t6056: F, t6069: F, t6084: F, t6088: F, t71672: F, t78225: F, t78327: F, t78329: F, t78331: F, t78333: F, t78335: F, t78355: F) -> (F,) {
    let t78944 = -t78327 - t78329 - t78331 - t78333 - t78335 + 0.20779030926817756511e3 * t15126 * t21839 - 0.62337092780453269531e3 * t11365 * t6088 * t6084 - 0.46785788981077169656e1 * t3376 * t21939 * t1694 + 0.69263436422725855036e2 * t3401 * t71672 * t1694 + 0.61524113149298439947e4 * t11310 * t18622 * t6084 + 0.21053605041484726346e2 * t3401 * t6069 * t6084 - 24.0 * t15207 * t21842 + 0.3859675079686208416e3 * t15146 * t21845 - 0.11579025239058625248e4 * t11420 * t6056 * t6052 - 8.0 * t3332 * t21887 * t1682 - 0.19751673498613801407e-1 * t78225 - t78355;
    (t78944,)
}
