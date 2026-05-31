//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1860/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1860<F: Float>(t4195: F, t5398: F, t4194: F, t1530: F, t17116: F, t1877: F, t20723: F, t20724: F, t20744: F, t20745: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t9724: F) -> (F, F, F) {
    let t20749 = t4195 * t5398;
    let t20751 = F::cast_from(36.0_f64) * t4194 * t20749;
    let t20752 = -F::cast_from(3.0_f64) * t1530 * t17116 * t1877 + t20723 + t20724 + t20744 + t20745 + t20751 - t9457 - t9469 + t9476 + t9484 - t9496 - t9715 + t9724;
    (t20749, t20751, t20752)
}
