//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 956/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk956<F: Float>(t14623: F, t6355: F, t14626: F, t5055: F, t2039: F, t2479: F, t270: F, t638: F, t2046: F, t2050: F, t31: F, t2475: F) -> (F, F, F, F, F) {
    let t77302 = t6355 * t14623;
    let t77303 = F::new(0.2993560425465952141e-1) * t77302;
    let t77304 = t5055 * t14626;
    let t77305 = F::new(0.44903406381989282115e-1) * t77304;
    let t77308 = t638 * t2039 * t2479 * t270;
    let t77309 = F::new(0.15243824895787514157e-3) * t77308;
    let t77312 = t2046 * t2050 * t2479 * t31;
    let t77313 = F::new(0.21684485328539747656e-4) * t77312;
    let t77316 = t638 * t2039 * t2475 * t270;
    (t77303, t77305, t77309, t77313, t77316)
}
