//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1253/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1253<F: Float>(t10948: F, t23540: F, t10472: F, t10478: F, t6753: F, t10961: F, t6754: F, t3077: F, t6764: F, t1937: F, t607: F, t6722: F, t10375: F, t1942: F, t1025: F, t10346: F, t1046: F, t10485: F, t10879: F, t10886: F, t10972: F, t10998: F, t1935: F, t3043: F, t3134: F, t343: F, t6717: F, t6734: F, t6765: F, t83034: F, t83038: F, t83041: F, t83043: F, t83046: F, t83054: F, t83058: F) -> (F,) {
    let t83061 = t10948 * t23540;
    let t83065 = t10472 * t6753 * t10478;
    let t83068 = t10961 * t6754;
    let t83071 = t3077 * t6764;
    let t83075 = t6722 * t607 * t1937;
    let t83080 = t1942 * t10375 / 5184.0;
    let t83081 = -0.30279567070605293142e-3 * t83034 + t6717 * t10998 / 48.0 - t83038 * t1046 / 72.0 + t83041 / 576.0 + t83043 * t3134 / 256.0 - t83046 / 72.0 - 0.10093189023535097714e-3 * t1935 * t10346 * t343 * t6734 + t83054 * t10485 / 256.0 - t83058 * t10879 / 256.0 - t83061 * t3043 / 512.0 + t83065 * t10886 / 1536.0 + t83068 * t1025 / 512.0 + t83071 * t1046 / 768.0 - 0.48447307312968469026e-2 * t83075 + 5.0 / 2592.0 * t6765 * t10972 + t83080;
    (t83081,)
}
