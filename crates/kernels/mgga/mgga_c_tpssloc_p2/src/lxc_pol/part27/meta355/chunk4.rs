//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1473/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1473<F: Float>(t13176: F, t816: F, t1512: F, t9671: F, t1484: F, t2379: F, t820: F, t9607: F, t2697: F, t4257: F, t4119: F, t776: F) -> (F, F, F, F, F, F) {
    let t13177 = t13176 * t816;
    let t13182 = t9671 * t1512;
    let t13184 = t1484 * t2379;
    let t13186 = t9607 * t820 * t13184;
    let t13190 = F::new(35.0) / F::new(576.0) * t2697 * t4257;
    let t13191 = t4119 * t776;
    (t13177, t13182, t13184, t13186, t13190, t13191)
}
