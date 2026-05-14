//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1251/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1251<F: Float>(t23418: F, t3180: F, t10401: F, t23417: F, t3186: F, t3158: F, t6712: F, t10383: F, t1926: F, t10394: F, t10398: F, t10405: F, t10419: F, t10482: F, t10860: F, t1937: F, t23413: F, t23414: F, t23419: F, t23453: F, t23495: F, t3073: F, t6722: F, t6729: F, t6730: F, t6735: F, t6747: F, t6755: F, t82981: F, t82987: F, t82989: F, t82990: F, t82996: F, t83004: F) -> (F, F) {
    let t83008 = t3180 * t23418;
    let t83015 = t23417 * t10401;
    let t83016 = t3186 * t83015;
    let t83025 = t6712 * t3158;
    let t83028 = 5.0 / 1296.0 * t1926 * t10383;
    let t83029 = 0.30279567070605293142e-3 * t82981 * t6747 + 0.60559134141210586284e-3 * t82987 * t82989 * t82990 * t10482 + 0.30279567070605293142e-3 * t82996 + t6755 * t10860 / 1536.0 - 0.30279567070605293142e-3 * t23414 * t6735 - 0.30279567070605293142e-3 * t6730 * t23495 + t83004 / 576.0 - t23419 * t10419 / 384.0 + t83008 * t3073 / 384.0 + t23419 * t10394 / 768.0 + t23419 * t10398 / 768.0 + t83016 * t10405 / 384.0 - 0.24223653656484234513e-2 * t6722 * t23413 * t1937 + 0.21801288290835811062e-1 * t23453 * t6729 * t1937 + t83025 / 54.0 + t83028;
    (t83015, t83029)
}
