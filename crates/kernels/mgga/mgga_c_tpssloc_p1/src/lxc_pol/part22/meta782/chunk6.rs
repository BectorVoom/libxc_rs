//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2678/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2678<F: Float>(t16224: F, t16225: F, t16305: F, t16311: F, t5246: F, t5250: F, t54013: F, t54199: F, t56927: F, t56933: F, t56935: F, t56937: F, t56946: F, t56953: F, t56959: F, t56961: F, t56963: F, t56993: F, t57172: F, t6388: F, t74415: F) -> F {
    let t74655 = -F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t56927 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t56933 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t56935 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t56937 - t54199 + F::cast_from(35.0_f64) / F::cast_from(24.0_f64) * t56946 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t56953 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t56959 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t56961 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t56963 + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t5246 * t54013 * t74415 * t5250 + F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t5246 * t16224 * t16311 * t57172 - F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t5246 * t16305 * t6388 * t16225 + F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t56993;
    t74655
}
