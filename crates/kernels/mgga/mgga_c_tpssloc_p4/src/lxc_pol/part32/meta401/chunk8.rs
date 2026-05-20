//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1524/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1524<F: Float>(t17712: F, t4594: F, t4582: F, t1023: F, t1041: F, t10413: F, t10436: F, t10511: F, t10871: F, t14049: F, t14059: F, t17688: F, t17693: F, t17697: F, t17701: F, t17705: F, t3039: F, t3070: F, t3114: F, t3130: F, t4585: F, t4590: F, t4644: F, t5869: F) -> F {
    let t17713 = t17712 * t4594;
    let t17714 = t4582 * t17713;
    let t17717 = t17712 * t1023;
    let t17718 = t4582 * t17717;
    let t17725 = -t14049 - t10436 / F::new(13824.0) - F::new(5.0) / F::new(2304.0) * t1041 * t17688 + F::new(5.0) / F::new(6912.0) * t1041 * t17693 + F::new(5.0) / F::new(5184.0) * t1041 * t17697 - t10413 * t17701 / F::new(4608.0) + t3070 * t17705 / F::new(2304.0) - t4644 * t4585 / F::new(1152.0) + F::new(5.0) / F::new(6912.0) * t4644 * t4590 - t14059 + t3130 * t17714 / F::new(1536.0) - t3039 * t17718 / F::new(3072.0) + t3114 * t5869 / F::new(3072.0) - t10511 / F::new(13824.0) - t10871 / F::new(20736.0);
    t17725
}
