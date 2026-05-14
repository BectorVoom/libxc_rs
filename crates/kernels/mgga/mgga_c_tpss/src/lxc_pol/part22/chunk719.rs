//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 719/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk719<F: Float>(t3907: F, t3908: F, t912: F, t140: F, t1460: F, t925: F, t2697: F, t926: F, t3749: F, t928: F, t3754: F, t3431: F, t929: F, t241: F, t360: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3909 = t3907 * t3908;
    let t3911 = 0.17315859105681463759e2 * t912 * t3909;
    let t3916 = t140 * t1460;
    let t3917 = t925 * t3916;
    let t3919 = t926 * t2697;
    let t3920 = t3919 * t3749;
    let t3923 = t926 * t928;
    let t3924 = t3923 * t3754;
    let t3927 = t929 * t3431;
    let t3928 = t926 * t3927;
    let t3931 = t241 * t360;
    (t3909, t3911, t3916, t3917, t3919, t3920, t3923, t3924, t3927, t3928, t3931)
}
