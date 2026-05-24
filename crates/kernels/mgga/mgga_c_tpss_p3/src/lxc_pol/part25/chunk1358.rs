//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1358/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1358<F: Float>(t62690: F, t69945: F, t69948: F, t69950: F, t69952: F, t69954: F, t69956: F, t69958: F, t69960: F, t69962: F, t69964: F, t69966: F, t69968: F) -> F {
    let t72057 = -t69945 / F::new(2.0) + t69948 / F::new(4.0) - t62690 - t69950 / F::new(192.0) + F::new(7.0) / F::new(288.0) * t69952 - F::new(35.0) / F::new(288.0) * t69954 - t69956 / F::new(384.0) - t69958 / F::new(768.0) - F::new(5.0) / F::new(192.0) * t69960 - t69962 / F::new(128.0) + t69964 / F::new(128.0) + t69966 / F::new(192.0) - t69968 / F::new(768.0);
    t72057
}
