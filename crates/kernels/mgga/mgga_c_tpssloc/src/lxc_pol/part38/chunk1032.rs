//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1032/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1032<F: Float>(t10300: F, t10556: F, t10558: F, t10560: F, t10562: F, t10784: F, t10785: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13552: F, t13557: F, t13561: F, t13616: F, t13624: F, t13626: F, t14287: F, t14291: F, t14304: F, t14326: F) -> (F,) {
    let t14328 = -0.69463333333333333334e-1 * t13530 - 0.34731666666666666667e-1 * t13534 - 0.46308888888888888889e-1 * t13539 + 0.41678e0 * t13544 + 0.20839e0 * t13548 - t14287 + 0.46308888888888888889e-1 * t13552 + 0.20839e0 * t13557 - 0.62517e0 * t13561 + t14291 + t14304 - t10784 - t10785 + 0.6311625e0 * t13616 - 0.13892666666666666667e0 * t10300 - 0.45908888888888888888e0 * t10556 + 0.11477222222222222222e0 * t10558 - 0.34431666666666666666e0 * t10560 + 0.17215833333333333333e0 * t10562 + 0.6311625e0 * t13624 + 0.31558125e0 * t13626 + t14326;
    (t14328,)
}
