//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1359/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1359<F: Float>(t63935: F, t63945: F, t63949: F, t63957: F, t63964: F, t66420: F, t69972: F, t69974: F, t69976: F, t69978: F, t69981: F, t69983: F, t69985: F) -> F {
    let t72069 = -t69972 / F::new(24.0) + t69974 / F::new(96.0) + t69976 / F::new(96.0) - t69978 / F::new(96.0) - t63935 - F::new(7.0) / F::new(24.0) * t69981 + F::new(7.0) / F::new(72.0) * t69983 + t69985 / F::new(192.0) - F::new(119.0) / F::new(1728.0) * t63945 - t63949 - F::new(35.0) / F::new(54.0) * t63957 + t66420 - F::new(119.0) / F::new(432.0) * t63964;
    t72069
}
