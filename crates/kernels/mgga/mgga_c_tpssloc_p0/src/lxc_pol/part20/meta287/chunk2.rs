//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1489/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1489<F: Float>(t10620: F, t10649: F, t10652: F, t10654: F, t10657: F, t10665: F, t10699: F, t10707: F, t10771: F, t10772: F, t10806: F, t10811: F, t10814: F, t10819: F, t10820: F, t10825: F, t10828: F, t10829: F, t10843: F, t2900: F, t2925: F, t2933: F, t311: F, t924: F, t952: F) -> F {
    let t10847 = -F::cast_from(0.19298375398431042081e3_f64) * t10771 * t10772 + F::cast_from(1.0_f64) * t924 * t10806 + F::cast_from(0.2069040516770936012e4_f64) * t10811 * t10814 + t10819 + t10649 - t10652 - t10654 - t10657 + t10665 - t10699 - t10707 + F::cast_from(0.17544670867903938621e1_f64) * t10820 * t952 + F::cast_from(0.17544670867903938621e1_f64) * t2900 * t2925 + F::cast_from(0.51947577317044391276e2_f64) * t10825 * t2933 - F::cast_from(0.10389515463408878255e3_f64) * t10828 * t10829 - F::cast_from(0.310907e-1_f64) * t10843 * t311 - F::cast_from(0.19751673498613801407e-1_f64) * t10620;
    t10847
}
