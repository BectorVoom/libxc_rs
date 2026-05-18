//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1275/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1275<F: Float>(t4162: F, t8342: F, t8344: F, t23083: F, t32837: F, t23062: F, t32834: F, t112778: F, t112784: F, t112803: F, t118533: F, t118535: F, t118539: F, t118546: F, t118549: F, t118552: F, t118556: F, t118559: F, t118562: F, t118566: F, t118569: F, t118573: F) -> F {
    let t118576 = t4162 * t8342 * t8344;
    let t118578 = t23083 * t32837;
    let t118580 = t23062 * t32834;
    let t118582 = -t118533 / F::new(1536.0) - t118535 / F::new(1536.0) - t118539 / F::new(1536.0) + F::new(5.0) / F::new(384.0) * t118546 - F::new(0.80745512188280781708e-3) * t118549 + F::new(0.33913115119077928318e-1) * t118552 + F::new(0.13457585364713463618e-3) * t112778 + F::new(0.16149102437656156342e-2) * t118556 + F::new(0.48447307312968469025e-2) * t118559 + F::new(0.33913115119077928318e-1) * t112784 + t118562 / F::new(768.0) + F::new(7.0) / F::new(2304.0) * t112803 + F::new(0.48447307312968469025e-2) * t118566 - F::new(0.80745512188280781708e-3) * t118569 + F::new(0.80745512188280781708e-3) * t118573 + t118576 / F::new(1536.0) + F::new(0.56521858531796547196e-2) * t118578 + F::new(0.33913115119077928318e-1) * t118580;
    t118582
}
