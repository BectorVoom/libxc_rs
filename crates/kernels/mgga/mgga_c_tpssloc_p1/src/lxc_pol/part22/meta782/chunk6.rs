//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2678/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2678<F: Float>(t16224: F, t16225: F, t16305: F, t16311: F, t5246: F, t5250: F, t54013: F, t54199: F, t56927: F, t56933: F, t56935: F, t56937: F, t56946: F, t56953: F, t56959: F, t56961: F, t56963: F, t56993: F, t57172: F, t6388: F, t74415: F) -> F {
    let t74655 = -F::new(119.0) / F::new(4608.0) * t56927 + F::new(7.0) / F::new(1536.0) * t56933 - F::new(7.0) / F::new(384.0) * t56935 - F::new(35.0) / F::new(384.0) * t56937 - t54199 + F::new(35.0) / F::new(24.0) * t56946 - F::new(35.0) / F::new(72.0) * t56953 - F::new(7.0) / F::new(192.0) * t56959 - F::new(7.0) / F::new(192.0) * t56961 - F::new(7.0) / F::new(192.0) * t56963 + F::new(3.0) / F::new(512.0) * t5246 * t54013 * t74415 * t5250 + F::new(5.0) / F::new(128.0) * t5246 * t16224 * t16311 * t57172 - F::new(3.0) / F::new(128.0) * t5246 * t16305 * t6388 * t16225 + F::new(119.0) / F::new(576.0) * t56993;
    t74655
}
