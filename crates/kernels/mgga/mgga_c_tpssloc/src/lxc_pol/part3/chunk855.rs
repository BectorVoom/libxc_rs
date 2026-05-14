//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 855/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk855<F: Float>(t10955: F, t364: F, t354: F, t1009: F, t3020: F, t1011: F, t1019: F, t1040: F, t3077: F, t2775: F, t283: F, t135: F, t3142: F, t973: F, t3147: F, t3152: F) -> (F, F, F, F, F, F, F, F) {
    let t10956 = t364 * t10955;
    let t10957 = t354 * t10956;
    let t10960 = t3020 * t1009;
    let t10961 = t10960 * t1011;
    let t10962 = t10961 * t1019;
    let t10965 = t3077 * t1040;
    let t10969 = 1.0 / t283 / t2775;
    let t10981 = t135 * t3142;
    let t10982 = t973 * t10981;
    let t10984 = t135 * t3147;
    let t10985 = t973 * t10984;
    let t10993 = t135 * t3152;
    (t10957, t10960, t10962, t10965, t10969, t10982, t10985, t10993)
}
