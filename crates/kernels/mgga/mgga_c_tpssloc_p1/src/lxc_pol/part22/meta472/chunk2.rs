//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1867/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1867<F: Float>(t185: F, t20217: F, t707: F, t13115: F, t5499: F, t20777: F, t20815: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t9894: F) -> (F, F, F, F) {
    let t20816 = t185 * t20217;
    let t20818 = F::new(4.0) * t707 * t20816;
    let t20820 = F::new(36.0) * t13115 * t5499;
    let t20821 = -t9876 - t9820 - t9824 - t9884 + t9887 + t9890 - t20777 + t20815 + t20818 - t9894 + t20820;
    (t20816, t20818, t20820, t20821)
}
