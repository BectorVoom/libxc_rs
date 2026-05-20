//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1010/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1010<F: Float>(t5: F, t128333: F, t128368: F, t112: F, t33610: F, t7685: F, t28813: F, t8607: F, t27188: F, t7468: F, t33234: F, t28045: F, t7042: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t128370 = piecewise3::<F>(t8, F::new(0.0), t128333 + t128368);
    let t128371 = t128370 * t112;
    let t128375 = F::new(2.0) * t7685 * t33610;
    let t128377 = F::new(2.0) * t8607 * t28813;
    let t128381 = F::new(4.0) * t27188 * t7468;
    let t128383 = F::new(4.0) * t33234 * t7468;
    let t128385 = F::new(4.0) * t7042 * t28045;
    (t128371, t128375, t128377, t128381, t128383, t128385)
}
