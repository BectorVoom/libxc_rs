//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1118/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1118<F: Float>(t5: F, t7973: F, t8307: F, t8513: F, t32579: F, t32583: F, t32590: F, t33103: F, t33107: F, t33111: F, t33119: F, t8663: F, t8856: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t34221 = t8307 * t7973;
    let t34222 = t8513 * t34221;
    let t34228 = piecewise3::<F>(t8, F::new(0.0), F::new(5.0) / F::new(144.0) * t33103 * t8856 - F::new(5.0) / F::new(24.0) * t32579 * t33107 - F::new(5.0) / F::new(36.0) * t32583 * t33111 + F::new(5.0) / F::new(72.0) * t8663 * t34222 + F::new(5.0) / F::new(72.0) * t32590 * t33119);
    (t34221, t34222, t34228)
}
