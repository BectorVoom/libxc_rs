//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 852/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk852<F: Float>(t114759: F, t114814: F, t114864: F, t114891: F, t112834: F, t112840: F, t112850: F, t112855: F, t225: F, t31974: F, t114932: F, t114943: F, t114672: F, t31984: F, t814: F, t114688: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t116514 = 0.25587863262083522346e0 * t114759;
    let t116536 = 0.10417915756705434098e0 * t114814;
    let t116557 = 0.25587863262083522346e0 * t114864;
    let t116578 = 0.10417915756705434098e0 * t114891;
    let t116608 = 0.84334201618871038669e-2 * t112834;
    let t116610 = 0.26915170729426927235e-3 * t112840;
    let t116613 = 119.0 / 1728.0 * t112850;
    let t116615 = 0.18086994730174895102e0 * t112855;
    let t116645 = t31974 * t225;
    let t116648 = 0.3289868133696452873e-1 * t114932;
    let t116654 = 0.3289868133696452873e-1 * t114943;
    let t116673 = 0.10417915756705434098e0 * t114672;
    let t116681 = t814 * t31984;
    let t116686 = 0.3289868133696452873e-1 * t114688;
    (t116514, t116536, t116557, t116578, t116608, t116610, t116613, t116615, t116645, t116648, t116654, t116673, t116681, t116686)
}
