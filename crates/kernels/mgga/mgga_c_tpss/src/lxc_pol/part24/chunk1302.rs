//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1302/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1302<F: Float>(t19809: F, t63840: F, t198: F, t205: F, t6148: F, t17930: F, t52639: F, t21262: F, t750: F, t821: F, t19817: F, t14245: F, t19671: F, t1398: F, t3610: F, t1288: F, t3683: F, t823: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t69789 = t63840 * t19809;
    let t69793 = t198 * t205 * t6148;
    let t69796 = t17930 * t52639;
    let t69799 = t21262 * t750;
    let t69800 = t17930 * t69799;
    let t69803 = t21262 * t821;
    let t69804 = t19817 * t69803;
    let t69807 = t19671 * t14245;
    let t69810 = t3610 * t1398;
    let t69811 = t17930 * t69810;
    let t69817 = t823 * t1288 * t3683;
    (t69789, t69793, t69796, t69799, t69800, t69803, t69804, t69807, t69810, t69811, t69817)
}
