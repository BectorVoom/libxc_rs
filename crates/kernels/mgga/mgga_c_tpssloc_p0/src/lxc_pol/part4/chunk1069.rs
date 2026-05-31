//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1069/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1069<F: Float>(t17704: F, t3071: F, t376: F, t5866: F, t4594: F, t4582: F, t1023: F, t1041: F, t10413: F, t10436: F, t10511: F, t10871: F, t14049: F, t14059: F, t17688: F, t17693: F, t17697: F, t17701: F, t3039: F, t3070: F, t3114: F, t3130: F, t4585: F, t4590: F, t4644: F, t5869: F) -> F {
    let t17705 = t3071 * t17704;
    let t17712 = t376 * t5866;
    let t17713 = t17712 * t4594;
    let t17714 = t4582 * t17713;
    let t17717 = t17712 * t1023;
    let t17718 = t4582 * t17717;
    let t17725 = -t14049 - t10436 / F::cast_from(13824.0_f64) - F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1041 * t17688 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t1041 * t17693 + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1041 * t17697 - t10413 * t17701 / F::cast_from(4608.0_f64) + t3070 * t17705 / F::cast_from(2304.0_f64) - t4644 * t4585 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t4644 * t4590 - t14059 + t3130 * t17714 / F::cast_from(1536.0_f64) - t3039 * t17718 / F::cast_from(3072.0_f64) + t3114 * t5869 / F::cast_from(3072.0_f64) - t10511 / F::cast_from(13824.0_f64) - t10871 / F::cast_from(20736.0_f64);
    t17725
}
