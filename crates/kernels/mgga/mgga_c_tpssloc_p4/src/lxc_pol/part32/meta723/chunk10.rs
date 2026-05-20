//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2316/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2316<F: Float>(t27604: F, t4993: F, t19095: F, t24733: F, t1207: F, t19024: F, t7337: F, t19046: F, t7338: F, t6169: F, t7344: F, t1218: F, t1232: F, t1737: F, t1748: F, t18307: F, t18943: F, t18959: F, t24716: F, t6221: F, t7339: F, t7345: F, t86164: F, t95242: F, t95244: F, t95276: F, t95440: F) -> F {
    let t104007 = t27604 * t4993;
    let t104009 = t24733 * t19095;
    let t104012 = t1207 * t7337 * t19024;
    let t104015 = t19046 * t7338;
    let t104018 = t6169 * t7344;
    let t104029 = t95242 - t95244 + t24716 * t6221 / F::new(1536.0) + t7339 * t18943 / F::new(1536.0) + t104007 / F::new(324.0) - t104009 / F::new(2304.0) + F::new(19.0) / F::new(864.0) * t104012 * t1218 + t104015 * t1218 / F::new(1536.0) - t104018 * t1232 / F::new(2304.0) - t95276 * t1748 / F::new(1152.0) - t95440 * t1737 / F::new(144.0) - t86164 * t18307 / F::new(256.0) - t7345 * t18959 / F::new(1152.0);
    t104029
}
