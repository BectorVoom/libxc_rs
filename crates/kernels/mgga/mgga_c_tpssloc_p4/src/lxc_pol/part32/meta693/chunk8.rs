//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2154/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2154<F: Float>(t22751: F, t28139: F, t28159: F, t6897: F, t794: F, t19763: F, t1992: F, t6976: F, t1336: F, t19735: F, t22873: F, t26403: F, t26459: F, t5234: F, t5334: F, t6388: F, t6415: F, t81105: F, t90971: F, t90984: F, t90988: F, t93595: F, t97079: F, t97083: F, t97087: F, t97091: F, t97095: F, t97106: F) -> F {
    let t97108 = t22751 * t28139;
    let t97111 = t6897 * t794 * t28159;
    let t97114 = t1992 * t6976 * t19763;
    let t97116 = -F::cast_from(0.82246703342411321825e-2_f64) * t97079 + F::cast_from(0.3289868133696452873e-1_f64) * t97083 + F::cast_from(0.3289868133696452873e-1_f64) * t97087 + F::cast_from(0.3289868133696452873e-1_f64) * t97091 - F::cast_from(2.0_f64) * t5234 * t26459 + t90971 + F::cast_from(0.76763589786250567037e-1_f64) * t97095 + F::cast_from(4.0_f64) * t5334 * t26403 * t19735 - t1336 * t22873 * t6415 + t93595 + t90984 - t90988 + F::cast_from(2.0_f64) * t1336 * t81105 * t6388 + F::cast_from(0.3289868133696452873e-1_f64) * t97106 + F::cast_from(0.38381794893125283518e-1_f64) * t97108 - F::cast_from(0.41123351671205660912e-2_f64) * t97111 - F::cast_from(0.82246703342411321825e-2_f64) * t97114;
    t97116
}
