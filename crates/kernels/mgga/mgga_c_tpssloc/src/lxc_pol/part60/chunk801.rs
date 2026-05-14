//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 801/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk801<F: Float>(t1894: F, t7823: F, t214: F, t1880: F, t1510: F, t31394: F, t31353: F, t31355: F, t31359: F, t32835: F, t32838: F, t32841: F, t32845: F, t32847: F, t235: F, t1499: F, t226: F, t30675: F, t30683: F, t31375: F, t31383: F, t32821: F, t32825: F, t32829: F, t33377: F, t33381: F, t812: F, t8560: F) -> (F, F, F, F, F, F) {
    let t33383 = t1894 * t7823;
    let t33384 = t214 * t33383;
    let t33385 = t1880 * t33384;
    let t33388 = t31394 * t1510;
    let t33395 = -t31353 - 0.96894614625936938046e-2 * t32835 - t31355 - 0.16149102437656156341e-2 * t32838 + t32841 / 768.0 - t32845 / 768.0 - t31359 - t32847 / 192.0;
    let t33396 = t235 * t33395;
    let t33398 = -t30675 - t32821 - t30683 - t32825 + t32829 - t31375 - 0.16449340668482264365e-1 * t33377 - t31383 - 0.82246703342411321825e-2 * t33381 + 0.82246703342411321825e-2 * t33385 + t1499 * t8560 - t812 * t33388 + t226 * t33396;
    (t33383, t33384, t33388, t33395, t33396, t33398)
}
