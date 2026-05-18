//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1040/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1040<F: Float>(t2533: F, t3806: F, t865: F, t2531: F, t2525: F, t3810: F, t1424: F, t8712: F, t2482: F, t8710: F, t1425: F, t2633: F, t3894: F) -> (F, F, F, F, F) {
    let t11252 = t3806 * t2533;
    let t11253 = t11252 * t865;
    let t11255 = F::new(0.32163958997385070134e2) * t2531 * t11253;
    let t11256 = t3810 * t2525;
    let t11258 = F::new(0.16081979498692535067e2) * t2531 * t11256;
    let t11259 = t1424 * t8712;
    let t11260 = t11259 * t2482;
    let t11262 = F::new(0.51726012919273400301e3) * t8710 * t11260;
    let t11263 = t1425 * t2482;
    let t11265 = F::new(6.0) * t2531 * t11263;
    let t11267 = F::new(0.11696447245269292414e1) * t3894 * t2633;
    (t11255, t11258, t11262, t11265, t11267)
}
