//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 681/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk681<F: Float>(t678: F, t9826: F, t1737: F, t649: F, t27: F, t7273: F, t1763: F, t7263: F, t2368: F, t623: F, t1704: F, t665: F) -> (F, F, F, F, F, F, F) {
    let t9827 = t9826 * t678;
    let t9828 = F::cast_from(0.42564599893297839398e-5_f64) * t9827;
    let t9830 = t649 * t1737;
    let t9831 = t27 * t9830;
    let t9832 = t7273 * t9831;
    let t9833 = F::cast_from(0.6818665413561335432e-1_f64) * t9832;
    let t9834 = t649 * t1763;
    let t9835 = t27 * t9834;
    let t9836 = t7263 * t9835;
    let t9837 = F::cast_from(0.68186654135613354322e-2_f64) * t9836;
    let t9838 = t623 * t2368;
    let t9839 = F::cast_from(0.39914139006212695214e-1_f64) * t9838;
    let t9840 = t665 * t1704;
    (t9828, t9831, t9833, t9835, t9837, t9839, t9840)
}
