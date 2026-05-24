//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 920/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk920<F: Float>(t939: F, t348: F, t2717: F, t328: F, t356: F, t353: F, t8550: F, t2724: F, t345: F, t2716: F, t941: F, t2662: F, t921: F) -> (F, F, F, F, F, F, F) {
    let t8551 = t939 * t939;
    let t8552 = F::new(1.0) / t8551;
    let t8553 = t8552 * t348;
    let t8556 = F::new(1.0) / t2717 / t356 / t328;
    let t8557 = t353 * t8556;
    let t8559 = t8550 * t8553 * t8557;
    let t8561 = t2724 * t345;
    let t8568 = t8550 * t2716 * t8557;
    let t8577 = t8550 * t941 * t8557;
    let t8586 = t2662 * t921;
    (t8552, t8556, t8559, t8561, t8568, t8577, t8586)
}
