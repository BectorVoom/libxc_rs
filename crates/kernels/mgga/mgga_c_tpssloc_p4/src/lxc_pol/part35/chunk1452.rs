//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1452/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1452<F: Float>(t104410: F, t104425: F, t104435: F, t104441: F, t104445: F, t106348: F, t1726: F, t1730: F, t2132: F, t2136: F, t22129: F, t22137: F, t27674: F, t29562: F, t29600: F, t29625: F, t29651: F, t488: F, t6178: F, t6184: F, t6188: F, t7310: F, t7573: F, t8028: F, t8031: F, t8035: F, t95550: F) -> F {
    let t109694 = t95550 / F::new(3456.0) + F::cast_from(0.30279567070605293142e-3_f64) * t8031 * t29625 - F::new(11.0) / F::new(108.0) * t104410 * t1726 + t27674 * t6184 / F::new(36.0) + t27674 * t6188 / F::new(18.0) - t7310 * t22129 / F::new(288.0) - F::cast_from(0.21801288290835811062e-1_f64) * t29562 * t7573 * t2136 + F::cast_from(0.30279567070605293142e-3_f64) * t29651 * t8035 + F::cast_from(0.24223653656484234513e-2_f64) * t8028 * t29625 + F::new(19.0) / F::new(288.0) * t1730 * t29600 * t488 - F::cast_from(0.10093189023535097714e-3_f64) * t2132 * t106348 * t2136 - t27674 * t6178 / F::new(27.0) + t7310 * t22137 / F::new(36.0) - F::cast_from(0.48447307312968469026e-2_f64) * t104425 + F::new(5.0) / F::new(3456.0) * t104435 + t104441 / F::new(216.0) + t104445 / F::new(768.0);
    t109694
}
