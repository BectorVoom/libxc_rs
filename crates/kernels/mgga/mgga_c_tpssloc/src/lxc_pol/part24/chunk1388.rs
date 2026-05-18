//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1388/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1388<F: Float>(t10383: F, t1926: F, t10394: F, t10398: F, t10405: F, t10419: F, t10482: F, t10860: F, t1937: F, t23413: F, t23414: F, t23419: F, t23453: F, t23495: F, t3073: F, t6722: F, t6729: F, t6730: F, t6735: F, t6747: F, t6755: F, t82981: F, t82987: F, t82989: F, t82990: F, t82996: F, t83004: F, t83008: F, t83016: F, t83025: F) -> F {
    let t83028 = F::new(5.0) / F::new(1296.0) * t1926 * t10383;
    let t83029 = F::new(0.30279567070605293142e-3) * t82981 * t6747 + F::new(0.60559134141210586284e-3) * t82987 * t82989 * t82990 * t10482 + F::new(0.30279567070605293142e-3) * t82996 + t6755 * t10860 / F::new(1536.0) - F::new(0.30279567070605293142e-3) * t23414 * t6735 - F::new(0.30279567070605293142e-3) * t6730 * t23495 + t83004 / F::new(576.0) - t23419 * t10419 / F::new(384.0) + t83008 * t3073 / F::new(384.0) + t23419 * t10394 / F::new(768.0) + t23419 * t10398 / F::new(768.0) + t83016 * t10405 / F::new(384.0) - F::new(0.24223653656484234513e-2) * t6722 * t23413 * t1937 + F::new(0.21801288290835811062e-1) * t23453 * t6729 * t1937 + t83025 / F::new(54.0) + t83028;
    t83029
}
