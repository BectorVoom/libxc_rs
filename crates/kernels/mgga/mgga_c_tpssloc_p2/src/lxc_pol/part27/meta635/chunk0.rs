//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2140/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2140<F: Float>(t4191: F, t81865: F, t13302: F, t23146: F, t13322: F, t4250: F, t13316: F, t13312: F, t81749: F, t23145: F, t4166: F, t2649: F) -> (F, F, F, F, F, F, F, F) {
    let t87185 = t81865 * t4191;
    let t87187 = t23146 * t13302;
    let t87189 = t23146 * t13322;
    let t87191 = t81865 * t4250;
    let t87193 = t23146 * t13316;
    let t87195 = t23146 * t13312;
    let t87197 = t81749 * t4250;
    let t87198 = F::new(7.0) / F::new(288.0) * t87197;
    let t87199 = t4166 * t23145;
    let t87200 = t87199 * t2649;
    (t87185, t87187, t87189, t87191, t87193, t87195, t87198, t87200)
}
