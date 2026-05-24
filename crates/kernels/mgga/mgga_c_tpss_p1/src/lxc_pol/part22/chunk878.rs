//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 878/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk878<F: Float>(t27: F, t7666: F, t1965: F, t571: F, t1971: F, t567: F, t1970: F, t3: F, t25: F, t1974: F, t577: F, t1980: F, t574: F) -> (F, F, F, F, F, F) {
    let t7668 = F::new(120.0) * t7666 * t27;
    let t7669 = t1965 * t571;
    let t7671 = t567 * t1971;
    let t7673 = t1970 * t3;
    let t7674 = F::new(1.0) / t7673;
    let t7676 = F::new(336.0) * t25 * t7674;
    let t7679 = t1974 * t577;
    let t7682 = t574 * t1980;
    (t7668, t7669, t7671, t7676, t7679, t7682)
}
