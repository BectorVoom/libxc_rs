//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1016/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1016<F: Float>(t4528: F, t973: F, t1597: F, t2987: F, t2990: F, t2824: F, t3003: F, t4384: F, t4387: F, t4390: F, t4393: F) -> (F, F, F, F) {
    let t4529 = t973 * t4528;
    let t4531 = t2987 * t1597;
    let t4532 = t4531 * t2990;
    let t4540 = -t3003 - t2824 / F::new(9.0) - t4384 / F::new(9.0) + t4387 / F::new(18.0) - t4390 / F::new(3.0) + t4393 / F::new(6.0);
    (t4529, t4531, t4532, t4540)
}
