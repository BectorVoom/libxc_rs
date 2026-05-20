//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 791/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk791<F: Float>(t40: F, t52: F, t761: F, t9494: F, t607: F, t75: F, t2250: F, t634: F, t767: F, t9258: F, t9288: F, t78: F, t638: F, t771: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t9496 = F::cast_from(0.10254018858216406658e4_f64) * t761 * t9494;
    let t9499 = t75 * t607;
    let t9505 = piecewise3::<F>(t146, F::new(0.0), F::new(8.0) / F::new(27.0) * t634 * t9288 - F::new(2.0) / F::new(3.0) * t9499 * t2250 + F::new(2.0) / F::new(3.0) * t767 * t9258);
    let t9508 = t78 * t607;
    let t9514 = piecewise3::<F>(t150, F::new(0.0), -F::new(8.0) / F::new(27.0) * t638 * t9288 - F::new(2.0) / F::new(3.0) * t9508 * t2250 - F::new(2.0) / F::new(3.0) * t771 * t9258);
    (t9496, t9499, t9505, t9508, t9514)
}
