//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1082/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1082<F: Float>(t5: F, t32578: F, t9239: F, t33: F, t8854: F, t2240: F, t7254: F, t8307: F, t8513: F, t31000: F, t31006: F, t31013: F, t31024: F, t8663: F, t8856: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t32579 = t9239 * t32578;
    let t32582 = t33 * t8854;
    let t32583 = t2240 * t32582;
    let t32587 = t8513 * t8307 * t7254;
    let t32590 = t2240 * t32578;
    let t32594 = piecewise3::<F>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t31000 * t8856 - F::new(5.0) / F::new(24.0) * t32579 * t31006 - F::new(5.0) / F::new(36.0) * t32583 * t31013 + F::new(5.0) / F::new(72.0) * t8663 * t32587 + F::new(5.0) / F::new(72.0) * t32590 * t31024);
    (t32579, t32582, t32583, t32587, t32590, t32594)
}
