//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2049/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2049<F: Float>(t14025: F, t23537: F, t13970: F, t23541: F, t4616: F, t6764: F, t23544: F, t4571: F, t23482: F, t25682: F, t25588: F, t344: F, t6740: F) -> (F, F, F, F, F, F) {
    let t88249 = t23537 * t14025 / F::new(576.0);
    let t88251 = t23541 * t13970 / F::new(1152.0);
    let t88277 = t4616 * t6764;
    let t88281 = t23544 * t4571 / F::new(1728.0);
    let t88286 = t23482 * t25682;
    let t88290 = t6740 * t25588 * t344;
    (t88249, t88251, t88277, t88281, t88286, t88290)
}
