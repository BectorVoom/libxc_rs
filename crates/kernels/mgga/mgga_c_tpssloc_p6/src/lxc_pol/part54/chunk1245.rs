//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1245/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1245<F: Float>(t2039: F, t8103: F, t2096: F, t33605: F, t33611: F, t33615: F, t33619: F, t33622: F, t33624: F, t33746: F, t4028: F, t652: F, t7266: F, t7458: F, t7802: F, t7806: F, t7904: F, t7941: F, t8690: F, t8835: F) -> (F, F) {
    let t34170 = t8103 * t2039;
    let t34173 = t2096 * t33746 - F::cast_from(2.0_f64) * t34170 * t652 - F::cast_from(2.0_f64) * t4028 * t8835 - F::cast_from(2.0_f64) * t7266 * t7802 - F::cast_from(2.0_f64) * t7266 * t7806 - F::cast_from(2.0_f64) * t7458 * t8835 + F::cast_from(3.0_f64) * t7904 * t8690 + t7941 * t8690 + t33605 - t33611 + t33615 - t33619 - t33622 - t33624;
    (t34170, t34173)
}
