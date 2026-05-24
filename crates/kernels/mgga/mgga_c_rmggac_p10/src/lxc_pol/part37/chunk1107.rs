//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1107/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1107<F: Float>(t78007: F, t78008: F, t78009: F, t78010: F, t78011: F, t78012: F, t78017: F, t78018: F, t78019: F, t78020: F, t78021: F, t78024: F, t78027: F) -> F {
    let t80466 = -t78007 - t78008 + t78009 + t78010 - t78011 - t78012 - t78017 - t78018 - t78019 + t78020 + t78021 - t78024 - t78027;
    t80466
}
