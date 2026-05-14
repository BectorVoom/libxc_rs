//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 908/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk908<F: Float>(t1510: F, t17027: F, t20723: F, t20724: F, t20744: F, t20745: F, t20751: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t20760: F, t20761: F, t20765: F, t20766: F, t20768: F, t9724: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F) -> (F, F, F) {
    let t20806 = t17027 * t1510;
    let t20811 = t20723 - t9457 + t20724 - t9469 + t20744 + t20745 + t9476 + t9484 - t9496 + t20751 - t9715;
    let t20812 = t9724 + t9863 + t9780 - t20760 + t20761 + t20765 + t20766 + t20768 - t9789 + t9793 + t9797;
    (t20806, t20811, t20812)
}
