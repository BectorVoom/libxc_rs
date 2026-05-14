//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1295/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1295<F: Float>(t104231: F, t104355: F, t104364: F, t104367: F, t104369: F, t104371: F, t104375: F, t2121: F, t2134: F, t2136: F, t2139: F, t21745: F, t22038: F, t22133: F, t22173: F, t27599: F, t28525: F, t29563: F, t29615: F, t460: F, t471: F, t488: F, t4899: F, t6221: F, t7310: F, t7320: F, t8027: F, t8028: F, t8031: F, t8035: F, t8040: F) -> (F,) {
    let t109661 = 0.24223653656484234513e-2 * t8028 * t29615 - 0.10093189023535097714e-3 * t2134 * t22038 * t460 * t7320 + t2121 * t4899 * t21745 / 72.0 - t7310 * t22133 / 48.0 - 0.21801288290835811062e-1 * t29563 * t8035 + 0.30279567070605293142e-3 * t8031 * t29615 + 0.24223653656484234513e-2 * t8027 * t28525 * t2136 + 0.30279567070605293142e-3 * t104355 - 0.60559134141210586284e-3 * t104364 - 0.30279567070605293142e-3 * t104367 - t104369 / 1152.0 - t104371 / 576.0 - t104375 / 576.0 - 209.0 / 1296.0 * t471 * t2139 * t22173 * t488 + 0.48447307312968469026e-2 * t104231 * t8040 - t27599 * t6221 / 96.0;
    (t109661,)
}
