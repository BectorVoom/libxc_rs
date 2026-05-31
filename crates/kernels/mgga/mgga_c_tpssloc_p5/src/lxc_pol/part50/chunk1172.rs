//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1172/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1172<F: Float>(t4162: F, t8342: F, t8344: F, t23083: F, t32837: F, t23062: F, t32834: F, t112778: F, t112784: F, t112803: F, t118533: F, t118535: F, t118539: F, t118546: F, t118549: F, t118552: F, t118556: F, t118559: F, t118562: F, t118566: F, t118569: F, t118573: F) -> F {
    let t118576 = t4162 * t8342 * t8344;
    let t118578 = t23083 * t32837;
    let t118580 = t23062 * t32834;
    let t118582 = -t118533 / F::cast_from(1536.0_f64) - t118535 / F::cast_from(1536.0_f64) - t118539 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t118546 - F::cast_from(0.80745512188280781708e-3_f64) * t118549 + F::cast_from(0.33913115119077928318e-1_f64) * t118552 + F::cast_from(0.13457585364713463618e-3_f64) * t112778 + F::cast_from(0.16149102437656156342e-2_f64) * t118556 + F::cast_from(0.48447307312968469025e-2_f64) * t118559 + F::cast_from(0.33913115119077928318e-1_f64) * t112784 + t118562 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t112803 + F::cast_from(0.48447307312968469025e-2_f64) * t118566 - F::cast_from(0.80745512188280781708e-3_f64) * t118569 + F::cast_from(0.80745512188280781708e-3_f64) * t118573 + t118576 / F::cast_from(1536.0_f64) + F::cast_from(0.56521858531796547196e-2_f64) * t118578 + F::cast_from(0.33913115119077928318e-1_f64) * t118580;
    t118582
}
