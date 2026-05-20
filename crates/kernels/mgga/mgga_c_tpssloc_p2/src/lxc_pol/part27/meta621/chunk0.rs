//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2100/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2100<F: Float>(t1863: F, t83728: F, t1864: F, t2307: F, t22522: F, t9239: F, t9231: F, t2240: F, t22511: F, t33: F, t39049: F, t6489: F) -> (F, F, F, F, F, F) {
    let t83729 = t1863 * t83728;
    let t83737 = t1864 * t2307;
    let t83738 = t1863 * t83737;
    let t83741 = t9239 * t22522;
    let t83750 = t9231 * t22522;
    let t83760 = t2240 * t33 * t22511;
    let t83775 = t39049 * t6489;
    (t83729, t83738, t83741, t83750, t83760, t83775)
}
