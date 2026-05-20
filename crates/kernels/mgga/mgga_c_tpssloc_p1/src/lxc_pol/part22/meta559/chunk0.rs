//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2063/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2063<F: Float>(t2289: F, t2769: F, t41654: F, t10629: F, t938: F, t2903: F, t2928: F, t315: F, t909: F, t9709: F, t10213: F, t241: F) -> (F, F, F, F, F, F, F) {
    let t41687 = F::new(1.0) / t2769 / t2289;
    let t41741 = F::cast_from(0.96141975308641975307e-1_f64) * t41654;
    let t41821 = t938 * t10629;
    let t41825 = F::new(1.0) / t2928 / t2903;
    let t41826 = t315 * t41825;
    let t41863 = t9709 * t909;
    let t41880 = t241 * t10213;
    (t41687, t41741, t41821, t41825, t41826, t41863, t41880)
}
