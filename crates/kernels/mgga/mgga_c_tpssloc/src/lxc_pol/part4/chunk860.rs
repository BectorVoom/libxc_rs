//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 860/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk860<F: Float>(t13822: F, t4548: F, t973: F, t2970: F, t4522: F, t10254: F, t3961: F, t10236: F, t10189: F, t1597: F, t2990: F, t2986: F, t2987: F, t4540: F, t2989: F, t3966: F) -> (F, F, F, F, F, F, F, F) {
    let t13823 = t13822 * t4548;
    let t13825 = 0.55555555555555555554e-3 * t973 * t13823;
    let t13828 = t2970 * t4522;
    let t13830 = 0.18518518518518518518e-3 * t973 * t13828;
    let t13835 = t10254 * t3961;
    let t13839 = t10236 * t3961;
    let t13847 = t10189 * t1597;
    let t13848 = t13847 * t2990;
    let t13850 = 0.18518518518518518518e-3 * t2986 * t13848;
    let t13851 = t2987 * t4540;
    let t13861 = t2989 * t3966;
    (t13825, t13830, t13835, t13839, t13847, t13850, t13851, t13861)
}
