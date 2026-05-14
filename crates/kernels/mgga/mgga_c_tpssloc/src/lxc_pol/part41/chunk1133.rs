//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1133/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1133<F: Float>(t19805: F, t554: F, t12211: F, t6371: F, t3726: F, t6375: F, t119: F, t19631: F, t210: F, t12385: F, t6390: F, t16288: F, t1827: F, t1340: F, t19815: F, t12215: F, t1315: F, t1354: F, t16147: F, t16159: F, t16211: F, t16214: F, t16278: F, t16394: F, t19823: F, t19827: F, t19831: F, t19834: F, t3733: F, t5235: F, t5289: F, t5293: F, t5303: F, t559: F) -> (F,) {
    let t19836 = t19805 * t554;
    let t19839 = t12211 * t6371;
    let t19841 = t3726 * t6375;
    let t19843 = t119 * t19631;
    let t19844 = t210 * t19843;
    let t19851 = t12385 * t6390;
    let t19853 = t16288 * t1827;
    let t19855 = t19815 * t1340;
    let t19862 = -t16147 + t16159 - 119.0 / 6912.0 * t16211 + t16214 - t12215 * t19823 / 4.0 + t3733 * t19827 / 8.0 + t3733 * t19831 / 16.0 - 7.0 / 4608.0 * t19834 + t19836 * t559 / 3072.0 - 7.0 / 48.0 * t19839 + 7.0 / 144.0 * t19841 - t1315 * t19844 / 48.0 - t16394 * t5293 / 1536.0 + t16394 * t5303 / 384.0 - 7.0 / 2304.0 * t19851 + 7.0 / 2304.0 * t19853 - t19855 * t1354 / 3072.0 - t16278 * t1827 / 1536.0 - t5235 * t5289 / 1536.0;
    (t19862,)
}
