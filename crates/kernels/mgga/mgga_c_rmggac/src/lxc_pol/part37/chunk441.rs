//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 441/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk441<F: Float>(t13862: F, t354: F, t3133: F, t1993: F, t3140: F, t1995: F, t305: F, t1986: F, t2002: F, t2001: F, t3141: F, t322: F, t793: F, t334: F, t797: F, t353: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13863 = t13862 * t354;
    let t13864 = t3133 * t13863;
    let t13866 = t1993 * t3140;
    let t13867 = t305 * t1995;
    let t13868 = t1986 * t13867;
    let t13869 = t13866 * t13868;
    let t13871 = t305 * t2002;
    let t13872 = t2001 * t13871;
    let t13873 = t3141 * t13872;
    let t13875 = t793 * t322;
    let t13876 = t1986 * t13875;
    let t13877 = t3141 * t13876;
    let t13879 = t797 * t334;
    let t13880 = t1986 * t13879;
    let t13881 = t3141 * t13880;
    let t13883 = t305 * t353;
    (t13863, t13864, t13866, t13868, t13869, t13872, t13873, t13876, t13877, t13880, t13881, t13883)
}
