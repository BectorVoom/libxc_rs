//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 912/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk912<F: Float>(t52: F, t20217: F, t20234: F, t4087: F, t5398: F, t76: F, t9438: F, t20732: F, t157: F, t182: F, t16587: F, t4195: F, t4194: F, t1530: F, t17116: F, t1877: F, t20723: F, t20724: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t9724: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t150 = t52 <= zeta_threshold;
    let t20740 = piecewise3(t150, 0.0, 8.0 / 27.0 * t9438 * t20234 + 4.0 / 3.0 * t4087 * t5398 - 4.0 / 3.0 * t76 * t20217);
    let t20741 = t20732 + t20740;
    let t20742 = t20741 * t157;
    let t20744 = 0.19751673498613801407e-1 * t20742 * t182;
    let t20745 = 36.0 * t16587;
    let t20749 = t4195 * t5398;
    let t20751 = 36.0 * t4194 * t20749;
    let t20752 = -3.0 * t1530 * t17116 * t1877 + t20723 + t20724 + t20744 + t20745 + t20751 - t9457 - t9469 + t9476 + t9484 - t9496 - t9715 + t9724;
    (t20741, t20742, t20744, t20745, t20749, t20751, t20752)
}
