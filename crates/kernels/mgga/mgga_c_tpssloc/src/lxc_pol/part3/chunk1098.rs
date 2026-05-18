//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1098/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1098<F: Float>(t13644: F, t13602: F, t13598: F, t13613: F, t13630: F, t13632: F, t13635: F, t13638: F, t13640: F, t13642: F, t13647: F, t10300: F, t10556: F, t10558: F, t10560: F, t10562: F, t10784: F, t10785: F, t13530: F, t13534: F, t13539: F, t13544: F, t13548: F, t13552: F, t13557: F, t13561: F, t13616: F, t13624: F, t13626: F, t14287: F, t14291: F, t14304: F) -> F {
    let t14321 = F::new(0.13892666666666666667e0) * t13644;
    let t14324 = F::new(0.34431666666666666666e0) * t13602;
    let t14326 = -F::new(0.3529725e1) * t13630 - F::new(0.17648625e1) * t13632 + F::new(0.264729375e1) * t13635 - F::new(0.157790625e0) * t13638 + F::new(0.3529725e1) * t13640 - F::new(0.11577222222222222222e0) * t13642 + t14321 - F::new(0.104195e0) * t13647 - F::new(0.22954444444444444444e0) * t13598 + t14324 - F::new(0.516475e0) * t13613;
    let t14328 = -F::new(0.69463333333333333334e-1) * t13530 - F::new(0.34731666666666666667e-1) * t13534 - F::new(0.46308888888888888889e-1) * t13539 + F::new(0.41678e0) * t13544 + F::new(0.20839e0) * t13548 - t14287 + F::new(0.46308888888888888889e-1) * t13552 + F::new(0.20839e0) * t13557 - F::new(0.62517e0) * t13561 + t14291 + t14304 - t10784 - t10785 + F::new(0.6311625e0) * t13616 - F::new(0.13892666666666666667e0) * t10300 - F::new(0.45908888888888888888e0) * t10556 + F::new(0.11477222222222222222e0) * t10558 - F::new(0.34431666666666666666e0) * t10560 + F::new(0.17215833333333333333e0) * t10562 + F::new(0.6311625e0) * t13624 + F::new(0.31558125e0) * t13626 + t14326;
    t14328
}
