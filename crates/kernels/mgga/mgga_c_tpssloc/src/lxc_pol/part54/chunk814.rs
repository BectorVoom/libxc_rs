//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 814/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk814<F: Float>(t5: F, t3941: F, t7769: F, t1401: F, t7467: F, t2031: F, t7445: F, t1860: F, t2032: F, t7026: F, t7034: F, t7428: F, t7432: F, t7435: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7771 = F::new(27.0) * t3941 * t7769;
    let t7773 = F::new(0.135e2) * t1401 * t7467;
    let t7782 = t2031 * t7445;
    let t7786 = piecewise3::<F>(t8, F::new(0.0), t7428 * t2032 / F::new(3.0) - F::new(5.0) / F::new(3.0) * t7026 * t7432 - F::new(2.0) / F::new(3.0) * t7435 * t2032 - t7034 + t1860 * t7782 / F::new(3.0));
    (t7771, t7773, t7782, t7786)
}
