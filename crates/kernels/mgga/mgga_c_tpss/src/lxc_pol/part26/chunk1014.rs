//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1014/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1014<F: Float>(t8118: F, t4806: F, t8096: F, t4740: F, t680: F, t682: F, t2436: F, t10558: F, t10560: F, t10687: F, t8212: F, t8218: F, t10710: F, t162: F, t3566: F, t10701: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14119 = 0.24415263074675393405e-3 * t8118;
    let t14123 = t4806 * t8096;
    let t14127 = t680 * t4740;
    let t14129 = 4.0 * t14127 * t682;
    let t14130 = t4806 * t2436;
    let t14137 = 0.11696447245269292414e1 * t10558;
    let t14138 = 0.34631718211362927517e2 * t10560;
    let t14139 = 0.48830526149350786811e-3 * t10687;
    let t14140 = 0.17315859105681463759e2 * t8212;
    let t14141 = 0.11696447245269292414e1 * t8218;
    let t14142 = t10710 * t162;
    let t14144 = 24.0 * t14142 * t3566;
    let t14145 = 0.23392894490538584828e1 * t10701;
    (t14119, t14123, t14129, t14130, t14137, t14138, t14139, t14140, t14141, t14144, t14145)
}
