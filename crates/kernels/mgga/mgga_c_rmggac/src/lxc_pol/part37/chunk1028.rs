//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1028/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1028<F: Float>(t14404: F, t14406: F, t14938: F, t14939: F, t15071: F, t15431: F, t15433: F, t15434: F, t15438: F, t15446: F, t15452: F, t15858: F, t15859: F, t70659: F, t70661: F) -> F {
    let t79944 = t15431 + t15858 + t15071 - t15433 - t15434 + t15859 + t15438 - t14938 + t15446 - t70659 - t14404 + t70661 + t14406 + t14939 + t15452;
    t79944
}
