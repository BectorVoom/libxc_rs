//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 808/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk808<F: Float>(t2121: F, t24844: F, t210: F, t7371: F, t7284: F, t974: F, t1089: F, t491: F, t7327: F, t15707: F, t7376: F, t24574: F, t7365: F) -> (F, F, F, F) {
    let t24845 = t2121 * t24844;
    let t24847 = t7371 * t210;
    let t24848 = t974 * t7284;
    let t24849 = t24847 * t24848;
    let t24850 = t491 * t1089;
    let t24851 = t7327 * t24850;
    let t24852 = t15707 * t7376;
    let t24853 = t24851 * t24852;
    let t24856 = t24574 * t7365;
    (t24845, t24849, t24853, t24856)
}
