//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 961/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk961<F: Float>(t38397: F, t8571: F, t1981: F, t629: F, t676: F, t8512: F, t1971: F, t495: F, t515: F, t8517: F, t9843: F, t42054: F) -> (F, F, F, F) {
    let t45938 = t8571 * t38397;
    let t45942 = t8512 * t1981 * t676 * t629;
    let t45947 = t8517 * t1971 * t515 * t9843 * t495;
    let t45949 = t8571 * t42054;
    (t45938, t45942, t45947, t45949)
}
