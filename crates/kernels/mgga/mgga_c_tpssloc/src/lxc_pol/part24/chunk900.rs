//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 900/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk900<F: Float>(t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t10620: F, t10649: F, t10652: F, t10654: F, t10657: F, t10665: F, t10699: F, t10707: F, t10771: F, t10772: F, t10806: F, t10811: F, t10814: F, t10819: F, t10820: F, t10825: F, t10828: F, t10829: F, t2900: F, t2925: F, t2933: F, t311: F, t924: F, t952: F) -> (F,) {
    let t10832 = 0.53272592592592592592e-1 * t10544;
    let t10843 = -t10832 - 0.2283111111111111111e-1 * t10556 + 0.11415555555555555555e-1 * t10558 - 0.34246666666666666665e-1 * t10560 + 0.17123333333333333333e-1 * t10562 - 0.19025925925925925925e-1 * t10566 + 0.68493333333333333331e-1 * t10569 - 0.34246666666666666665e-1 * t10530 - 0.10274e0 * t10572 + 0.10274e0 * t10538 - 0.17123333333333333333e-1 * t10575;
    let t10847 = -0.19298375398431042081e3 * t10771 * t10772 + 1.0 * t924 * t10806 + 0.2069040516770936012e4 * t10811 * t10814 + t10819 + t10649 - t10652 - t10654 - t10657 + t10665 - t10699 - t10707 + 0.17544670867903938621e1 * t10820 * t952 + 0.17544670867903938621e1 * t2900 * t2925 + 0.51947577317044391276e2 * t10825 * t2933 - 0.10389515463408878255e3 * t10828 * t10829 - 0.310907e-1 * t10843 * t311 - 0.19751673498613801407e-1 * t10620;
    (t10847,)
}
