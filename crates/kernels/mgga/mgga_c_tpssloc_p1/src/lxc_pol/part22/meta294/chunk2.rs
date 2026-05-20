//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1455/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1455<F: Float>(t13779: F, t4343: F, t2986: F, t134: F, t2978: F, t344: F) -> (F, F, F) {
    let t13780 = t13779 * t4343;
    let t13782 = F::cast_from(0.37037037037037037036e-3_f64) * t2986 * t13780;
    let t13783 = t134 * t2978;
    let t13784 = t13783 * t344;
    (t13782, t13783, t13784)
}
