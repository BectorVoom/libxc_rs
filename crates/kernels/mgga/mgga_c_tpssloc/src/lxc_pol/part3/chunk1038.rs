//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1038/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1038<F: Float>(t11215: F, t11217: F, t14722: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F, t15083: F, t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F) -> (F, F, F) {
    let t15091 = -0.13892666666666666667e0 * t11215 - 0.69463333333333333333e-1 * t11217 + 0.11577222222222222222e0 * t14766 + t15083 - 0.68863333333333333334e0 * t14738 - 0.34431666666666666667e0 * t14742 - 0.20659e1 * t14733 + 0.20659e1 * t14751 + 0.103295e1 * t14755 + 0.309885e1 * t14746 - 0.68863333333333333333e0 * t14722;
    let t15094 = 0.27785333333333333334e0 * t14781;
    let t15115 = -0.3529725e1 * t14809 - 0.17648625e1 * t14811 + 0.6311625e0 * t14814 + 0.31558125e0 * t14816 + 0.46308888888888888889e-1 * t14818 + 0.45908888888888888888e0 * t11137 + 0.11477222222222222222e0 * t11139 - 0.34431666666666666666e0 * t11141 - 0.17215833333333333333e0 * t11143 + 0.6311625e0 * t14824 + 0.57386111111111111112e0 * t14728;
    (t15091, t15094, t15115)
}
