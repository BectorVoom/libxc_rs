//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1051/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1051<F: Float>(t252: F, t5584: F, t828: F, t9975: F, t16758: F, t4182: F, t2732: F, t5617: F, t829: F, t1499: F, t4290: F, t4166: F, t4177: F) -> (F, F, F, F, F, F, F, F) {
    let t16815 = t252 * t5584;
    let t16816 = t9975 * t828;
    let t16817 = t16815 * t16816;
    let t16820 = t16758 * t4182;
    let t16823 = t2732 * t5617;
    let t16825 = t16815 * t4182;
    let t16828 = t16815 * t829;
    let t16830 = t1499 * t4290;
    let t16836 = t4166 * t4177;
    (t16816, t16817, t16820, t16823, t16825, t16828, t16830, t16836)
}
