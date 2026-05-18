//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1032/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1032<F: Float>(t13998: F, t14591: F, t14592: F, t14593: F, t14594: F, t14595: F, t14596: F, t14598: F, t14607: F, t14955: F, t14956: F, t14957: F, t14958: F, t14959: F, t14960: F, t14961: F) -> F {
    let t79951 = t14591 + t14592 + t14593 - t14594 + t14595 + t13998 - t14955 + t14596 + t14956 + t14598 + t14957 - t14958 + t14959 + t14960 + t14961 + t14607;
    t79951
}
