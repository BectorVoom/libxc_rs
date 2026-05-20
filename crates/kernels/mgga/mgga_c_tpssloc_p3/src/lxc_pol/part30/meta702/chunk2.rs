//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2275/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2275<F: Float>(t28557: F, t381: F, t3173: F, t5919: F, t1921: F, t28702: F, t82431: F, t1052: F, t1409: F, t1626: F, t1634: F, t17686: F, t23327: F, t23329: F, t23330: F, t23336: F, t23369: F, t254: F, t25429: F, t25731: F, t25759: F, t28475: F, t28499: F, t28713: F, t3169: F, t3174: F, t3966: F, t4693: F, t5944: F, t6680: F, t6687: F, t6691: F, t88035: F, t88758: F, t986: F) -> (F, F) {
    let t99273 = t28557 * t381;
    let t99296 = t3173 * t5919;
    let t99297 = t1921 * t99296;
    let t99301 = t82431 * t28702;
    let t99313 = -F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t99273 * t6691 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t23330 * t3966 * t1634 - F::cast_from(0.21932454224643019154e-1_f64) * t25429 * t23329 * t88035 * t17686 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23336 * t28499 - t23369 * t5944 - F::cast_from(0.21932454224643019153e-1_f64) * t6680 * t28475 + F::new(4.0) * t1052 * t3174 * t25731 * t1634 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t99297 + t88758 - F::cast_from(0.18277045187202515961e-2_f64) * t99301 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t23330 * t1409 * t4693 - F::new(12.0) * t1626 * t254 * t25759 + F::new(2.0) * t3169 * t28713;
    (t99296, t99313)
}
