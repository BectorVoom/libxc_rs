//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 453/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk453<F: Float>(t30: F, t33: F, t1288: F, t490: F, t1497: F, t493: F, t162: F, zeta_threshold: F) -> (F,) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1608 = piecewise3(t31, 0.0, 4.0 / 3.0 * t490 * t1288);
    let t1611 = piecewise3(t34, 0.0, 4.0 / 3.0 * t493 * t1497);
    let t1613 = (t1608 + t1611) * t162;
    (t1613,)
}
