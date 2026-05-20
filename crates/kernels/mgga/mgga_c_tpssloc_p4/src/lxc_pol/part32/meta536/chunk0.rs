//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1874/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1874<F: Float>(t5075: F, t7376: F, t7375: F, t225: F, t8034: F, t7364: F, t5072: F, t1215: F, t1409: F, t24851: F, t24589: F, t24812: F, t24827: F, t24849: F, t27406: F, t27481: F, t27484: F, t27492: F, t27498: F, t27502: F, t27507: F, t7283: F, t7368: F, t7373: F, t7378: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27510 = t5075 * t7376;
    let t27511 = t7375 * t27510;
    let t27516 = t8034 * t225;
    let t27517 = t27516 * t7364;
    let t27520 = t5072 * t7376;
    let t27521 = t7375 * t27520;
    let t27524 = t1409 * t1215;
    let t27525 = t27524 * t7376;
    let t27526 = t24851 * t27525;
    let t27529 = -F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27481 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27484 + F::cast_from(0.27415567780803773942e-2_f64) * t24827 + F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t27492 - F::cast_from(0.82246703342411321825e-2_f64) * t24812 * t27498 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t27502 - F::cast_from(0.21932454224643019153e-1_f64) * t27507 * t7378 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t27511 + F::cast_from(0.21932454224643019153e-1_f64) * t27406 * t7368 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t27517 + F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t27521 - F::cast_from(0.27415567780803773942e-2_f64) * t24849 * t27526;
    (t27510, t27511, t27516, t27517, t27520, t27521, t27525, t27526, t27529)
}
