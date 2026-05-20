//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 647/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk647<F: Float>(t5: F, t8512: F, t8515: F, t112: F, t1874: F, t7042: F, t1873: F, t89: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t8518 = piecewise3::<F>(t8, F::new(0.0), -F::new(5.0) / F::new(72.0) * t8512 * t8515);
    let t8519 = t8518 * t112;
    let t8522 = F::new(2.0) * t7042 * t1874;
    let t8526 = t89 * t1873;
    (t8518, t8519, t8522, t8526)
}
