//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2056/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2056<F: Float>(t2691: F, t812: F, t815: F, t10024: F, t809: F, t238: F, t244: F, t248: F, t40445: F, t9525: F, t9577: F, t116: F) -> (F, F, F, F, F) {
    let t41115 = t812 * t815 * t2691;
    let t41130 = t809 * t10024;
    let t41139 = F::new(13685.0) / F::new(31104.0) * t238 * t40445 * t244 * t248;
    let t41144 = t9577 * t9525;
    let t41146 = t244 * t116;
    (t41115, t41130, t41139, t41144, t41146)
}
