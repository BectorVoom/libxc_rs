//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1863/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1863<F: Float>(t14228: F, t4342: F, t3071: F, t1025: F, t10403: F, t1041: F, t10413: F, t10909: F, t10923: F, t10927: F, t14174: F, t14180: F, t14184: F, t14189: F, t14194: F, t14198: F, t14203: F, t14207: F, t14215: F, t14222: F, t2960: F, t3070: F, t3117: F, t4590: F, t4609: F, t973: F) -> (F, F, F) {
    let t14229 = t4342 * t14228;
    let t14230 = t3071 * t14229;
    let t14233 = -F::new(5.0) / F::new(2304.0) * t1041 * t14174 + F::new(5.0) / F::new(6912.0) * t3117 * t4590 + F::new(5.0) / F::new(6912.0) * t1041 * t14180 + F::new(5.0) / F::new(13824.0) * t1041 * t14184 + F::new(5.0) / F::new(5184.0) * t1041 * t14189 + t14194 - t2960 * t4609 / F::new(54.0) + t973 * t14198 / F::new(288.0) - t14203 / F::new(20736.0) + t14207 * t1025 / F::new(1536.0) + t10909 / F::new(4608.0) + t10403 * t14215 / F::new(1152.0) - t10413 * t14222 / F::new(2304.0) - t10923 / F::new(648.0) - t10927 / F::new(162.0) - t3070 * t14230 / F::new(1152.0);
    (t14229, t14230, t14233)
}
