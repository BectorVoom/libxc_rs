//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 899/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk899<F: Float>(t5: F, t31019: F, t31672: F, t31675: F, t31677: F, t31681: F, t31684: F, t31690: F, t31693: F, t8512: F, t8515: F, t112: F, t1873: F, t23938: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t31699 = piecewise3::<F>(t8, F::new(0.0), -F::new(5.0) / F::new(72.0) * t31672 * t8515 + F::new(5.0) / F::new(12.0) * t31675 * t31677 + F::new(5.0) / F::new(18.0) * t31681 * t31684 + t31690 - F::new(5.0) / F::new(36.0) * t8512 * t31693 - F::new(5.0) / F::new(72.0) * t8512 * t31019);
    let t31700 = t31699 * t112;
    let t31704 = F::new(2.0) * t23938 * t1873;
    (t31699, t31700, t31704)
}
