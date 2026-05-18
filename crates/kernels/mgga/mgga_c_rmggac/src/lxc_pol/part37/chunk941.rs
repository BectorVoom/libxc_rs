//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 941/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk941<F: Float>(t72138: F, t77050: F, t74498: F, t74501: F, t74503: F, t15523: F, t2191: F, t1986: F, t675: F, t9566: F, t68660: F, t68686: F) -> (F, F, F, F, F, F, F, F) {
    let t77051 = t72138 * t77050;
    let t77052 = F::new(0.20455996240684006297e-1) * t77051;
    let t77054 = F::new(0.1276937996798935182e-4) * t74498;
    let t77055 = F::new(0.3830813990396805546e-4) * t74501;
    let t77056 = F::new(0.1276937996798935182e-4) * t74503;
    let t77057 = t2191 * t15523;
    let t77058 = F::new(0.42564599893297839398e-5) * t77057;
    let t77060 = t675 * t1986 * t9566;
    let t77061 = F::new(0.42564599893297839398e-5) * t77060;
    let t77062 = F::new(0.638468998399467591e-4) * t68660;
    let t77069 = F::new(0.36366215538993788974e-1) * t68686;
    (t77052, t77054, t77055, t77056, t77058, t77061, t77062, t77069)
}
