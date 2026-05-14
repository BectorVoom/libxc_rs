//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1292/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1292<F: Float>(t20245: F, t337: F, t104126: F, t104128: F, t104139: F, t104142: F, t104150: F, t104153: F, t104181: F, t104184: F, t104187: F, t131: F, t22280: F, t22284: F, t22288: F, t24741: F, t27704: F, t29569: F, t29625: F, t467: F, t8035: F, t86324: F, t86327: F, t95450: F) -> (F, F) {
    let t109535 = t20245 * t337;
    let t109553 = -t104126 / 72.0 + 19.0 / 432.0 * t104128 - 0.30279567070605293142e-3 * t27704 * t29625 - 0.48447307312968469026e-2 * t29569 * t8035 - 77.0 / 162.0 * t109535 * t131 * t467 + 11.0 / 108.0 * t104139 + 0.21801288290835811062e-1 * t104142 + t104150 / 54.0 + 0.48447307312968469026e-2 * t104153 + 0.60559134141210586284e-3 * t104181 - 0.30279567070605293142e-3 * t104184 + t95450 / 54.0 - 0.60559134141210586284e-3 * t104187 - t86324 * t22280 / 384.0 + t86327 * t22284 / 768.0 - t24741 * t22288 / 384.0;
    (t109535, t109553)
}
