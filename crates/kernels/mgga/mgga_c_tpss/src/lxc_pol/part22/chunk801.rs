//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 801/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk801<F: Float>(t3348: F, t4478: F, t774: F, t1248: F, t4397: F, t1213: F, t1222: F, t1244: F, t3239: F, t3241: F, t3244: F, t3268: F, t3271: F, t3340: F, t3343: F, t4402: F, t4405: F, t4409: F, t4413: F, t4419: F, t4422: F, t4425: F, t4462: F, t4466: F, t4473: F, t4476: F) -> (F, F, F) {
    let t4480 = t3348 * t774 * t4478;
    let t4484 = t1248 * t774 * t4397;
    let t4487 = t3239 + F::new(7.0) / F::new(144.0) * t3241 + F::new(7.0) / F::new(144.0) * t4402 + t3244 * t4405 / F::new(16.0) - t1213 * t4409 / F::new(48.0) + t4413 * t4419 / F::new(1536.0) + F::new(7.0) / F::new(4608.0) * t4422 + t3271 * t4425 / F::new(768.0) - t1222 * t4462 / F::new(3072.0) - t3271 * t4466 / F::new(3072.0) + F::new(7.0) / F::new(4608.0) * t3268 + t3340 + F::new(7.0) / F::new(1152.0) * t3343 + t3271 * t4473 / F::new(768.0) + F::new(7.0) / F::new(1152.0) * t4476 + F::new(5.0) / F::new(768.0) * t1244 * t4480 - t1244 * t4484 / F::new(768.0);
    (t4480, t4484, t4487)
}
