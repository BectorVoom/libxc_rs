//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 566/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk566<F: Float>(t14194: F, t14200: F, t14202: F, t3219: F, t7720: F, t498: F, t698: F, t515: F, t7231: F, t3351: F, t8235: F, t3352: F) -> (F, F, F, F, F, F, F) {
    let t14661 = F::new(0.2553875993597870364e-4) * t14194;
    let t14662 = F::new(0.1702583995731913576e-4) * t14200;
    let t14663 = F::new(0.85129199786595678799e-5) * t14202;
    let t14664 = t7720 * t3219;
    let t14665 = F::new(0.42564599893297839398e-5) * t14664;
    let t14666 = t698 * t498;
    let t14667 = t515 * t14666;
    let t14668 = t7231 * t14667;
    let t14669 = t3351 * t14668;
    let t14670 = F::new(0.42564599893297839398e-5) * t14669;
    let t14671 = t515 * t8235;
    let t14672 = t3352 * t14671;
    (t14661, t14662, t14663, t14665, t14668, t14670, t14672)
}
