//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1083/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1083<F: Float>(t8447: F, t8577: F, t8368: F, t8533: F, t10115: F, t275: F, t1743: F, t1971: F, t495: F, t511: F, t7230: F, t34847: F, t9990: F) -> (F, F, F, F, F) {
    let t47757 = t8577 * t8447;
    let t47759 = t8368 * t8533;
    let t47761 = t275 * t10115;
    let t47765 = t7230 * t1971 * t511 * t1743 * t495;
    let t47767 = t34847 * t9990;
    (t47757, t47759, t47761, t47765, t47767)
}
