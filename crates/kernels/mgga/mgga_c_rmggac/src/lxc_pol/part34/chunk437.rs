//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 437/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk437<F: Float>(t291: F, t7755: F, t13823: F, t2123: F, t649: F, t27: F, t2145: F, t3076: F, t321: F, t2044: F, t12200: F, t333: F, t7273: F, t235: F, t7190: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13824 = t7755 * t291;
    let t13825 = t13823 * t13824;
    let t13827 = t649 * t2123;
    let t13828 = t27 * t13827;
    let t13829 = t2145 * t13828;
    let t13831 = t3076 * t321;
    let t13832 = t2044 * t13831;
    let t13833 = t12200 * t13832;
    let t13835 = t3076 * t333;
    let t13836 = t2044 * t13835;
    let t13837 = t7273 * t13836;
    let t13839 = t235 * t7190;
    (t13824, t13825, t13828, t13829, t13832, t13833, t13836, t13837, t13839)
}
