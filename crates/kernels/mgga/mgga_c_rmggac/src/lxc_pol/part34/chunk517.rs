//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 517/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk517<F: Float>(t14589: F, t2147: F, t13982: F, t13986: F, t13990: F, t13994: F, t14005: F, t14008: F, t14013: F, t14016: F, t14028: F, t14036: F, t14043: F, t14049: F, t14054: F, t14057: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14590 = t14589 * t2147;
    let t14591 = 0.68186654135613354322e-2 * t14590;
    let t14592 = 0.30487649791575028312e-3 * t13982;
    let t14593 = 0.30487649791575028312e-3 * t13986;
    let t14594 = 0.20455996240684006298e-1 * t13990;
    let t14595 = 0.2727466165424534173e-1 * t13994;
    let t14596 = 0.13637330827122670865e-1 * t14005;
    let t14597 = 0.52557918278704101564e-6 * t14008;
    let t14598 = 0.2627895913935205078e-5 * t14013;
    let t14599 = 0.87596530464506835935e-6 * t14016;
    let t14600 = 0.87596530464506835935e-6 * t14028;
    let t14601 = 0.17519306092901367188e-6 * t14036;
    let t14602 = 0.4379826523225341797e-6 * t14043;
    let t14603 = 0.87596530464506835935e-6 * t14049;
    let t14607 = 0.19709219354514038085e-5 * t14054;
    let t14608 = 0.87596530464506835935e-6 * t14057;
    (t14591, t14592, t14593, t14594, t14595, t14596, t14597, t14598, t14599, t14600, t14601, t14602, t14603, t14607, t14608)
}
