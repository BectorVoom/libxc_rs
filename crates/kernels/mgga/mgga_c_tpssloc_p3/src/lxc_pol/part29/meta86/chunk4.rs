//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 563/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk563<F: Float>(t25: F, t28: F, t1268: F, t1442: F, t1458: F, t1408: F, t514: F, t1649: F, t517: F, t157: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t1778 = F::new(2.0) * t1268 * t1458 + t1442;
    let t1782 = piecewise3::<F>(t26, F::new(0.0), F::new(4.0) / F::new(3.0) * t514 * t1408);
    let t1785 = piecewise3::<F>(t29, F::new(0.0), F::new(4.0) / F::new(3.0) * t517 * t1649);
    let t1787 = (t1782 + t1785) * t157;
    (t1778, t1787)
}
