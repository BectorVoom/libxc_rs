//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 907/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk907<F: Float>(t12886: F, t145: F, t185: F, t2250: F, t4195: F, t4194: F, t4303: F, t870: F, t262: F, t4119: F, t2553: F, t4315: F, t9717: F, t12850: F, t12854: F, t12860: F, t12861: F, t12889: F, t1877: F, t2522: F, t4310: F, t4314: F, t776: F, t868: F, t9457: F, t9462: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F) -> (F, F, F, F) {
    let t12890 = t145 * t12886;
    let t12891 = t12890 * t185;
    let t12892 = t4195 * t2250;
    let t12894 = 12.0 * t4194 * t12892;
    let t12895 = t4303 * t870;
    let t12899 = t262 * t4119;
    let t12903 = t4315 * t2553;
    let t12906 = 0.5848223622634646207e0 * t9717;
    let t12907 = -2.0 * t12854 * t1877 * t868 + 6.0 * t12895 * t2522 * t776 + 12.0 * t12899 * t4314 * t776 + 3.0 * t2522 * t2553 * t4310 + 6.0 * t12903 * t4314 + t12850 - t12860 + t12861 + t12889 + t12891 + t12894 - t12906 - t9457 + t9462 - t9469 + t9476 + t9484 - t9496 - t9715;
    (t12891, t12894, t12906, t12907)
}
