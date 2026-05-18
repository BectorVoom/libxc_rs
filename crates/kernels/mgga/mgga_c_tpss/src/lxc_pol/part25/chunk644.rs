//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 644/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk644<F: Float>(t3764: F, t3767: F, t3769: F, t3772: F, t3809: F, t3813: F, t3891: F, t3893: F, t3896: F, t3898: F, t3902: F, t3906: F, t3911: F) -> F {
    let t3949 = -t3764 + t3767 + t3769 - t3772 + t3809 + t3813 + t3891 + t3893 - t3896 - t3898 + t3902 - t3906 - t3911;
    t3949
}
