//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2143/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2143<F: Float>(t1516: F, t81766: F, t23127: F, t4261: F, t13347: F, t6621: F, t131: F, t6598: F, t9537: F, t225: F, t2627: F, t236: F, t25093: F) -> (F, F, F, F, F, F) {
    let t87222 = t81766 * t1516;
    let t87224 = t23127 * t4261;
    let t87226 = t6621 * t13347;
    let t87229 = t6598 * t131 * t9537;
    let t87230 = t225 * t2627;
    let t87233 = t87229 * t87230 * t236 * t25093;
    (t87222, t87224, t87226, t87229, t87230, t87233)
}
