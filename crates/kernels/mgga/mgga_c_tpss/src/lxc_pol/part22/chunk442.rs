//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 442/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk442<F: Float>(t30: F, t33: F, t1165: F, t1322: F, t1338: F, t1288: F, t490: F, t1497: F, t493: F, t162: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1604 = F::new(2.0) * t1165 * t1338 + t1322;
    let t1608 = piecewise3::<F>(t31, F::new(0.0), F::new(4.0) / F::new(3.0) * t490 * t1288);
    let t1611 = piecewise3::<F>(t34, F::new(0.0), F::new(4.0) / F::new(3.0) * t493 * t1497);
    let t1613 = (t1608 + t1611) * t162;
    (t1604, t1613)
}
