//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 690/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk690<F: Float>(t2990: F, t4531: F, t2824: F, t3003: F, t4384: F, t4387: F, t4390: F, t4393: F, t340: F, t343: F, t974: F, t1597: F, t984: F) -> (F, F, F, F, F, F) {
    let t4532 = t4531 * t2990;
    let t4540 = -t3003 - t2824 / F::new(9.0) - t4384 / F::new(9.0) + t4387 / F::new(18.0) - t4390 / F::new(3.0) + t4393 / F::new(6.0);
    let t4541 = t340 * t4540;
    let t4542 = t4541 * t343;
    let t4543 = t974 * t4542;
    let t4546 = t974 * t340;
    let t4547 = t1597 * t984;
    (t4532, t4540, t4542, t4543, t4546, t4547)
}
