//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2684/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2684<F: Float>(t74699: F, t74735: F, t74754: F, t74765: F, t225: F, t1307: F, t16305: F, t16311: F, t19876: F, t19890: F, t19966: F, t40124: F, t40145: F, t5246: F, t54534: F, t554: F, t559: F, t57127: F, t57143: F, t57145: F, t57158: F, t57160: F, t57170: F, t6414: F, t74677: F) -> (F, F, F) {
    let t74767 = t74699 + t74735 + t74754 + t74765;
    let t74768 = t74767 * t225;
    let t74786 = -t5246 * t16305 * t16311 * t6414 * t1307 / F::new(128.0) - t19876 * t19890 / F::new(64.0) + t74768 * t554 * t559 / F::new(3072.0) + F::new(595.0) / F::new(10368.0) * t40124 - F::new(595.0) / F::new(10368.0) * t40145 - t54534 + F::new(35.0) / F::new(192.0) * t57127 + t19876 * t19966 / F::new(512.0) + F::new(35.0) / F::new(384.0) * t57143 - F::new(7.0) / F::new(384.0) * t57145 + F::new(7.0) / F::new(4.0) * t57158 - F::new(7.0) / F::new(8.0) * t57160 - F::new(7.0) / F::new(16.0) * t57170 - t5246 * t16305 * t16311 * t74677 / F::new(64.0);
    (t74767, t74768, t74786)
}
