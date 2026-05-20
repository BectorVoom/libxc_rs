//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 970/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk970<F: Float>(t1510: F, t17027: F, t20723: F, t20724: F, t20744: F, t20745: F, t20751: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F) -> (F, F) {
    let t20806 = t17027 * t1510;
    let t20811 = t20723 - t9457 + t20724 - t9469 + t20744 + t20745 + t9476 + t9484 - t9496 + t20751 - t9715;
    (t20806, t20811)
}
