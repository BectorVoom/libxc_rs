//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 954/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk954<F: Float>(t21589: F, t959: F, t21093: F, t21097: F, t21099: F, t21103: F, t21105: F, t21107: F, t21365: F, t21367: F, t21369: F, t21375: F, t21251: F, t21255: F, t21263: F, t21265: F, t21267: F, t21270: F, t21302: F, t21305: F, t21317: F, t21320: F, t21336: F, t21372: F) -> (F, F, F) {
    let t21591 = 0.5848223622634646207e0 * t959 * t21589;
    let t21592 = t21367 + t21375 + t21369 - t21093 + t21097 - t21591 + t21365 - t21099 - t21105 - t21107 - t21103;
    let t21593 = -t21251 + t21255 - t21317 + t21320 - t21372 + t21263 + t21265 + t21267 - t21270 + t21302 + t21305 - t21336;
    (t21591, t21592, t21593)
}
