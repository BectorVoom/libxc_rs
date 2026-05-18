//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1050/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1050<F: Float>(t11361: F, t11377: F, t11413: F, t11431: F, t294: F, t11224: F, t11226: F, t11228: F, t11230: F, t11234: F, t11237: F, t11240: F, t11242: F, t11245: F, t11248: F, t11251: F, t11255: F, t11258: F, t11262: F, t11265: F, t11267: F, t11269: F) -> (F, F) {
    let t11434 = t294 * (t11361 + t11377 + t11413 + t11431);
    let t11435 = -t11224 + t11226 - t11228 - t11230 - t11234 - t11237 - t11240 + t11242 - t11245 - t11248 - t11251 + t11255 + t11258 + t11262 + t11265 + t11267 - t11269 + t11434;
    (t11434, t11435)
}
