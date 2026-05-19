//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 995/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk995<F: Float>(t52: F, t5392: F, t638: F, t5398: F, t78: F, t16558: F, t3966: F, t4111: F, t607: F, t771: F, t16648: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t16649 = t638 * t5392;
    let t16654 = t78 * t5398;
    let t16660 = piecewise3::<F>(t150, F::new(0.0), -F::new(8.0) / F::new(27.0) * t16649 * t607 - F::new(4.0) / F::new(9.0) * t4111 * t3966 - F::new(2.0) / F::new(9.0) * t16654 * t607 - F::new(2.0) / F::new(3.0) * t771 * t16558);
    let t16662 = t16648 / F::new(2.0) + t16660 / F::new(2.0);
    t16662
}
