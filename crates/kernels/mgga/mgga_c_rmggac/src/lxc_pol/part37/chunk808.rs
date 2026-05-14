//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 808/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk808<F: Float>(t74652: F, t74657: F, t68753: F, t74674: F, t16503: F, t2211: F, t34976: F, t8435: F, t15450: F, t34761: F, t1502: F, t699: F, t74684: F, t74687: F, t74690: F, t74693: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t77135 = 0.12263514265030957031e-4 * t74652;
    let t77137 = 0.54549323308490683456e-1 * t74657;
    let t77138 = 0.54549323308490683456e-1 * t68753;
    let t77143 = 0.1702583995731913576e-4 * t74674;
    let t77147 = t16503 * t34976 * t2211 * t8435;
    let t77148 = 0.85129199786595678796e-5 * t77147;
    let t77149 = t34761 * t15450;
    let t77150 = 0.42564599893297839398e-5 * t77149;
    let t77153 = t16503 * t34976 * t699 * t1502;
    let t77154 = 0.42564599893297839398e-5 * t77153;
    let t77155 = 0.2553875993597870364e-4 * t74684;
    let t77156 = 0.2553875993597870364e-4 * t74687;
    let t77157 = 0.3830813990396805546e-4 * t74690;
    let t77158 = 0.1276937996798935182e-4 * t74693;
    (t77135, t77137, t77138, t77143, t77148, t77150, t77154, t77155, t77156, t77157, t77158)
}
