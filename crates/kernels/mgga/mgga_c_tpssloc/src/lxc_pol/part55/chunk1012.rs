//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1012/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1012<F: Float>(t1241: F, t34305: F, t32519: F, t8002: F, t2154: F, t8087: F, t3598: F, t1760: F, t8897: F, t1238: F, t1761: F, t2121: F, t2155: F, t27792: F, t32482: F, t34238: F, t34241: F, t34244: F, t34247: F, t34251: F, t34254: F, t34278: F, t4945: F, t498: F, t7283: F, t8898: F) -> (F, F, F, F, F) {
    let t34306 = t1241 * t34305;
    let t34310 = t32519 * t8002;
    let t34313 = t2154 * t8087;
    let t34314 = t3598 * t34313;
    let t34317 = t8897 * t1760;
    let t34318 = t3598 * t34317;
    let t34321 = 0.16449340668482264365e-1 * t2121 * t34238 - 0.16449340668482264365e-1 * t7283 * t34241 - 0.16449340668482264365e-1 * t7283 * t34244 - 0.16449340668482264365e-1 * t7283 * t34247 - 0.16449340668482264365e-1 * t7283 * t34251 + t34254 * t498 + t34278 * t498 - 2.0 * t27792 * t2155 - t1238 * t34306 - t4945 * t8898 - t32482 * t1761 - 0.54831135561607547883e-2 * t7283 * t34310 + 4.0 * t1238 * t34314 + 2.0 * t1238 * t34318;
    (t34306, t34310, t34314, t34318, t34321)
}
