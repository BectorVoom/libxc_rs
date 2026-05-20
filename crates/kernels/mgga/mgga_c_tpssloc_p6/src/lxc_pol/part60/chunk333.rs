//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 333/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk333<F: Float>(t5: F, t1983: F, t2020: F, t1401: F, t1873: F, t63: F, t67: F, t1864: F, t1860: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t2021 = t1983 * t2020;
    let t2028 = F::new(0.135e2) * t1401 * t1873;
    let t2031 = t63 * t67;
    let t2032 = t2031 * t1864;
    let t2035 = piecewise3::<F>(t8, F::new(0.0), t1860 * t2032 / F::new(3.0));
    (t2021, t2028, t2031, t2032, t2035)
}
